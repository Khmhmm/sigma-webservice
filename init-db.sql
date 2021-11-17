--
-- PostgreSQL database dump
--

-- Dumped from database version 12.6
-- Dumped by pg_dump version 12.6

-- Started on 2021-11-18 01:20:19

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- TOC entry 240 (class 1255 OID 16661)
-- Name: existsUserByName(text); Type: FUNCTION; Schema: public; Owner: sigma_admin
--

CREATE FUNCTION public."existsUserByName"(_name text) RETURNS boolean
    LANGUAGE plpgsql
    AS $$
BEGIN
RETURN EXISTS(
  SELECT public."Users".name FROM public."Users" WHERE public."Users".name = _name
);
END;
$$;


ALTER FUNCTION public."existsUserByName"(_name text) OWNER TO sigma_admin;

--
-- TOC entry 259 (class 1255 OID 16855)
-- Name: getActionsJournalDate(); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public."getActionsJournalDate"() RETURNS TABLE(user_id bigint, description text, date date)
    LANGUAGE plpgsql
    AS $$
BEGIN
RETURN QUERY SELECT public."ActionsJournal".user_id, public."ActionsDescription".description, public."ActionsJournal".date
	FROM public."ActionsJournal" LEFT JOIN public."ActionsDescription"
	ON public."ActionsJournal".action_id = public."ActionsDescription".id;
END;
$$;


ALTER FUNCTION public."getActionsJournalDate"() OWNER TO postgres;

--
-- TOC entry 257 (class 1255 OID 16856)
-- Name: getOrdermakersCount(); Type: FUNCTION; Schema: public; Owner: sigma_admin
--

CREATE FUNCTION public."getOrdermakersCount"() RETURNS integer
    LANGUAGE plpgsql
    AS $$
DECLARE __cnt integer;
BEGIN
SELECT COUNT(1) INTO __cnt FROM public."OrderMakers" FETCH FIRST ROW ONLY;
RETURN __cnt;
END;
$$;


ALTER FUNCTION public."getOrdermakersCount"() OWNER TO sigma_admin;

--
-- TOC entry 243 (class 1255 OID 16703)
-- Name: getSaltAndHash(text); Type: FUNCTION; Schema: public; Owner: sigma_admin
--

CREATE FUNCTION public."getSaltAndHash"(_name text) RETURNS TABLE(salt text, hash text)
    LANGUAGE plpgsql
    AS $$
BEGIN
RETURN QUERY SELECT public."Users".salt, public."Users".hash FROM public."Users" WHERE public."Users".name = _name LIMIT 1;
END;
$$;


ALTER FUNCTION public."getSaltAndHash"(_name text) OWNER TO sigma_admin;

--
-- TOC entry 249 (class 1255 OID 16813)
-- Name: getTotalAuthors(); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public."getTotalAuthors"() RETURNS TABLE(id bigint, name text)
    LANGUAGE plpgsql
    AS $$
BEGIN
RETURN QUERY SELECT public."Authors".id, public."Authors".name from public."Authors";
END;
$$;


ALTER FUNCTION public."getTotalAuthors"() OWNER TO postgres;

--
-- TOC entry 250 (class 1255 OID 16814)
-- Name: getTotalCategories(); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public."getTotalCategories"() RETURNS TABLE(id bigint, name text)
    LANGUAGE plpgsql
    AS $$
BEGIN
RETURN QUERY SELECT public."Categories".id, public."Categories".name from public."Categories";
END;
$$;


ALTER FUNCTION public."getTotalCategories"() OWNER TO postgres;

--
-- TOC entry 251 (class 1255 OID 16816)
-- Name: getTotalOrdermakers(); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public."getTotalOrdermakers"() RETURNS TABLE(id bigint, name text)
    LANGUAGE plpgsql
    AS $$
BEGIN
RETURN QUERY SELECT public."OrderMakers".id, public."OrderMakers".title from public."OrderMakers";
END;
$$;


ALTER FUNCTION public."getTotalOrdermakers"() OWNER TO postgres;

--
-- TOC entry 244 (class 1255 OID 16706)
-- Name: getTotalOrders(); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public."getTotalOrders"() RETURNS TABLE(author text, ord text, category text, yr integer, ty text, typography text, ordermaker text, price real, status smallint)
    LANGUAGE plpgsql
    AS $$
BEGIN
RETURN QUERY
SELECT
public."Authors".name,
public."Orders".name,
public."Categories".name,
public."Orders".year,
public."PaperTypes".name,
public."Typography".name,
public."OrderMakers".title,
public."Orders".price,
public."Orders".status
FROM public."Orders"
LEFT JOIN public."Authors" ON public."Orders".author_id = public."Authors".id
LEFT JOIN public."Categories" ON public."Orders".category_id = public."Categories".id
LEFT JOIN public."PaperTypes" ON public."Orders".type_id = public."PaperTypes".id
LEFT JOIN public."Typography" ON public."Orders".typography_id = public."Typography".id
LEFT JOIN public."OrderMakers" ON public."Orders".ordermaker_id = public."OrderMakers".id
;
END;
$$;


