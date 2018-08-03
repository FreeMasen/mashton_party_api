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

-- DROP TABLE public.guest;

CREATE SEQUENCE public.guest_id_sequence;
ALTER SEQUENCE public.guest_id_sequence OWNER to postgres;

CREATE TABLE public.guest
(
    id integer NOT NULL DEFAULT nextval('guest_id_sequence'::regclass),
    name character varying(1000) COLLATE pg_catalog."default" NOT NULL,
    token uuid NOT NULL DEFAULT uuid_generate_v1(),
    email character varying(500) COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT invited_pkey PRIMARY KEY (id)
)
WITH (
    OIDS = FALSE
)
TABLESPACE pg_default;

ALTER TABLE public.guest
    OWNER to postgres;

GRANT ALL ON TABLE public.guest TO miley;

GRANT ALL ON TABLE public.guest TO postgres;

CREATE SEQUENCE public.invite_id_sequence;
ALTER SEQUENCE public.invite_id_sequence OWNER TO postgres;

CREATE TABLE public.invite
(
    guest_id integer NOT NULL,
    id integer NOT NULL DEFAULT nextval('invite_id_sequence'::regclass),
    guid uuid NOT NULL DEFAULT uuid_generate_v1(),
    party_id integer NOT NULL,
    responded boolean NOT NULL DEFAULT false,
    attending boolean NOT NULL DEFAULT false,
    bringing character varying(2000),
    message character varying(2000),
    CONSTRAINT invite_pkey PRIMARY KEY (id),
    CONSTRAINT invite_party_id_fkey FOREIGN KEY (party_id)
        REFERENCES public.party (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT invite_guest_id_fkey FOREIGN KEY (guest_id)
        REFERENCES public.guest (id) MATCH SIMPLE
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



CREATE OR REPLACE VIEW public.rsvp AS
 SELECT
    i.id,
    g.name,
    i.attending,
    i.bringing,
    i.message,
    i.party_id,
    g.token,
    g.id as guest_id
   FROM guest g
     LEFT JOIN invite i ON i.guest_id = g.id;

ALTER TABLE public.rsvp
    OWNER TO postgres;

GRANT ALL ON TABLE public.rsvp TO miley;

GRANT ALL ON TABLE public.rsvp TO postgres;

CREATE OR REPLACE VIEW public.guest_invite AS
    SELECT g.id, g.name, g.token, g.email, i.guid as invite_token
    FROM guest g
        LEFT JOIN invite i
        ON g.id = i.guest_id;

ALTER TABLE public.user_invite
    OWNER TO postgres;
GRANT ALL on TABLE public.guest_invite TO miley;
GRANT ALL on TABLE public.guest_invite TO postgres;