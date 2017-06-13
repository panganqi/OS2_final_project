use std::mem;

use context::{Context, Stack};

/// Coroutine is nothing more than a context and a stack
#[derive(Debug)]
pub struct Coroutine {
    context: Context,
    stack: Option<Stack>,
}

impl Coroutine {
    /// Spawn an empty coroutine
    #[inline(always)]
    pub unsafe fn empty() -> Coroutine {
        Coroutine {
            context: Context::empty(),
            stack: None,
        }
    }

    /// New a coroutine with initialized context and stack
    #[inline(always)]
    pub fn new(ctx: Context, stack: Stack) -> Coroutine {
        Coroutine {
            context: ctx,
            stack: Some(stack),
        }
    }

    /// Yield to another coroutine
    #[inline(always)]
    pub fn yield_to(&mut self, target: &Coroutine) {
        Context::swap(&mut self.context, &target.context);
    }

    /// Take out the stack
    #[inline(always)]
    pub fn take_stack(mut self) -> Option<Stack> {
        self.stack.take()
    }

    /// Get context
    #[inline(always)]
    pub fn context(&self) -> &Context {
        &self.context
    }

    /// Get mutable context
    #[inline(always)]
    pub fn context_mut(&mut self) -> &mut Context {
        &mut self.context
    }

    /// Set stack
    #[inline(always)]
    pub fn set_stack(&mut self, stack: Stack) -> Option<Stack> {
        let mut opt_stack = Some(stack);
        mem::swap(&mut self.stack, &mut opt_stack);
        opt_stack
    }
}
