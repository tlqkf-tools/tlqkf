#[cfg(test)]
extern crate rspec;

#[test]
pub fn arguments_tests() {
  use super::*;

  #[derive(Clone, Debug)]
  struct Environment {
    args: Arguments,
  }

  let given_arguments = vec![String::from("a"), String::from("b"), String::from("c")];
  let environment: Environment = Environment {
    args: Arguments(given_arguments),
  };

  rspec::run(&rspec::describe("contains", environment, |ctx| {
    ctx.context("when contains has given value", |ctx| {
      const GIVEN_VALUE: &str = "a";

      ctx.it("should returns true", |env| {
        assert_eq!(env.args.contains(GIVEN_VALUE), true);
      });
    });

    ctx.context("when contains hasn't given value", |ctx| {
      const GIVEN_VALUE: &str = "not exists";

      ctx.it("should returns false", |env| {
        assert_eq!(env.args.contains(GIVEN_VALUE), false);
      });
    });
  }));
}
