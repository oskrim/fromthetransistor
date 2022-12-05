module Syntax where

type Name = String

data Program
  = Program [Function]
  deriving (Eq, Ord, Show)

data Argument
  = Argument Name Name
  deriving (Eq, Ord, Show)

data Function
  = Function Name Name [Argument] [Statement]
  deriving (Eq, Ord, Show)

data Statement
  = Expr Expr
  deriving (Eq, Ord, Show)

data Expr
  = Integer Integer
  | BinOp Op Expr Expr
  | Var String
  | Call Name [Expr]
  deriving (Eq, Ord, Show)

data Op
  = Plus
  | Minus
  | Times
  deriving (Eq, Ord, Show)
