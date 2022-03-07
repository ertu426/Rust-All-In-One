create table course
(
    id         serial,
    teacher_id integer      not null,
    name       varchar(140) not null,
    time       timestamp default now(),
    constraint course_pk
        primary key (id)
);

create unique index course_id_uindex
    on course (id);