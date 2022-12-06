import Parser
import Test.Hspec

test :: String -> String -> SpecWith (Arg Expectation)
test input expected = it input $ do
  let res = compile input
  shouldBe expected res

main :: IO ()
main = hspec $ do
  describe "parser" $ do
    test "int main() { 1 + 2; }" "Program [Function \"int\" \"main\" [] [Expr (BinOp Plus (Integer 1) (Integer 2))]]"
    test "int main() { 1 + 2; 3 + 4; }" "Program [Function \"int\" \"main\" [] [Expr (BinOp Plus (Integer 1) (Integer 2)),Expr (BinOp Plus (Integer 3) (Integer 4))]]"
    test "int main() { return 42; }" "Program [Function \"int\" \"main\" [] [Return (Integer 42)]]"