ALTER FUNCTION public."getTotalOrders"() OWNER TO postgres;

--
-- TOC entry 252 (class 1255 OID 16817)
-- Name: getTotalTypes(); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public."getTotalTypes"() RETURNS TABLE(id bigint, name text)
    LANGUAGE plpgsql
    AS $$
BEGIN
RETURN QUERY SELECT public."PaperTypes".id, public."PaperTypes".name from public."PaperTypes";
END;
$$;


ALTER FUNCTION public."getTotalTypes"() OWNER TO postgres;

--
-- TOC entry 253 (class 1255 OID 16818)
-- Name: getTotalTypographies(); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public."getTotalTypographies"() RETURNS TABLE(id bigint, name text)
    LANGUAGE plpgsql
    AS $$
BEGIN
RETURN QUERY SELECT public."Typography".id, public."Typography".name from public."Typography";
END;
$$;


ALTER FUNCTION public."getTotalTypographies"() OWNER TO postgres;

--
-- TOC entry 245 (class 1255 OID 16713)
-- Name: getUntypedSaltAndHash(text); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public."getUntypedSaltAndHash"(_name text) RETURNS SETOF record
    LANGUAGE plpgsql
    AS $$
BEGIN
RETURN QUERY SELECT public."Users".salt, public."Users".hash FROM public."Users" WHERE public."Users".name = _name LIMIT 1;
END;
$$;


ALTER FUNCTION public."getUntypedSaltAndHash"(_name text) OWNER TO postgres;

--
-- TOC entry 256 (class 1255 OID 16849)
-- Name: getUserId(text); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public."getUserId"(_hash text) RETURNS bigint
    LANGUAGE plpgsql
    AS $$
DECLARE __id bigint;
BEGIN
SELECT public."Users".id INTO __id FROM public."Users" WHERE public."Users".hash = _hash FETCH FIRST ROW ONLY;
RETURN __id;
END;
$$;


ALTER FUNCTION public."getUserId"(_hash text) OWNER TO postgres;

--
-- TOC entry 247 (class 1255 OID 16714)
-- Name: getUserRights(text); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public."getUserRights"(_hash text) RETURNS smallint
    LANGUAGE plpgsql
    AS $$
DECLARE __role smallint;
BEGIN
SELECT public."Users".role_id INTO __role FROM public."Users" WHERE public."Users".hash = _hash FETCH FIRST ROW ONLY;
RETURN __role;
END;
$$;


ALTER FUNCTION public."getUserRights"(_hash text) OWNER TO postgres;

--
-- TOC entry 260 (class 1255 OID 16851)
-- Name: insertAction(bigint, integer); Type: PROCEDURE; Schema: public; Owner: postgres
--

CREATE PROCEDURE public."insertAction"(_id bigint, _action_id integer)
    LANGUAGE plpgsql
    AS $$
BEGIN
INSERT INTO public."ActionsJournal"(user_id, action_id, date)
	VALUES(_id,_action_id,NOW());
END;
$$;


ALTER PROCEDURE public."insertAction"(_id bigint, _action_id integer) OWNER TO postgres;

--
-- TOC entry 258 (class 1255 OID 16852)
-- Name: insertActionByHash(text, integer); Type: PROCEDURE; Schema: public; Owner: postgres
--

CREATE PROCEDURE public."insertActionByHash"(_hash text, _action_id integer)
    LANGUAGE plpgsql
    AS $$
DECLARE __id bigint;
BEGIN
SELECT public."Users".id INTO __id FROM public."Users" WHERE public."Users".hash = _hash FETCH FIRST ROW ONLY;
INSERT INTO public."ActionsJournal"(user_id, action_id,date)
	VALUES(__id,_action_id,NOW());
END;
$$;


ALTER PROCEDURE public."insertActionByHash"(_hash text, _action_id integer) OWNER TO postgres;

--
-- TOC entry 246 (class 1255 OID 16847)
-- Name: insertAuthor(text, date, smallint); Type: PROCEDURE; Schema: public; Owner: postgres
--

CREATE PROCEDURE public."insertAuthor"(_name text, _birthday date, _zodiac smallint)
    LANGUAGE plpgsql
    AS $$
BEGIN
INSERT INTO public."Authors"(name, birthday, zodiac_id)
	VALUES(_name,_birthday,_zodiac);
END;
$$;


ALTER PROCEDURE public."insertAuthor"(_name text, _birthday date, _zodiac smallint) OWNER TO postgres;

--
-- TOC entry 254 (class 1255 OID 16846)
-- Name: insertOrder(bigint, text, bigint, integer, bigint, bigint, bigint, real); Type: PROCEDURE; Schema: public; Owner: postgres
--

CREATE PROCEDURE public."insertOrder"(_author_id bigint, _name text, _category_id bigint, _year integer, _type_id bigint, _typography_id bigint, _ordermaker_id bigint, _price real)
    LANGUAGE plpgsql
    AS $$
BEGIN
INSERT INTO public."Orders"(author_id, name, category_id, year, type_id, typography_id, ordermaker_id, price, picture_id, file_id, status)
	VALUES(_author_id, _name, _category_id, _year, _type_id, _typography_id, _ordermaker_id, _price, 1, 1, 1);
