from fastapi import FastAPI, Depends, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from sqlalchemy.orm import Session

from .database import SessionLocal, engine
from . import models, schemas, crud

# テーブル作成
models.Base.metadata.create_all(bind=engine)

# インスタンス化
app = FastAPI(title="TODO API")

# CORS設定：あまり理解できていないので後回し
# app.add_middleware(
#     CORSMiddleware,
#     allow_origins=["*"],
#     allow_methods=["GET", "POST", "PUT", "DELETE"],
#     allow_headers=["*"],
# )


# リクエストごとにDBセッション作成
def get_db():
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()

# TODO一覧取得処理
@app.get("/todos", response_model=list[schemas.TodoResponse])
def get_todos(db: Session = Depends(get_db)):
    return crud.get_todos(db)


# TODO新規作成処理
@app.post("/todos", response_model=schemas.TodoResponse)
def create_todo(
    req: schemas.TodoCreate,
    db: Session = Depends(get_db)
):
    return crud.create_todo(db, req.title)


# TODO更新処理
@app.put("/todos/{todo_id}", response_model=schemas.TodoResponse)
def update_todo(
    todo_id: int,
    req: schemas.TodoUpdate,
    db: Session = Depends(get_db)
):
    todo = crud.update_todo(db, todo_id, req.completed)

    if todo is None:
        raise HTTPException(status_code=404, detail="Todo not found")

    return todo


# TODO削除処理
@app.delete("/todos/{todo_id}")
def delete_todo(
    todo_id: int,
    db: Session = Depends(get_db)
):
    success = crud.delete_todo(db, todo_id)

    if not success:
        raise HTTPException(status_code=404, detail="Todo not found")

    return {"message": "deleted"}
