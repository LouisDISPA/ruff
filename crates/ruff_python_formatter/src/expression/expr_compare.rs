use crate::comments::leading_comments;
use crate::expression::parentheses::{
    default_expression_needs_parentheses, in_parentheses_only_group, NeedsParentheses, Parentheses,
    Parenthesize,
};
use crate::prelude::*;
use crate::FormatNodeRule;
use ruff_formatter::{write, FormatOwnedWithRule, FormatRefWithRule, FormatRuleWithOptions};
use rustpython_parser::ast::{CmpOp, ExprCompare};

#[derive(Default)]
pub struct FormatExprCompare {
    parentheses: Option<Parentheses>,
}

impl FormatRuleWithOptions<ExprCompare, PyFormatContext<'_>> for FormatExprCompare {
    type Options = Option<Parentheses>;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.parentheses = options;
        self
    }
}

impl FormatNodeRule<ExprCompare> for FormatExprCompare {
    fn fmt_fields(&self, item: &ExprCompare, f: &mut PyFormatter) -> FormatResult<()> {
        let ExprCompare {
            range: _,
            left,
            ops,
            comparators,
        } = item;

        let comments = f.context().comments().clone();

        write!(f, [in_parentheses_only_group(&left.format())])?;

        assert_eq!(comparators.len(), ops.len());

        for (operator, comparator) in ops.iter().zip(comparators) {
            let leading_comparator_comments = comments.leading_comments(comparator);
            if leading_comparator_comments.is_empty() {
                write!(f, [soft_line_break_or_space()])?;
            } else {
                // Format the expressions leading comments **before** the operator
                write!(
                    f,
                    [
                        hard_line_break(),
                        leading_comments(leading_comparator_comments)
                    ]
                )?;
            }

            write!(
                f,
                [
                    operator.format(),
                    space(),
                    in_parentheses_only_group(&comparator.format())
                ]
            )?;
        }

        Ok(())
    }
}

impl NeedsParentheses for ExprCompare {
    fn needs_parentheses(
        &self,
        parenthesize: Parenthesize,
        context: &PyFormatContext,
    ) -> Parentheses {
        default_expression_needs_parentheses(self.into(), parenthesize, context)
    }
}

#[derive(Copy, Clone)]
pub struct FormatCmpOp;

impl<'ast> AsFormat<PyFormatContext<'ast>> for CmpOp {
    type Format<'a> = FormatRefWithRule<'a, CmpOp, FormatCmpOp, PyFormatContext<'ast>>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatCmpOp)
    }
}

impl<'ast> IntoFormat<PyFormatContext<'ast>> for CmpOp {
    type Format = FormatOwnedWithRule<CmpOp, FormatCmpOp, PyFormatContext<'ast>>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatCmpOp)
    }
}

impl FormatRule<CmpOp, PyFormatContext<'_>> for FormatCmpOp {
    fn fmt(&self, item: &CmpOp, f: &mut Formatter<PyFormatContext<'_>>) -> FormatResult<()> {
        let operator = match item {
            CmpOp::Eq => "==",
            CmpOp::NotEq => "!=",
            CmpOp::Lt => "<",
            CmpOp::LtE => "<=",
            CmpOp::Gt => ">",
            CmpOp::GtE => ">=",
            CmpOp::Is => "is",
            CmpOp::IsNot => "is not",
            CmpOp::In => "in",
            CmpOp::NotIn => "not in",
        };

        text(operator).fmt(f)
    }
}