END;
$$;


ALTER PROCEDURE public."insertOrder"(_author_id bigint, _name text, _category_id bigint, _year integer, _type_id bigint, _typography_id bigint, _ordermaker_id bigint, _price real) OWNER TO postgres;

--
-- TOC entry 255 (class 1255 OID 16848)
-- Name: insertOrdermaker(boolean, text, text, text, text); Type: PROCEDURE; Schema: public; Owner: postgres
--

CREATE PROCEDURE public."insertOrdermaker"(_is_organization boolean, _contact_name text, _address text, _phone text, _title text)
    LANGUAGE plpgsql
    AS $$
BEGIN
INSERT INTO public."OrderMakers"(is_organization, contact_name, address, phone, title)
	VALUES(_is_organization, _contact_name, _address, _phone, _title);
END;
$$;


ALTER PROCEDURE public."insertOrdermaker"(_is_organization boolean, _contact_name text, _address text, _phone text, _title text) OWNER TO postgres;

--
-- TOC entry 248 (class 1255 OID 16716)
-- Name: insertTypography(text, text, text); Type: PROCEDURE; Schema: public; Owner: postgres
--

CREATE PROCEDURE public."insertTypography"(_name text, _address text, _phone text)
    LANGUAGE plpgsql
    AS $$
BEGIN
INSERT INTO public."Typography"(name, address, phone) VALUES(_name,_address,_phone);
END;
$$;


ALTER PROCEDURE public."insertTypography"(_name text, _address text, _phone text) OWNER TO postgres;

--
-- TOC entry 241 (class 1255 OID 16664)
-- Name: insertUser(text, text, text); Type: PROCEDURE; Schema: public; Owner: postgres
--

CREATE PROCEDURE public."insertUser"(_name text, _hash text, _salt text)
    LANGUAGE plpgsql
    AS $$
BEGIN
INSERT INTO public."Users"(role_id, name, hash, salt) VALUES(1, _name,_hash,_salt);
END;
$$;


ALTER PROCEDURE public."insertUser"(_name text, _hash text, _salt text) OWNER TO postgres;

--
-- TOC entry 242 (class 1255 OID 16692)
-- Name: validHash(text); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public."validHash"(_hash text) RETURNS boolean
    LANGUAGE plpgsql
    AS $$
BEGIN
RETURN EXISTS(
	SELECT hash FROM public."Users" WHERE public."Users".hash = _hash
);
END;
$$;


ALTER FUNCTION public."validHash"(_hash text) OWNER TO postgres;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- TOC entry 225 (class 1259 OID 16667)
-- Name: ActionsDescription; Type: TABLE; Schema: public; Owner: sigma_admin
--

CREATE TABLE public."ActionsDescription" (
    id bigint NOT NULL,
    description text
);


ALTER TABLE public."ActionsDescription" OWNER TO sigma_admin;

--
-- TOC entry 224 (class 1259 OID 16665)
-- Name: ActionsDescription_id_seq; Type: SEQUENCE; Schema: public; Owner: sigma_admin
--

ALTER TABLE public."ActionsDescription" ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public."ActionsDescription_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 227 (class 1259 OID 16677)
-- Name: ActionsJournal; Type: TABLE; Schema: public; Owner: sigma_admin
--

CREATE TABLE public."ActionsJournal" (
    id bigint NOT NULL,
    user_id bigint NOT NULL,
    action_id integer NOT NULL,
    date date
);


ALTER TABLE public."ActionsJournal" OWNER TO sigma_admin;

--
-- TOC entry 226 (class 1259 OID 16675)
-- Name: ActionsJournal_id_seq; Type: SEQUENCE; Schema: public; Owner: sigma_admin
--

ALTER TABLE public."ActionsJournal" ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public."ActionsJournal_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 221 (class 1259 OID 16558)
-- Name: Authors; Type: TABLE; Schema: public; Owner: sigma_admin
--

CREATE TABLE public."Authors" (
    id bigint NOT NULL,
    name text,
    birthday date,
    zodiac_id smallint
);


ALTER TABLE public."Authors" OWNER TO sigma_admin;

--
-- TOC entry 220 (class 1259 OID 16556)
-- Name: Authors_id_seq; Type: SEQUENCE; Schema: public; Owner: sigma_admin
--

ALTER TABLE public."Authors" ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public."Authors_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 215 (class 1259 OID 16526)
-- Name: Categories; Type: TABLE; Schema: public; Owner: sigma_admin
--

CREATE TABLE public."Categories" (
    id bigint NOT NULL,
    name text
);


ALTER TABLE public."Categories" OWNER TO sigma_admin;

--
-- TOC entry 214 (class 1259 OID 16524)
-- Name: Categories_id_seq; Type: SEQUENCE; Schema: public; Owner: sigma_admin
--

ALTER TABLE public."Categories" ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public."Categories_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 217 (class 1259 OID 16536)
-- Name: Files; Type: TABLE; Schema: public; Owner: sigma_admin
--

CREATE TABLE public."Files" (
    id bigint NOT NULL,
    path text
);


