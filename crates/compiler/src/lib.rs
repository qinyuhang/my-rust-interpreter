mod compilation_scope;
mod emitted_instruction;
mod symbol_table;
mod test;

use ::ast::*;
pub use compilation_scope::*;
pub use emitted_instruction::*;
// use ::parser::*;
pub use crate::symbol_table::*;
use ::object::*;
#[allow(unused)]
use byteorder::{BigEndian, ByteOrder};
use code::OpCode::OpPop;
use code::{self, *};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use token::*;

#[derive(Debug)]
pub struct Compiler<'a> {
    constants: RefCell<Vec<Rc<dyn Object>>>,
    external_constants: RefCell<Option<&'a mut Vec<Rc<dyn Object>>>>,

    symbol_table: RefCell<Rc<SymbolTable>>,
    external_symbol_table: RefCell<Option<&'a mut SymbolTable>>,

    scopes: RefCell<Vec<Rc<CompilationScope>>>,
    scope_index: Cell<usize>,
}

#[derive(Debug)]
pub struct ByteCode {
    pub instructions: Rc<Instructions>,
    pub constants: RefCell<Vec<Rc<dyn Object>>>,
}

thread_local! {
    static EMPTY_V16: Vec<u16> = vec![];
}

impl<'a> Compiler<'a> {
    pub fn new() -> Self {
        let main_scope = Rc::new(CompilationScope::new());

        let mut symbol_table = SymbolTable::new();
        Self::define_builtin_to(&mut symbol_table);

        Self {
            constants: RefCell::new(vec![]),
            external_constants: RefCell::new(None),

            symbol_table: RefCell::new(Rc::new(symbol_table)),
            external_symbol_table: RefCell::new(None),
            scopes: RefCell::new(vec![main_scope]),
            scope_index: Cell::new(0),
        }
    }

    fn define_builtin_to(symbol_table: &mut SymbolTable) {
        BUILTINS.with(|b| {
            b.iter().enumerate().for_each(|(idx, (name, _))| {
                symbol_table.define_builtin(
                    idx,
                    Rc::new(Identifier {
                        token: Rc::new(Token {
                            literal: Rc::new(name.to_string()),
                            token_type: IDENT,
                        }),
                        value: Rc::new(name.to_string()),
                    }),
                );
            });
        });
    }

    pub fn create_constants() -> Vec<Rc<dyn Object>> {
        vec![]
    }
    pub fn create_symbol_table() -> SymbolTable {
        SymbolTable::new()
    }

    /// IF want call this, must call before compile
    pub fn load_external_constants(
        &self,
        external_constants: &'a mut Vec<Rc<dyn Object>>,
    ) -> Result<(), String> {
        if self.constants.borrow().len() != 0 {
            return Err(format!(
                "call load_external_constants before compiler.compile()"
            ));
        }
        *self.external_constants.borrow_mut() = Some(external_constants);
        Ok(())
    }

    /// IF want call this, must call before compile
    pub fn load_external_symbol_table(
        &self,
        external_symbol_table: &'a mut SymbolTable,
    ) -> Result<(), String> {
        if self.symbol_table.borrow().outer.borrow().is_some() {
            return Err(format!(
                "call load_external_symbol_table before compiler.compile()"
            ));
        }
        Self::define_builtin_to(external_symbol_table);
        *self.external_symbol_table.borrow_mut() = Some(external_symbol_table);
        Ok(())
    }

