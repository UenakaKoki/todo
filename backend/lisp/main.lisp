;;;; main.lisp

;;;; Quicklisp をロード
(require :asdf)

;; clack-handler-hunchentoot を明示的にロード
(asdf:load-system :clack-handler-hunchentoot)
(asdf:load-system :ningle)

;;;; パッケージ定義
(defpackage :todo-app
  (:use :cl :ningle))
(in-package :todo-app)

(clack:clackup *app*
               :server :hunchentoot
               :port 9000)

;;;; アプリ作成
(defparameter *app* (make-instance 'app))

;;;; データ（超シンプル）
(defparameter *todos* '())

;;;; GET /todos
(setf (route *app* "/todos" :method :GET)
      (lambda (params)
        (declare (ignore params))
        (list 200
              '(:content-type "application/json")
              (list
               (with-output-to-string (s)
                 (write *todos* :stream s))))))

;;;; POST /todos
(setf (route *app* "/todos" :method :POST)
      (lambda (params)
