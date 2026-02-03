""" バリデーションのインポート """
from pydantic import BaseModel

class Config:
    orm_mode = True

class TodoBase(BaseModel):
    title: str


class TodoCreate(TodoBase):
    """POST /todos 用"""
    pass


class TodoUpdate(BaseModel):
    completed: bool


class TodoResponse(TodoBase):
    id: int
    completed: bool