ALTER TABLE public."Files" OWNER TO sigma_admin;

--
-- TOC entry 216 (class 1259 OID 16534)
-- Name: Files_id_seq; Type: SEQUENCE; Schema: public; Owner: sigma_admin
--

ALTER TABLE public."Files" ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public."Files_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 207 (class 1259 OID 16486)
-- Name: OrderMakers; Type: TABLE; Schema: public; Owner: sigma_admin
--

CREATE TABLE public."OrderMakers" (
    id bigint NOT NULL,
    is_organization boolean,
    contact_name text,
    address text,
    phone text,
    title text
);


ALTER TABLE public."OrderMakers" OWNER TO sigma_admin;

--
-- TOC entry 206 (class 1259 OID 16484)
-- Name: OrderMakers_id_seq; Type: SEQUENCE; Schema: public; Owner: sigma_admin
--

ALTER TABLE public."OrderMakers" ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public."OrderMakers_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 223 (class 1259 OID 16573)
-- Name: Orders; Type: TABLE; Schema: public; Owner: sigma_admin
--

CREATE TABLE public."Orders" (
    id bigint NOT NULL,
    author_id bigint,
    name text,
    category_id bigint,
    year integer,
    type_id bigint,
    typography_id bigint,
    ordermaker_id bigint,
    price real,
    picture_id bigint,
    file_id bigint,
    status smallint
);


ALTER TABLE public."Orders" OWNER TO sigma_admin;

--
-- TOC entry 222 (class 1259 OID 16571)
-- Name: Orders_id_seq; Type: SEQUENCE; Schema: public; Owner: sigma_admin
--

ALTER TABLE public."Orders" ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public."Orders_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 213 (class 1259 OID 16516)
-- Name: PaperTypes; Type: TABLE; Schema: public; Owner: sigma_admin
--

CREATE TABLE public."PaperTypes" (
    id bigint NOT NULL,
    name text
);


ALTER TABLE public."PaperTypes" OWNER TO sigma_admin;

--
-- TOC entry 212 (class 1259 OID 16514)
-- Name: PaperTypes_id_seq; Type: SEQUENCE; Schema: public; Owner: sigma_admin
--

ALTER TABLE public."PaperTypes" ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public."PaperTypes_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 209 (class 1259 OID 16496)
-- Name: Pictures; Type: TABLE; Schema: public; Owner: sigma_admin
--

CREATE TABLE public."Pictures" (
    id bigint NOT NULL,
    path text
);


ALTER TABLE public."Pictures" OWNER TO sigma_admin;

--
-- TOC entry 208 (class 1259 OID 16494)
-- Name: Pictures_id_seq; Type: SEQUENCE; Schema: public; Owner: sigma_admin
--

ALTER TABLE public."Pictures" ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public."Pictures_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 202 (class 1259 OID 16410)
-- Name: Roles; Type: TABLE; Schema: public; Owner: sigma_admin
--

CREATE TABLE public."Roles" (
    id bigint NOT NULL,
    name text
);


ALTER TABLE public."Roles" OWNER TO sigma_admin;

--
-- TOC entry 203 (class 1259 OID 16458)
-- Name: Roles_id_seq; Type: SEQUENCE; Schema: public; Owner: sigma_admin
--

ALTER TABLE public."Roles" ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public."Roles_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 211 (class 1259 OID 16504)
-- Name: Typography; Type: TABLE; Schema: public; Owner: sigma_admin
--

CREATE TABLE public."Typography" (
    id bigint NOT NULL,
    name text,
    address text,
    phone text
);


ALTER TABLE public."Typography" OWNER TO sigma_admin;

--
-- TOC entry 210 (class 1259 OID 16502)
-- Name: Typography_id_seq; Type: SEQUENCE; Schema: public; Owner: sigma_admin
--

ALTER TABLE public."Typography" ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public."Typography_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 205 (class 1259 OID 16468)
-- Name: Users; Type: TABLE; Schema: public; Owner: sigma_admin
--

CREATE TABLE public."Users" (
    id bigint NOT NULL,
    role_id smallint NOT NULL,
    name text,
    hash text,
    salt text
);


ALTER TABLE public."Users" OWNER TO sigma_admin;

--
-- TOC entry 204 (class 1259 OID 16466)
-- Name: Users_id_seq; Type: SEQUENCE; Schema: public; Owner: sigma_admin
--

ALTER TABLE public."Users" ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public."Users_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 219 (class 1259 OID 16546)
-- Name: Zodiac; Type: TABLE; Schema: public; Owner: sigma_admin
--

CREATE TABLE public."Zodiac" (
    id bigint NOT NULL,
    name text
);


ALTER TABLE public."Zodiac" OWNER TO sigma_admin;

--
-- TOC entry 218 (class 1259 OID 16544)
-- Name: Zodiac_id_seq; Type: SEQUENCE; Schema: public; Owner: sigma_admin
--

ALTER TABLE public."Zodiac" ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public."Zodiac_id_seq"
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 2978 (class 0 OID 16667)
-- Dependencies: 225
-- Data for Name: ActionsDescription; Type: TABLE DATA; Schema: public; Owner: sigma_admin
--