    pub fn compile(&self, node: &dyn Node) -> Result<(), String> {
        let n = node.as_any();
        if n.is::<Program>() {
            let n = n.downcast_ref::<Program>().unwrap();
            for st in n.statement.iter() {
                self.compile(st.get_expression().upcast())?;
            }
        }
        if n.is::<ExpressionStatement>() {
            if let Some(ExpressionStatement {
                expression: Some(bbq),
                ..
            }) = n.downcast_ref::<ExpressionStatement>()
            {
                let x = bbq.get_expression();
                let r = self.compile(x.upcast());
                EMPTY_V16.with(|v| self.emit(OpCode::OpPop, v));
                return r;
            }
        }
        if n.is::<InfixExpression>() {
            if let Some(InfixExpression {
                left: Some(left),
                right: Some(right),
                operator,
                ..
            }) = n.downcast_ref::<InfixExpression>()
            {
                let left = left.get_expression();
                let right = right.get_expression();
                // flip `x < y` to `y > x`
                if operator.as_str() == "<" {
                    self.compile(right.upcast())?;
                    self.compile(left.upcast())?;
                    EMPTY_V16.with(|v| {
                        self.emit(OpCode::OpGreaterThan, v);
                    });
                    return Ok(());
                }
                self.compile(left.upcast())?;
                self.compile(right.upcast())?;
                match operator.as_str() {
                    "+" => EMPTY_V16.with(|v| {
                        self.emit(OpCode::OpAdd, v);
                    }),
                    "-" => EMPTY_V16.with(|v| {
                        self.emit(OpCode::OpSub, v);
                    }),
                    "*" => EMPTY_V16.with(|v| {
                        self.emit(OpCode::OpMul, v);
                    }),
                    "/" => EMPTY_V16.with(|v| {
                        self.emit(OpCode::OpDiv, v);
                    }),
                    ">" => EMPTY_V16.with(|v| {
                        self.emit(OpCode::OpGreaterThan, v);
                    }),
                    "==" => EMPTY_V16.with(|v| {
                        self.emit(OpCode::OpEqual, v);
                    }),
                    "!=" => EMPTY_V16.with(|v| {
                        self.emit(OpCode::OpNotEqual, v);
                    }),
                    _ => return Err(format!("unknown operator: {}", operator)),
                }
            }
        }
        if n.is::<PrefixExpression>() {
            if let Some(PrefixExpression {
                right: Some(right),
                operator,
                ..
            }) = n.downcast_ref::<PrefixExpression>()
            {
                self.compile(right.upcast())?;
                match operator.as_str() {
                    "!" => EMPTY_V16.with(|v| {
                        self.emit(OpCode::OpBang, v);
                    }),
                    "-" => EMPTY_V16.with(|v| {
                        self.emit(OpCode::OpMinus, v);
                    }),
                    _ => return Err(format!("unknown operator {}", operator)),
                }
            };
        }
        if n.is::<IntegerLiteral>() {
            let i = n.downcast_ref::<IntegerLiteral>().unwrap();
            let i = Integer { value: i.value };
            self.emit(
                code::OpCode::OpConstant,
                &vec![self.add_constant(Rc::new(i)) as u16],
            );
        }
        if n.is::<BooleanLiteral>() {
            let i = n.downcast_ref::<BooleanLiteral>().unwrap();
            match i.value {
                true => EMPTY_V16.with(|v| {
                    self.emit(OpCode::OpTrue, v);
                }),
                false => EMPTY_V16.with(|v| {
                    self.emit(OpCode::OpFalse, v);
                }),
            };
        }
        if n.is::<IfExpression>() {
            // with alternative:
            //     condition
            //     jnt───────────────────┐
            //     consequence           │
            // ┌───jmp                   │
            // │   null / alternative◄───┘
            // └──►next
            let i = n.downcast_ref::<IfExpression>().unwrap();
            self.compile(i.condition.get_expression().upcast())?;

            // fake offset
            let jnt_position = self.emit(OpCode::OpJNT, &vec![9999]);

            if let Some(consequence) = &i.consequence {
                self.compile(consequence.get_expression().upcast())?;
            }
            if self.last_instruction_is(OpPop) {
                self.remove_last_pop();
            }

            let jmp_position = self.emit(OpCode::OpJMP, &vec![9999]);

            let after_consequence = self.current_instructions().borrow().len();
            self.change_operand(jnt_position, after_consequence)?;

            if let Some(alternative) = &i.alternative {
                self.compile(alternative.get_expression().upcast())?;
                if self.last_instruction_is(OpPop) {
                    self.remove_last_pop();
                }
            } else {
                EMPTY_V16.with(|v| self.emit(OpCode::OpNull, v));
            }
            let after_alternative = self.current_instructions().borrow().len();
            self.change_operand(jmp_position, after_alternative)?;

            // dbg!(after_consequence);
        }
        if n.is::<BlockStatement>() {
            let i = n.downcast_ref::<BlockStatement>().unwrap();
            for val in &i.statement {
                self.compile(val.get_expression().upcast())?;
            }
        }
        if n.is::<LetStatement>() {
            let i = n.downcast_ref::<LetStatement>().unwrap();
            self.compile(i.value.as_ref().unwrap().get_expression().upcast())?;
            // 这里和书不一样
            if self.last_instruction_is(OpPop) {
                self.remove_last_pop();
            }
            let symbol = self.define_symbol(i.name.clone());
            let op = match symbol.scope {
                GLOBAL_SCOPE => OpCode::OpSetGlobal,
                LOCAL_SCOPE => OpCode::OpSetLocal,
                _ => return Err("unsupported SCOPE".to_string()),
            };
            self.emit(op, &vec![symbol.index as u16]);
        }
        if n.is::<Identifier>() {
            let i = n.downcast_ref::<Identifier>().unwrap();
            // FIXME: DRAW BACK CLONE
            let symbol = self.resolve_symbol(Rc::new(i.clone()))?;
            let op = match symbol.scope {
                GLOBAL_SCOPE => OpCode::OpGetGlobal,
                LOCAL_SCOPE => OpCode::OpGetLocal,
                BUILTIN_SCOPE => OpCode::OpGetBuiltin,
                _ => return Err("unsupported SCOPE".to_string()),
            };
            self.emit(op, &vec![symbol.index as u16]);
        }
        if n.is::<StringLiteral>() {
            let i = n.downcast_ref::<StringLiteral>().unwrap();
            let obj = Rc::new(StringObject {
                value: i.value.clone(),
            });
            self.emit(OpCode::OpConstant, &vec![self.add_constant(obj) as u16]);
        }
        if n.is::<ArrayLiteral>() {
            let i = n.downcast_ref::<ArrayLiteral>().unwrap();
            for val in i.elements.iter() {
                self.compile(val.upcast())?;
            }
            self.emit(OpCode::OpArray, &vec![i.elements.len() as u16]);
        }
        if n.is::<HashLiteral>() {
            let i = n.downcast_ref::<HashLiteral>().unwrap();
            let p = i.pairs.borrow();
            let mut expressions = p.iter().map(|(&ref k, &ref v)| (k, v)).collect::<Vec<_>>();
            expressions.sort_by(|&ref a, &ref b| {
                a.0.get_expression()
                    .to_string()
                    .cmp(&b.0.get_expression().to_string())
            });
            for (k, v) in &expressions {
                self.compile(k.upcast())?;
                self.compile(v.upcast())?;
            }
            self.emit(OpCode::OpHash, &vec![2 * expressions.len() as u16]);
        }
        if n.is::<IndexExpression>() {
            let i = n.downcast_ref::<IndexExpression>().unwrap();
            self.compile(i.left.upcast())?;
            self.compile(i.index.upcast())?;
            EMPTY_V16.with(|v| self.emit(OpCode::OpIndex, v));
        }
        if n.is::<FunctionLiteral>() {
            let i = n.downcast_ref::<FunctionLiteral>().unwrap();
            self.enter_scope();
            if let Some(params) = i.parameters.as_ref() {
                for p in params.iter() {
                    self.define_symbol(p.clone());
                }
            }
            if let Some(body) = i.body.clone() {
                self.compile(body.upcast())?;
            }
            if self.last_instruction_is(OpCode::OpPop) {
                EMPTY_V16.with(|v| self.replace_last_instruction(OpCode::OpReturnValue, v));
            }
            if !self.last_instruction_is(OpCode::OpReturnValue) {
                EMPTY_V16.with(|v| self.emit(OpCode::OpReturn, v));
            }
            let num_locals = self.symbol_table.borrow().define_count();
            let num_parameters = i.parameters.as_ref().map_or(0, |v| v.len());
            let ins = self.leave_scope();
            let compiled_fn = CompiledFunction {
                instructions: Rc::new(ins.take()),
                num_locals,
                num_parameters,
            };
            self.emit(
                OpCode::OpConstant,
                &vec![self.add_constant(Rc::new(compiled_fn)) as u16],
            );
        }
        if n.is::<ReturnStatement>() {
            let i = n.downcast_ref::<ReturnStatement>().unwrap();
            if let Some(r) = i.return_value.clone() {
                self.compile(r.upcast())?;
                EMPTY_V16.with(|v| self.emit(OpCode::OpReturnValue, v));
            }
            // FIXME: OpReturn
        }
        if n.is::<CallExpression>() {
            let i = n.downcast_ref::<CallExpression>().unwrap();
            if let Some(r) = i.function.clone() {
                self.compile(r.upcast())?;
            }
            if let Some(args) = i.arguments.as_ref() {
                for arg in args.iter() {
                    self.compile(arg.upcast())?;
                }
            };
            self.emit(
                OpCode::OpCall,
                &vec![i.arguments.as_ref().map_or(0, |v| v.len() as u16)],
            );
        }
        // match _node.ty {  }
        Ok(())
    }

