CREATE TABLE courses (
                         id UUID PRIMARY KEY,
                         blockchain_id VARCHAR NOT NULL,
                         instructor_address VARCHAR NOT NULL,
                         title VARCHAR NOT NULL,
                         metadata TEXT NOT NULL,
                         price BIGINT NOT NULL,
                         is_active BOOLEAN NOT NULL DEFAULT true,
                         created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE certificates (
                              id UUID PRIMARY KEY,
                              student_address VARCHAR NOT NULL,
                              course_id UUID NOT NULL REFERENCES courses(id),
                              blockchain_hash VARCHAR NOT NULL,
                              issued_at TIMESTAMP WITH TIME ZONE NOT NULL,
                              created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                              UNIQUE(student_address, course_id)
);