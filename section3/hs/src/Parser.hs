module Parser where

import Lexer
import Syntax
import Text.Parsec
import qualified Text.Parsec.Expr as Ex
import Text.Parsec.String (Parser)
import qualified Text.Parsec.Token as Tok

binary s f assoc = Ex.Infix (reservedOp s >> return (BinOp f)) assoc

table =
  [ [binary "*" Times Ex.AssocLeft],
    [ binary "+" Plus Ex.AssocLeft,
      binary "-" Minus Ex.AssocLeft
    ]
  ]

int :: Parser Expr
int = do
  n <- integer
  return $ Integer n

expr :: Parser Expr
expr = Ex.buildExpressionParser table factor

argument :: Parser Argument
argument = do
  varType <- identifier
  varName <- identifier
  return $ Argument varType varName

statement :: Parser Statement
statement = do
  stmt <- expr
  reserved ";"
  return $ Expr stmt

function :: Parser Function
function = do
  retType <- identifier
  name <- identifier
  args <- parens $ commaSep argument
  body <- braces $ many statement
  return $ Function retType name args body

call :: Parser Expr
call = do
  name <- identifier
  args <- parens $ many expr
  return $ Call name args

factor :: Parser Expr
factor =
  try int
    <|> try call
    <|> parens expr

contents :: Parser a -> Parser a
contents p = do
  Tok.whiteSpace lexer
  r <- p
  eof
  return r

toplevel :: Parser Program
toplevel = do
  fnList <- many $ try function
  return $ Program fnList

parseToplevel :: String -> Either ParseError Program
parseToplevel s = parse (contents toplevel) "<stdin>" s

compile :: String -> String
compile s = case parseToplevel s of
  Left err -> show err
  Right ex -> show ex