COPY public."ActionsDescription" (id, description) FROM stdin;
1	Login
2	New order
3	New typography
4	New author
5	New ordermaker
6	Logout
7	Unknown
8	Fetch data
9	Fetch actions journal
\.


--
-- TOC entry 2980 (class 0 OID 16677)
-- Dependencies: 227
-- Data for Name: ActionsJournal; Type: TABLE DATA; Schema: public; Owner: sigma_admin
--

COPY public."ActionsJournal" (id, user_id, action_id, date) FROM stdin;
1	4	1	\N
71	4	1	\N
72	4	9	\N
73	4	1	2021-10-18
74	4	9	2021-10-18
75	4	1	2021-10-18
76	4	9	2021-10-18
77	4	1	2021-10-18
78	4	1	2021-10-18
79	4	1	2021-10-18
80	4	1	2021-10-18
81	4	1	2021-10-18
82	4	1	2021-10-18
83	4	1	2021-10-18
84	4	1	2021-10-18
85	4	9	2021-10-18
86	4	1	2021-10-18
87	4	9	2021-10-18
88	4	1	2021-10-18
89	4	1	2021-10-18
95	4	1	2021-10-18
96	4	9	2021-10-18
97	4	1	2021-10-18
98	4	1	2021-10-18
99	4	1	2021-10-18
100	4	1	2021-10-18
101	4	1	2021-10-18
102	4	1	2021-10-18
103	4	1	2021-10-18
104	4	1	2021-10-18
105	4	1	2021-10-18
106	4	1	2021-10-18
108	4	1	2021-10-18
109	4	1	2021-10-26
110	4	1	2021-10-26
111	4	1	2021-10-26
112	4	1	2021-10-26
113	2	1	2021-10-26
114	4	1	2021-10-26
115	4	9	2021-10-26
116	4	1	2021-10-26
117	4	9	2021-10-26
118	4	1	2021-11-18
119	4	1	2021-11-18
120	4	9	2021-11-18
121	4	1	2021-11-18
122	4	9	2021-11-18
123	4	1	2021-11-18
124	4	1	2021-11-18
125	4	1	2021-11-18
126	4	1	2021-11-18
127	4	1	2021-11-18
128	4	1	2021-11-18
129	4	1	2021-11-18
130	4	9	2021-11-18
131	4	1	2021-11-18
132	4	9	2021-11-18
133	4	4	2021-11-18
134	4	4	2021-11-18
135	4	4	2021-11-18
136	4	1	2021-11-18
\.


--
-- TOC entry 2974 (class 0 OID 16558)
-- Dependencies: 221
-- Data for Name: Authors; Type: TABLE DATA; Schema: public; Owner: sigma_admin
--

