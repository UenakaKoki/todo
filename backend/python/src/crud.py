from sqlalchemy.orm import Session
from .models import Todo


def get_todos(db: Session):
    """TODO一覧を取得"""
    return db.query(Todo).all()


def create_todo(db: Session, title: str):
    """TODOを新規作成"""
    todo = Todo(title=title)

    db.add(todo)
    db.commit()
    db.refresh(todo)

    return todo


def update_todo(db: Session, todo_id: int, completed: bool):
    """完了状態を更新"""
    todo = db.query(Todo).filter(Todo.id == todo_id).first()

    if todo is None:
        return None

    todo.completed = completed
    db.commit()
    db.refresh(todo)

    return todo


def delete_todo(db: Session, todo_id: int):
    """TODOを削除"""
    todo = db.query(Todo).filter(Todo.id == todo_id).first()

    if todo is None:
        return False

    db.delete(todo)
    db.commit()

    return True
