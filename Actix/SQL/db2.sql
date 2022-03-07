create table course
(
    id          serial
        constraint course_pk
            primary key,
    teacher_id  integer      not null,
    name        varchar(140) not null,
    time        timestamp default now(),
    description varchar(2000),
    format      varchar(30),
    structure   varchar(200),
    duration    varchar(30),
    price       integer,
    language    varchar(30),
    level       varchar(30)
);

create unique index course_id_uindex
    on course (id);