    fn emit(&self, op: OpCode, operands: &Vec<u16>) -> usize {
        let ins = make(&op, &operands[..]);
        let pos = self.add_instruction(&ins);
        self.set_last_instruction(op, pos);
        pos
    }

    fn set_last_instruction(&self, op_code: OpCode, pos: usize) {
        let scopes = self.scopes.borrow();
        let previous = scopes
            .get(self.scope_index.get())
            .unwrap()
            .last_instruction
            .take();
        let last = EmittedInstruction {
            op_code,
            position: pos,
        };

        scopes
            .get(self.scope_index.get())
            .unwrap()
            .previous_instruction
            .replace(previous);
        scopes
            .get(self.scope_index.get())
            .unwrap()
            .last_instruction
            .replace(last);
    }

    fn add_instruction(&self, ins: &[u8]) -> usize {
        let pos_new_instruction = self.current_instructions().borrow().len();
        // assert_eq!(self.current_instructions());
        self.current_instructions()
            .borrow_mut()
            .extend_from_slice(ins);
        pos_new_instruction
    }

    fn add_constant(&self, obj: Rc<dyn Object>) -> usize {
        let has_external = self.external_constants.borrow().is_some();
        if has_external {
            self.external_constants
                .borrow_mut()
                .as_mut()
                .unwrap()
                .push(obj);
            self.external_constants.borrow().as_ref().unwrap().len() - 1
        } else {
            self.constants.borrow_mut().push(obj);
            self.constants.borrow().len() - 1
        }
    }

