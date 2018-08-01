-- Table: public.party
CREATE SEQUENCE public.party_id_sequence;
ALTER SEQUENCE public.party_id_sequence OWNER TO postgres;

CREATE TABLE public.party
(
    id integer NOT NULL DEFAULT nextval('party_id_sequence'::regclass),
    name character varying(500) COLLATE pg_catalog."default" NOT NULL,
    date timestamp with time zone NOT NULL,
    snippet character varying(1000) COLLATE pg_catalog."default" NOT NULL,
    description character varying(10000) COLLATE pg_catalog."default" NOT NULL,
    image_path character varying(2000) COLLATE pg_catalog."default" NOT NULL,
    rsvp_item character varying(1000) COLLATE pg_catalog."default",
    CONSTRAINT party_pkey PRIMARY KEY (id, name),
    CONSTRAINT party_id_key UNIQUE (id),
    CONSTRAINT party_name_key UNIQUE (name)
)
WITH (
    OIDS = FALSE
)
TABLESPACE pg_default;

ALTER TABLE public.party
    OWNER to postgres;

GRANT ALL ON TABLE public.party TO miley;

GRANT ALL ON TABLE public.party TO postgres;
-- Table: public."user"

-- DROP TABLE public."user";

CREATE SEQUENCE public.user_id_sequence;
ALTER SEQUENCE public.user_id_sequence OWNER to postgres;

CREATE TABLE public."user"
(
    id integer NOT NULL DEFAULT nextval('user_id_sequence'::regclass),
    name character varying(1000) COLLATE pg_catalog."default" NOT NULL,
    token uuid NOT NULL DEFAULT uuid_generate_v1(),
    email character varying(500) COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT invited_pkey PRIMARY KEY (id)
)
WITH (
    OIDS = FALSE
)
TABLESPACE pg_default;

ALTER TABLE public."user"
    OWNER to postgres;

GRANT ALL ON TABLE public."user" TO miley;

GRANT ALL ON TABLE public."user" TO postgres;

CREATE SEQUENCE public.invite_id_sequence;
ALTER SEQUENCE public.invite_id_sequence OWNER TO postgres;

CREATE TABLE public.invite
(
    user_id integer NOT NULL,
    id integer NOT NULL DEFAULT nextval('invite_id_sequence'::regclass),
    guid uuid NOT NULL DEFAULT uuid_generate_v1(),
    party_id integer NOT NULL,
    CONSTRAINT invite_pkey PRIMARY KEY (id),
    CONSTRAINT invite_party_id_fkey FOREIGN KEY (party_id)
        REFERENCES public.party (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT invite_user_id_fkey FOREIGN KEY (user_id)
        REFERENCES public."user" (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
)
WITH (
    OIDS = FALSE
)
TABLESPACE pg_default;

ALTER TABLE public.invite
    OWNER to postgres;

GRANT ALL ON TABLE public.invite TO miley;

GRANT ALL ON TABLE public.invite TO postgres;

CREATE SEQUENCE public.place_id_sequence;
ALTER SEQUENCE public.place_id_sequence OWNER TO postgres;

CREATE TABLE public.place
(
    id integer NOT NULL DEFAULT nextval('place_id_sequence'::regclass),
    address character varying(1000) COLLATE pg_catalog."default",
    city character varying(1000) COLLATE pg_catalog."default",
    state character varying(10) COLLATE pg_catalog."default",
    zip character varying(5) COLLATE pg_catalog."default",
    description character varying(2000) COLLATE pg_catalog."default",
    party_id integer NOT NULL,
    name character varying(1000) COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT place_pkey PRIMARY KEY (id),
    CONSTRAINT place_party_id_fkey FOREIGN KEY (party_id)
        REFERENCES public.party (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
)
WITH (
    OIDS = FALSE
)
TABLESPACE pg_default;

ALTER TABLE public.place
    OWNER to miley;

GRANT ALL ON TABLE public.place TO miley;

-- Table: public.rsvp

-- DROP TABLE public.rsvp;
CREATE SEQUENCE public.rsvp_id_sequence;
ALTER SEQUENCE public.rsvp_id_sequence OWNER TO postgres;
CREATE TABLE public.rsvp
(
    id integer NOT NULL DEFAULT nextval('rsvp_id_sequence'::regclass),
    name character varying(1000) COLLATE pg_catalog."default" NOT NULL,
    bringing character varying(1000) COLLATE pg_catalog."default" DEFAULT 0,
    message character varying(5000) COLLATE pg_catalog."default",
    party_id integer,
    user_id integer NOT NULL,
    attending boolean NOT NULL DEFAULT false,
    CONSTRAINT rsvp_pkey PRIMARY KEY (id),
    CONSTRAINT rsvp_party_id_fkey FOREIGN KEY (party_id)
        REFERENCES public.party (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT rsvp_user_id_fkey FOREIGN KEY (user_id)
        REFERENCES public."user" (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
)
WITH (
    OIDS = FALSE
)
TABLESPACE pg_default;

ALTER TABLE public.rsvp
    OWNER to postgres;

GRANT ALL ON TABLE public.rsvp TO miley;

GRANT ALL ON TABLE public.rsvp TO postgres;

-- View: public.user_rsvps

-- DROP VIEW public.user_rsvps;

CREATE OR REPLACE VIEW public.user_rsvps AS
 SELECT u.token,
    r.id
   FROM "user" u
     LEFT JOIN rsvp r ON r.user_id = u.id;

ALTER TABLE public.user_rsvps
    OWNER TO postgres;

GRANT ALL ON TABLE public.user_rsvps TO miley;

GRANT ALL ON TABLE public.user_rsvps TO postgres;