COPY public."Authors" (id, name, birthday, zodiac_id) FROM stdin;
1	F.M. Dostoevsky	1821-11-11	9
2	V.O. Pelevin	1962-11-22	9
3	А.С.Пушкин	1799-06-06	4
4	Т.Е.Стасов	2004-08-11	6
5	А.Полярный	2001-01-01	11
6	&lt;script&gt;alert(&#x27;malicious!&#x27;)&lt;&#x2F;script&gt;	1941-11-13	1
\.


--
-- TOC entry 2968 (class 0 OID 16526)
-- Dependencies: 215
-- Data for Name: Categories; Type: TABLE DATA; Schema: public; Owner: sigma_admin
--

COPY public."Categories" (id, name) FROM stdin;
2	Other
3	Drama
4	Comedy
5	Sci-fi
6	Fantasy
7	Detective
8	Management
9	Regular
10	Medicine
11	Anecdotes
12	Selfimprovement
\.


--
-- TOC entry 2970 (class 0 OID 16536)
-- Dependencies: 217
-- Data for Name: Files; Type: TABLE DATA; Schema: public; Owner: sigma_admin
--

COPY public."Files" (id, path) FROM stdin;
1	-1
\.


--
-- TOC entry 2960 (class 0 OID 16486)
-- Dependencies: 207
-- Data for Name: OrderMakers; Type: TABLE DATA; Schema: public; Owner: sigma_admin
--

COPY public."OrderMakers" (id, is_organization, contact_name, address, phone, title) FROM stdin;
1	t	Petrovich	Saratov, Bolshaya st., 33, 1	+76661122101	Sosh n6
2	t	Валерий Леонтьевич Пникс	г. Многопивск, ул. Синяя, д. 3	-	Пникс Многопивский
\.


--
-- TOC entry 2976 (class 0 OID 16573)
-- Dependencies: 223
-- Data for Name: Orders; Type: TABLE DATA; Schema: public; Owner: sigma_admin
--

COPY public."Orders" (id, author_id, name, category_id, year, type_id, typography_id, ordermaker_id, price, picture_id, file_id, status) FROM stdin;
19	3	Капитанская дочка	2	1836	1	1	1	500	1	1	1
3	1	Crime and punishment	2	1866	2	1	1	400	1	1	2
4	2	Поколение П	2	1999	2	2	1	450	1	1	1
5	2	Чапаев и пустота	2	1996	1	2	1	450	1	1	1
6	1	Братья Карамазовы	2	1880	2	1	1	450	1	1	1
20	4	Тоска зеленая	2	2020	2	3	2	1500	1	1	3
21	4	Тоска красная	11	2019	4	2	2	1200	1	1	3
\.


--
-- TOC entry 2966 (class 0 OID 16516)
-- Dependencies: 213
-- Data for Name: PaperTypes; Type: TABLE DATA; Schema: public; Owner: sigma_admin
--

COPY public."PaperTypes" (id, name) FROM stdin;
1	Solid book
2	Soft book
3	Picture
4	booklet
5	Elections voting paper
\.


--
-- TOC entry 2962 (class 0 OID 16496)
-- Dependencies: 209
-- Data for Name: Pictures; Type: TABLE DATA; Schema: public; Owner: sigma_admin
--

COPY public."Pictures" (id, path) FROM stdin;
1	default
\.


--
-- TOC entry 2955 (class 0 OID 16410)
-- Dependencies: 202
-- Data for Name: Roles; Type: TABLE DATA; Schema: public; Owner: sigma_admin
--

COPY public."Roles" (id, name) FROM stdin;
1	user
2	employer
3	admin
\.


--
-- TOC entry 2964 (class 0 OID 16504)
-- Dependencies: 211
-- Data for Name: Typography; Type: TABLE DATA; Schema: public; Owner: sigma_admin
--

COPY public."Typography" (id, name, address, phone) FROM stdin;
1	Petrovka	Petrovskoe, Sovetskaya st., 22	441133
2	Пожилухино	с. Пожилухино, ул. Печкина, д. 2	-
3	Семеновка	г. Москва, ул. Садовая, д. 5	89271026265
\.


--
-- TOC entry 2958 (class 0 OID 16468)
-- Dependencies: 205
-- Data for Name: Users; Type: TABLE DATA; Schema: public; Owner: sigma_admin
--

COPY public."Users" (id, role_id, name, hash, salt) FROM stdin;
1	3	admin	\N	\N
2	1	11	$scrypt$ln=15,r=8,p=1$3EW4sy4LOOuVMuJxdqWayw$k8kneRPWM93V+wmbX/ba1dwmTwY3UmhXmcsbIeEpwOI	3EW4sy4LOOuVMuJxdqWayw
4	3	112	$scrypt$ln=15,r=8,p=1$OG7/H6TRIN+scuLP1vHUPQ$KGZgidHncWcU0mX8HwqJNUl8THcluikegBjsTMk0Dbs	OG7/H6TRIN+scuLP1vHUPQ
5	1	root	$scrypt$ln=15,r=8,p=1$kr3qh0QtoIoyqliVvXtZ1Q$MVgSzuj0wJ9tgnEdmIJi4Ym9M4Ep5isfWkfGQniT8Fc	kr3qh0QtoIoyqliVvXtZ1Q
\.


--
-- TOC entry 2972 (class 0 OID 16546)
-- Dependencies: 219
-- Data for Name: Zodiac; Type: TABLE DATA; Schema: public; Owner: sigma_admin
--

COPY public."Zodiac" (id, name) FROM stdin;
1	Unknown
2	Ram
3	Bull
4	Twins
6	Lion
10	Archer
11	Goat
12	Water-bearer
13	Fish
5	Crab
7	Maiden
8	Scales
9	Scorpion
\.


--
-- TOC entry 2986 (class 0 OID 0)
-- Dependencies: 224
-- Name: ActionsDescription_id_seq; Type: SEQUENCE SET; Schema: public; Owner: sigma_admin
--

SELECT pg_catalog.setval('public."ActionsDescription_id_seq"', 9, true);


--
-- TOC entry 2987 (class 0 OID 0)
-- Dependencies: 226
-- Name: ActionsJournal_id_seq; Type: SEQUENCE SET; Schema: public; Owner: sigma_admin
--

SELECT pg_catalog.setval('public."ActionsJournal_id_seq"', 136, true);


--
-- TOC entry 2988 (class 0 OID 0)
-- Dependencies: 220
-- Name: Authors_id_seq; Type: SEQUENCE SET; Schema: public; Owner: sigma_admin
--

SELECT pg_catalog.setval('public."Authors_id_seq"', 6, true);


--
-- TOC entry 2989 (class 0 OID 0)
-- Dependencies: 214
-- Name: Categories_id_seq; Type: SEQUENCE SET; Schema: public; Owner: sigma_admin
--

SELECT pg_catalog.setval('public."Categories_id_seq"', 12, true);


--
-- TOC entry 2990 (class 0 OID 0)
-- Dependencies: 216
-- Name: Files_id_seq; Type: SEQUENCE SET; Schema: public; Owner: sigma_admin
--

SELECT pg_catalog.setval('public."Files_id_seq"', 1, true);


--
-- TOC entry 2991 (class 0 OID 0)
-- Dependencies: 206
-- Name: OrderMakers_id_seq; Type: SEQUENCE SET; Schema: public; Owner: sigma_admin
--

SELECT pg_catalog.setval('public."OrderMakers_id_seq"', 2, true);


--
-- TOC entry 2992 (class 0 OID 0)
-- Dependencies: 222
-- Name: Orders_id_seq; Type: SEQUENCE SET; Schema: public; Owner: sigma_admin
--

SELECT pg_catalog.setval('public."Orders_id_seq"', 21, true);


--
-- TOC entry 2993 (class 0 OID 0)
-- Dependencies: 212
-- Name: PaperTypes_id_seq; Type: SEQUENCE SET; Schema: public; Owner: sigma_admin
--

SELECT pg_catalog.setval('public."PaperTypes_id_seq"', 5, true);


--
-- TOC entry 2994 (class 0 OID 0)
-- Dependencies: 208
-- Name: Pictures_id_seq; Type: SEQUENCE SET; Schema: public; Owner: sigma_admin
--

SELECT pg_catalog.setval('public."Pictures_id_seq"', 1, true);


--
-- TOC entry 2995 (class 0 OID 0)
-- Dependencies: 203
-- Name: Roles_id_seq; Type: SEQUENCE SET; Schema: public; Owner: sigma_admin
--

SELECT pg_catalog.setval('public."Roles_id_seq"', 3, true);


--
-- TOC entry 2996 (class 0 OID 0)
-- Dependencies: 210
-- Name: Typography_id_seq; Type: SEQUENCE SET; Schema: public; Owner: sigma_admin
--

SELECT pg_catalog.setval('public."Typography_id_seq"', 3, true);


--
-- TOC entry 2997 (class 0 OID 0)
-- Dependencies: 204
-- Name: Users_id_seq; Type: SEQUENCE SET; Schema: public; Owner: sigma_admin
--

SELECT pg_catalog.setval('public."Users_id_seq"', 5, true);


--
-- TOC entry 2998 (class 0 OID 0)
-- Dependencies: 218
-- Name: Zodiac_id_seq; Type: SEQUENCE SET; Schema: public; Owner: sigma_admin
--

SELECT pg_catalog.setval('public."Zodiac_id_seq"', 13, true);


--
-- TOC entry 2815 (class 2606 OID 16724)
-- Name: ActionsDescription ActionsDescription_pkey; Type: CONSTRAINT; Schema: public; Owner: sigma_admin
--

ALTER TABLE ONLY public."ActionsDescription"
    ADD CONSTRAINT "ActionsDescription_pkey" PRIMARY KEY (id);


--
-- TOC entry 2811 (class 2606 OID 16565)
-- Name: Authors Authors_pkey; Type: CONSTRAINT; Schema: public; Owner: sigma_admin
--

ALTER TABLE ONLY public."Authors"
    ADD CONSTRAINT "Authors_pkey" PRIMARY KEY (id);


--
-- TOC entry 2805 (class 2606 OID 16739)
-- Name: Categories Categories_pkey; Type: CONSTRAINT; Schema: public; Owner: sigma_admin
--

ALTER TABLE ONLY public."Categories"
    ADD CONSTRAINT "Categories_pkey" PRIMARY KEY (id);


--
-- TOC entry 2807 (class 2606 OID 16543)
-- Name: Files Files_pkey; Type: CONSTRAINT; Schema: public; Owner: sigma_admin
--

ALTER TABLE ONLY public."Files"
    ADD CONSTRAINT "Files_pkey" PRIMARY KEY (id);


--
-- TOC entry 2797 (class 2606 OID 16754)
-- Name: OrderMakers OrderMakers_pkey; Type: CONSTRAINT; Schema: public; Owner: sigma_admin
--

ALTER TABLE ONLY public."OrderMakers"
    ADD CONSTRAINT "OrderMakers_pkey" PRIMARY KEY (id);


--
-- TOC entry 2813 (class 2606 OID 16580)
-- Name: Orders Orders_pkey; Type: CONSTRAINT; Schema: public; Owner: sigma_admin
--

ALTER TABLE ONLY public."Orders"
    ADD CONSTRAINT "Orders_pkey" PRIMARY KEY (id);


--
-- TOC entry 2803 (class 2606 OID 16769)
-- Name: PaperTypes PaperTypes_pkey; Type: CONSTRAINT; Schema: public; Owner: sigma_admin
--

ALTER TABLE ONLY public."PaperTypes"
    ADD CONSTRAINT "PaperTypes_pkey" PRIMARY KEY (id);


--
-- TOC entry 2799 (class 2606 OID 16513)
-- Name: Pictures Pictures_pkey; Type: CONSTRAINT; Schema: public; Owner: sigma_admin
--

ALTER TABLE ONLY public."Pictures"
    ADD CONSTRAINT "Pictures_pkey" PRIMARY KEY (id);


--
-- TOC entry 2793 (class 2606 OID 16784)
-- Name: Roles Roles_pkey; Type: CONSTRAINT; Schema: public; Owner: sigma_admin
--

ALTER TABLE ONLY public."Roles"
    ADD CONSTRAINT "Roles_pkey" PRIMARY KEY (id);


--
-- TOC entry 2801 (class 2606 OID 16511)
-- Name: Typography Typography_pkey; Type: CONSTRAINT; Schema: public; Owner: sigma_admin
--

ALTER TABLE ONLY public."Typography"
    ADD CONSTRAINT "Typography_pkey" PRIMARY KEY (id);


--
-- TOC entry 2795 (class 2606 OID 16472)
-- Name: Users Users_pkey; Type: CONSTRAINT; Schema: public; Owner: sigma_admin
--

ALTER TABLE ONLY public."Users"
    ADD CONSTRAINT "Users_pkey" PRIMARY KEY (id);


--
-- TOC entry 2809 (class 2606 OID 16799)
-- Name: Zodiac Zodiac_pkey; Type: CONSTRAINT; Schema: public; Owner: sigma_admin
--

ALTER TABLE ONLY public."Zodiac"
    ADD CONSTRAINT "Zodiac_pkey" PRIMARY KEY (id);


--
-- TOC entry 2817 (class 2606 OID 16681)
-- Name: ActionsJournal id; Type: CONSTRAINT; Schema: public; Owner: sigma_admin
--

ALTER TABLE ONLY public."ActionsJournal"
    ADD CONSTRAINT id PRIMARY KEY (id) INCLUDE (id);


--
-- TOC entry 2820 (class 2606 OID 16581)
-- Name: Orders author; Type: FK CONSTRAINT; Schema: public; Owner: sigma_admin
--

ALTER TABLE ONLY public."Orders"
    ADD CONSTRAINT author FOREIGN KEY (author_id) REFERENCES public."Authors"(id);


--
-- TOC entry 2825 (class 2606 OID 16819)
-- Name: Orders category; Type: FK CONSTRAINT; Schema: public; Owner: sigma_admin
--

ALTER TABLE ONLY public."Orders"
    ADD CONSTRAINT category FOREIGN KEY (category_id) REFERENCES public."Categories"(id);


--
-- TOC entry 2823 (class 2606 OID 16611)
-- Name: Orders file; Type: FK CONSTRAINT; Schema: public; Owner: sigma_admin
--

ALTER TABLE ONLY public."Orders"
    ADD CONSTRAINT file FOREIGN KEY (file_id) REFERENCES public."Files"(id);


--
-- TOC entry 2824 (class 2606 OID 16755)
-- Name: Orders ordermaker; Type: FK CONSTRAINT; Schema: public; Owner: sigma_admin
--

ALTER TABLE ONLY public."Orders"
    ADD CONSTRAINT ordermaker FOREIGN KEY (ordermaker_id) REFERENCES public."OrderMakers"(id);


--
-- TOC entry 2822 (class 2606 OID 16606)
-- Name: Orders picture; Type: FK CONSTRAINT; Schema: public; Owner: sigma_admin
--

ALTER TABLE ONLY public."Orders"
    ADD CONSTRAINT picture FOREIGN KEY (picture_id) REFERENCES public."Pictures"(id);


--
-- TOC entry 2818 (class 2606 OID 16785)
-- Name: Users role_id; Type: FK CONSTRAINT; Schema: public; Owner: sigma_admin
--

ALTER TABLE ONLY public."Users"
    ADD CONSTRAINT role_id FOREIGN KEY (role_id) REFERENCES public."Roles"(id);


--
-- TOC entry 2828 (class 2606 OID 16725)
-- Name: ActionsJournal ty; Type: FK CONSTRAINT; Schema: public; Owner: sigma_admin
--

ALTER TABLE ONLY public."ActionsJournal"
    ADD CONSTRAINT ty FOREIGN KEY (action_id) REFERENCES public."ActionsDescription"(id);


--
-- TOC entry 2826 (class 2606 OID 16831)
-- Name: Orders type; Type: FK CONSTRAINT; Schema: public; Owner: sigma_admin
--

ALTER TABLE ONLY public."Orders"
    ADD CONSTRAINT type FOREIGN KEY (type_id) REFERENCES public."PaperTypes"(id);


--
-- TOC entry 2821 (class 2606 OID 16596)
-- Name: Orders typography; Type: FK CONSTRAINT; Schema: public; Owner: sigma_admin
--

ALTER TABLE ONLY public."Orders"
    ADD CONSTRAINT typography FOREIGN KEY (typography_id) REFERENCES public."Typography"(id);


--
-- TOC entry 2827 (class 2606 OID 16682)
-- Name: ActionsJournal user; Type: FK CONSTRAINT; Schema: public; Owner: sigma_admin
--

ALTER TABLE ONLY public."ActionsJournal"
    ADD CONSTRAINT "user" FOREIGN KEY (user_id) REFERENCES public."Users"(id);


--
-- TOC entry 2819 (class 2606 OID 16800)
-- Name: Authors zodiac; Type: FK CONSTRAINT; Schema: public; Owner: sigma_admin
--

ALTER TABLE ONLY public."Authors"
    ADD CONSTRAINT zodiac FOREIGN KEY (zodiac_id) REFERENCES public."Zodiac"(id) ON UPDATE CASCADE ON DELETE CASCADE;


-- Completed on 2021-11-18 01:20:19

--
-- PostgreSQL database dump complete
--
