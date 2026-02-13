-- Add migration script here
CREATE TABLE user_departments (
    user_id        BINARY(16) NOT NULL,
    department_id  BIGINT NOT NULL,
    assigned_at    TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (user_id, department_id),

    CONSTRAINT fk_ud_user
        FOREIGN KEY (user_id) REFERENCES users(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_ud_department
        FOREIGN KEY (department_id) REFERENCES departments(id)
        ON DELETE CASCADE
);

CREATE INDEX idx_ud_department ON user_departments(department_id);