    fn define_symbol(&self, i: Rc<Identifier>) -> Rc<Symbol> {
        let has_external = self.external_symbol_table.borrow().is_some();
        if has_external {
            self.external_symbol_table
                .borrow()
                .as_ref()
                .unwrap()
                .define(i)
        } else {
            self.symbol_table.borrow().define(i)
        }
    }
    fn resolve_symbol(&self, i: Rc<Identifier>) -> Result<Rc<Symbol>, String> {
        let has_external = self.external_symbol_table.borrow().is_some();
        if has_external {
            self.external_symbol_table
                .borrow()
                .as_ref()
                .unwrap()
                .resolve(i)
        } else {
            self.symbol_table.borrow().resolve(i)
        }
    }

    fn last_instruction_is(&self, op_code: OpCode) -> bool {
        self.scopes
            .borrow()
            .get(self.scope_index.get())
            .unwrap()
            .last_instruction
            .get()
            .op_code
            == op_code
    }

    fn remove_last_pop(&self) {
        let last_instruction_position = self
            .scopes
            .borrow()
            .get(self.scope_index.get())
            .unwrap()
            .last_instruction
            .get()
            .position;

        self.current_instructions()
            .borrow_mut()
            .truncate(last_instruction_position);

        let previous_instruction = self
            .scopes
            .borrow()
            .get(self.scope_index.get())
            .unwrap()
            .previous_instruction
            .get();
        self.scopes
            .borrow()
            .get(self.scope_index.get())
            .unwrap()
            .last_instruction
            .replace(previous_instruction);
    }

