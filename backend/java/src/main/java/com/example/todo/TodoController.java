package com.example.todo;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.List;
import java.util.Map;

@RestController
public class TodoController {

    @GetMapping("/todos")
    public List<Map<String, Object>> getTodos() {
        return List.of(
            Map.of(
                "id", 1,
                "title", "JavaでTODO作成",
                "completed", false
            )
        );
    }
}