    fn replace_instruction(&self, pos: usize, n: &[u8]) {
        let mut i = 0;
        while i < n.len() {
            self.current_instructions().borrow_mut()[pos + i] = n[i];
            i += 1;
        }
    }

    fn replace_last_instruction(&self, op: OpCode, operands: &Vec<u16>) -> usize {
        let ins = make(&op, operands);
        let pos = self.current_instructions().borrow().len() - 1;
        self.replace_instruction(pos, &ins);
        let mut last = self
            .scopes
            .borrow()
            .get(self.scope_index.get())
            .unwrap()
            .last_instruction
            .get();
        last.op_code = op;
        self.scopes
            .borrow()
            .get(self.scope_index.get())
            .unwrap()
            .last_instruction
            .replace(last);
        pos
    }

    fn change_operand(&self, op_pos: usize, operand: usize) -> Result<(), String> {
        let op = OpCode::from(
            *self
                .current_instructions()
                .borrow_mut()
                .get(op_pos)
                .unwrap(),
        );

        // look up op to find the op_width
        // convert operand to Vec<u16> limited to op_width

        // hope this will guard the safety restriction
        // u16::Max = 65535
        // u32::Max = 4294967295
        // which means the program written in `monkey lang` should
        // be small enough to be compiled to bytecode
        // OR OTHERWISE WE MUST WIDER THE OPERAND OF OpJNT AND OpJMP
        // write usize to u16 array

        // FIXME: operand is usize (consider it u64) it should be convert into u16 instead of `as u16`
        // anyway it now works, later change the op_width of JNT JMP to 4, we need impl the convert
        let new_instruction = make(&op, &vec![operand as u16]);
        self.replace_instruction(op_pos, &new_instruction);
        Ok(())
    }

    fn enter_scope(&self) {
        let scope = CompilationScope {
            instructions: Rc::new(RefCell::new(vec![])),
            last_instruction: Cell::new(EmittedInstruction::default()),
            previous_instruction: Cell::new(EmittedInstruction::default()),
        };
        self.scopes.borrow_mut().push(Rc::new(scope));
        self.scope_index.replace(self.scope_index.get() + 1);
        let pre = self.symbol_table.take();
        let new = Rc::new(SymbolTable::new_enclosed(pre));
        self.symbol_table.replace(new);
    }

    fn leave_scope(&self) -> Rc<RefCell<Instructions>> {
        // let instructions = self.current_instructions();
        // let instructions = {
        //     let scopes = self.scopes.borrow();
        //     let scope = scopes
        //         .get(self.scope_index.get())
        //         .expect("Scope index out of bounds");
        //     scope.instructions.clone()
        // };
        let instructions = self.scopes.borrow_mut().pop().unwrap().instructions.clone();
        self.scope_index.replace(self.scope_index.get() - 1);
        let outer = self.symbol_table.take();
        let outer = outer.outer.take().unwrap();
        self.symbol_table.replace(outer);
        instructions
    }

    fn current_instructions(&self) -> Rc<RefCell<Instructions>> {
        let scopes = self.scopes.borrow();
        scopes
            .get(self.scope_index.get())
            .unwrap()
            .instructions
            .clone()
    }

    pub fn bytecode(&self) -> Rc<ByteCode> {
        let has_external = self.external_constants.borrow().is_some();
        let constants = if has_external {
            RefCell::new(
                self.external_constants
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .iter()
                    .cloned()
                    .collect(),
            )
        } else {
            self.constants.clone()
        };
        Rc::new(ByteCode {
            // FIXME: which one should be use?
            instructions: Rc::new((*self.current_instructions()).borrow().clone()),
            constants,
        })
    }

    pub fn dump_instruction(&self) -> String {
        let instructions = self.current_instructions();
        let instructions = instructions.borrow();
        format_display_instructions(&instructions)
    }
}
