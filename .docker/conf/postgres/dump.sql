--
-- PostgreSQL database dump
--

-- Dumped from database version 11.3
-- Dumped by pg_dump version 11.3

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

DROP DATABASE IF EXISTS blog;
--
-- Name: blog; Type: DATABASE; Schema: -; Owner: postgres
--

CREATE DATABASE blog WITH TEMPLATE = template0 ENCODING = 'UTF8' LC_COLLATE = 'C' LC_CTYPE = 'en_US.UTF-8';


ALTER DATABASE blog OWNER TO postgres;

\connect blog

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

SET default_tablespace = '';

SET default_with_oids = false;

--
-- Name: __diesel_schema_migrations; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.__diesel_schema_migrations (
    version character varying(50) NOT NULL,
    run_on timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.__diesel_schema_migrations OWNER TO postgres;

--
-- Name: post; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.post (
    id integer NOT NULL,
    title character varying NOT NULL,
    subtitle character varying NOT NULL,
    raw_content text NOT NULL,
    rendered_content text NOT NULL,
    create_time timestamp without time zone NOT NULL,
    modify_time timestamp without time zone NOT NULL,
    post_type integer NOT NULL,
    hit_time integer NOT NULL,
    published boolean DEFAULT false NOT NULL,
    slug_url character varying NOT NULL,
    enable_comment boolean DEFAULT true NOT NULL
);


ALTER TABLE public.post OWNER TO postgres;

--
-- Name: post_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.post_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.post_id_seq OWNER TO postgres;

--
-- Name: post_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.post_id_seq OWNED BY public.post.id;


--
-- Name: user; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."user" (
    id integer NOT NULL,
    username character varying(64) NOT NULL,
    hashed_password character varying(128) NOT NULL,
    create_time timestamp without time zone NOT NULL,
    modify_time timestamp without time zone NOT NULL,
    email character varying(128) NOT NULL,
    avatar_url character varying(128)
);


ALTER TABLE public."user" OWNER TO postgres;

--
-- Name: user_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.user_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.user_id_seq OWNER TO postgres;

--
-- Name: user_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.user_id_seq OWNED BY public."user".id;


--
-- Name: visitor_log; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.visitor_log (
    id integer NOT NULL,
    ip inet NOT NULL,
    access_time timestamp without time zone NOT NULL,
    user_id integer DEFAULT 0 NOT NULL
);


ALTER TABLE public.visitor_log OWNER TO postgres;

--
-- Name: visitor_log_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.visitor_log_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.visitor_log_id_seq OWNER TO postgres;

--
-- Name: visitor_log_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.visitor_log_id_seq OWNED BY public.visitor_log.id;


--
-- Name: post id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.post ALTER COLUMN id SET DEFAULT nextval('public.post_id_seq'::regclass);


--
-- Name: user id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."user" ALTER COLUMN id SET DEFAULT nextval('public.user_id_seq'::regclass);


--
-- Name: visitor_log id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.visitor_log ALTER COLUMN id SET DEFAULT nextval('public.visitor_log_id_seq'::regclass);


--
-- Data for Name: __diesel_schema_migrations; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.__diesel_schema_migrations (version, run_on) FROM stdin;
00000000000000	2019-05-21 07:13:39.766691
20170723104134	2019-05-21 07:13:39.773715
20170723104153	2019-05-21 07:13:39.781829
20170914143713	2019-05-21 07:13:39.786882
\.


--
-- Data for Name: post; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.post (id, title, subtitle, raw_content, rendered_content, create_time, modify_time, post_type, hit_time, published, slug_url, enable_comment) FROM stdin;
5	Bloomfilter	布隆过滤器	# 爬虫去重\n笔者最近思考如何编写高效的爬虫; 而在编写高效爬虫的时候，有一个必需解决的问题就是：\nurl 的去重，即如何判别 url 是否已经被爬取，如果被爬取，那就不要重复爬取，不然的\n话，爬虫会重复爬取已经爬取过的页面，不但重复工作，还占用了大量的带宽和CPU资源，\n造成爬虫的效率低下。所以在编写爬虫的时候，对于提高爬虫的效率，如何实现去重就显得\n尤为关键。一般如果需要爬取的网站不是非常庞大的话，使用Python 内置的 set 就可以实\n现去重了，但是使用 set 内存利用率不高，此外对于那些不像Python 那样用 hash 实现的\nset 而言，时间复杂度是 `log(N)`,实在难说高效。\n# Bloom Filter\n那么如何实现高效的去重呢？ 笔者查阅资料之后得知：使用布隆过滤器 (Bloom\nFilter). 布隆过滤器可以用于快速检索一个元素是否在一个集合中。布隆过滤器实际上\n是一个很长的二进制向量和一系列随机映射函数（Hash函数）。而一般的判断一个元素是\n否在一个集合里面的做法是：用需要判断的元素和集合中的元素进行比较，一般的数据结\n构，例如链表，树，都是这么实现的。缺点是：随着集合元素的增多，需要比较的元素也\n增多，检索速度就越来越慢。而使用布隆过滤器判重可以实现常数级的时间复杂度(检索\n时间不随元素增长而增加).那么布隆过滤器又是怎样实现的呢\n## 布隆过滤器实现原理\n一个Bloom Filter是基于一个m位的位向量（`Bit Vector`），这些位向量的初始值为0,并\n且有一系列的 hash 函数，hash 函数值域为1-m.在下面例子中，是15位的位向量，初始\n值为0以空白表示，为1以颜色填充\n\n![](https://imgur.com/EJ6CTTe.jpg)\n\n现在有两个简单的 hash 函数：fnv,murmur.现在我输入一个字符串 "whatever" ,然后\n分别使用两个 hash 函数对 "whatever" 进行散列计算并且映射到上面的位向量。\n\n![](https://imgur.com/np1MtvA.jpg)\n\n可知，使用 fnv 函数计算出的 hash 值是11,使用 murmur 函数计算出的 hash 值\n是4. 然后映射到位向量上：\n\n![](https://imgur.com/98EANOh.jpg)\n\n如果下一次，笔者要判断 `whatever` 是否在字符串中，只需使用 `fnv` 和 `murmur` 两个\nhash 函数对 `whatever` 进行散列值计算，然后与位向量做 "与运算"，如果结果为0,\n那么说明 `whatever` 是不在集合中的，因为同样的元素使用同一个 hash 函数产生的\n值每次都是相同的，不相同就说明不是同一个元素。但是如果 "与运算" 的结果为1,是\n否可以说明 `whatever` 就在集合中呢？其实上是不能100% 确定的，因为 hash 函数存\n在散列冲突现象 (即两个散列值相同，但两个输入值是不同的),所以布隆过滤器只能说"\n我可以说这个元素我在集合中是看见过滴，只是我有一定的不确定性".当你在分配的内\n存足够大之后，不确定性会变得很小很小。你可以看到布隆过滤器可以有效利用内存实\n现常数级的判重任务，但是鱼和熊掌不可得兼，付出的代价就是一定的误判 (机率很小),所\n以本质上，布隆过滤器是 "概率数据结构 (`probabilistic data structure`)".这个就是\n布隆过滤器的基本原理。当然，位向量不会只是15位，hash 函数也不会仅是两个简单的\n函数.这只是简化枝节，为了清晰解述原理而已。\n# Python BloomFilter\n算法都是为了实际问题服务的，又回到爬虫这个话题上。在了解布隆过滤器原理之后，可\n以很容易地实现自己的布隆过滤器，但是想要实现一个高效健壮的布隆过滤器就需要比较\n多的功夫了，因为需要考虑的问题略多。幸好，得益Python 强大的社区，已经有[PythonBloomFilter](https://axiak.github.io/pybloomfiltermmap/)\n的库。一个文档中的简单例子，判断 `apple` 是否在布隆过滤器中：\n\n```\nfrom pybloomfilter import BloomFilter\n\nbf = BloomFilter(10000000, 0.01, 'filter.bloom')\n\nwith open("/usr/share/dict/words") as f:\n\tfor word in f:\n\t\tbf.add(word.rstrip())\n\nprint 'apple' in bf\n```\n结果为 `True`\n# 小结\n一个简单的爬虫高效去重实现：\n```\nfrom pybloomfilter import BloomFilter\ncrawled_urls = BloomFilter(10000000, 0.01, 'filter.bloom')\ndef filter(new_url):\n\tif new_url not in crawled_urls: #new url\n\t\tcrawled_urls.add(new_url)\n\t\tcrawl(new_url)\ndef crawl(url):\n\t#crawl page\n\tpass\n\n```\n原理就说得差不多了，要想对布隆过滤器有更深的认识，还需要更多的实战。多写，多思\n考。 Enjoy Python, Enjoy Crawler :)\n# 参考\n+ [https://llimllib.github.io/bloomfilter-tutorial/](https://llimllib.github.io/bloomfilter-tutorial/)\n+ [https://en.wikipedia.org/wiki/Bloom_filter](https://llimllib.github.io/bloomfilter-tutorial/)\n	<h1>爬虫去重</h1>\n<p>笔者最近思考如何编写高效的爬虫; 而在编写高效爬虫的时候，有一个必需解决的问题就是：\nurl 的去重，即如何判别 url 是否已经被爬取，如果被爬取，那就不要重复爬取，不然的\n话，爬虫会重复爬取已经爬取过的页面，不但重复工作，还占用了大量的带宽和CPU资源，\n造成爬虫的效率低下。所以在编写爬虫的时候，对于提高爬虫的效率，如何实现去重就显得\n尤为关键。一般如果需要爬取的网站不是非常庞大的话，使用Python 内置的 set 就可以实\n现去重了，但是使用 set 内存利用率不高，此外对于那些不像Python 那样用 hash 实现的\nset 而言，时间复杂度是 <code>log(N)</code>,实在难说高效。</p>\n<h1>Bloom Filter</h1>\n<p>那么如何实现高效的去重呢？ 笔者查阅资料之后得知：使用布隆过滤器 (Bloom\nFilter). 布隆过滤器可以用于快速检索一个元素是否在一个集合中。布隆过滤器实际上\n是一个很长的二进制向量和一系列随机映射函数（Hash函数）。而一般的判断一个元素是\n否在一个集合里面的做法是：用需要判断的元素和集合中的元素进行比较，一般的数据结\n构，例如链表，树，都是这么实现的。缺点是：随着集合元素的增多，需要比较的元素也\n增多，检索速度就越来越慢。而使用布隆过滤器判重可以实现常数级的时间复杂度(检索\n时间不随元素增长而增加).那么布隆过滤器又是怎样实现的呢</p>\n<h2>布隆过滤器实现原理</h2>\n<p>一个Bloom Filter是基于一个m位的位向量（<code>Bit Vector</code>），这些位向量的初始值为0,并\n且有一系列的 hash 函数，hash 函数值域为1-m.在下面例子中，是15位的位向量，初始\n值为0以空白表示，为1以颜色填充</p>\n<p><img src="https://imgur.com/EJ6CTTe.jpg" alt=""></p>\n<p>现在有两个简单的 hash 函数：fnv,murmur.现在我输入一个字符串 &quot;whatever&quot; ,然后\n分别使用两个 hash 函数对 &quot;whatever&quot; 进行散列计算并且映射到上面的位向量。</p>\n<p><img src="https://imgur.com/np1MtvA.jpg" alt=""></p>\n<p>可知，使用 fnv 函数计算出的 hash 值是11,使用 murmur 函数计算出的 hash 值\n是4. 然后映射到位向量上：</p>\n<p><img src="https://imgur.com/98EANOh.jpg" alt=""></p>\n<p>如果下一次，笔者要判断 <code>whatever</code> 是否在字符串中，只需使用 <code>fnv</code> 和 <code>murmur</code> 两个\nhash 函数对 <code>whatever</code> 进行散列值计算，然后与位向量做 &quot;与运算&quot;，如果结果为0,\n那么说明 <code>whatever</code> 是不在集合中的，因为同样的元素使用同一个 hash 函数产生的\n值每次都是相同的，不相同就说明不是同一个元素。但是如果 &quot;与运算&quot; 的结果为1,是\n否可以说明 <code>whatever</code> 就在集合中呢？其实上是不能100% 确定的，因为 hash 函数存\n在散列冲突现象 (即两个散列值相同，但两个输入值是不同的),所以布隆过滤器只能说&quot;\n我可以说这个元素我在集合中是看见过滴，只是我有一定的不确定性&quot;.当你在分配的内\n存足够大之后，不确定性会变得很小很小。你可以看到布隆过滤器可以有效利用内存实\n现常数级的判重任务，但是鱼和熊掌不可得兼，付出的代价就是一定的误判 (机率很小),所\n以本质上，布隆过滤器是 &quot;概率数据结构 (<code>probabilistic data structure</code>)&quot;.这个就是\n布隆过滤器的基本原理。当然，位向量不会只是15位，hash 函数也不会仅是两个简单的\n函数.这只是简化枝节，为了清晰解述原理而已。</p>\n<h1>Python BloomFilter</h1>\n<p>算法都是为了实际问题服务的，又回到爬虫这个话题上。在了解布隆过滤器原理之后，可\n以很容易地实现自己的布隆过滤器，但是想要实现一个高效健壮的布隆过滤器就需要比较\n多的功夫了，因为需要考虑的问题略多。幸好，得益Python 强大的社区，已经有<a href="https://axiak.github.io/pybloomfiltermmap/">PythonBloomFilter</a>\n的库。一个文档中的简单例子，判断 <code>apple</code> 是否在布隆过滤器中：</p>\n<pre><code>from pybloomfilter import BloomFilter\n\nbf = BloomFilter(10000000, 0.01, 'filter.bloom')\n\nwith open(&quot;/usr/share/dict/words&quot;) as f:\n\tfor word in f:\n\t\tbf.add(word.rstrip())\n\nprint 'apple' in bf\n</code></pre>\n<p>结果为 <code>True</code></p>\n<h1>小结</h1>\n<p>一个简单的爬虫高效去重实现：</p>\n<pre><code>from pybloomfilter import BloomFilter\ncrawled_urls = BloomFilter(10000000, 0.01, 'filter.bloom')\ndef filter(new_url):\n\tif new_url not in crawled_urls: #new url\n\t\tcrawled_urls.add(new_url)\n\t\tcrawl(new_url)\ndef crawl(url):\n\t#crawl page\n\tpass\n\n</code></pre>\n<p>原理就说得差不多了，要想对布隆过滤器有更深的认识，还需要更多的实战。多写，多思\n考。 Enjoy Python, Enjoy Crawler :)</p>\n<h1>参考</h1>\n<ul>\n<li><a href="https://llimllib.github.io/bloomfilter-tutorial/">https://llimllib.github.io/bloomfilter-tutorial/</a></li>\n<li><a href="https://llimllib.github.io/bloomfilter-tutorial/">https://en.wikipedia.org/wiki/Bloom_filter</a></li>\n</ul>\n	2019-05-28 21:53:28	2019-05-28 21:53:28	2	1	t	Bloomfilter	t
9	Diff	diff	如果你有使用过git,那么你一定不会对diff 陌生，因为对你源文件和修改后的文件进行比较\n的就是 `diff` 这个大名鼎鼎的家伙了。多年以来， `diff` 都一直是非常重要的工具，上古大神\n都是使用 `diff` 和 `patch` 对程序进行差分和打补丁滴(现在有git了，但是diff 同样\n发挥着重要作用)\n# 语法 \n  diff 的语法如下\n```\n    diff [OPTION].... file1 file2\n```\n  *OPTION* 指不同的选项参数，file1,file2 是文本文件的名字，如果比较的两个文件相同\n  diff 将不输出任何东西。如果两个文件有差异，diff 会显示一系列的指示，让你可以把\n  第一个文件修改为与第二个文件一致\n# 用法\n## 用法1\n   现在有两个文件，分别保存着不同的地址。\n   address1 包含：\n```\n guangdong \n shanghai \n beijing\n chengdu\n```\n   address2 包含：\n```\n guangdong \n shanghai \n beijin\n chengdu\n```\n   你可以注意到两个文件的区别就是第三行的 *beijing*. 然后运行 `diff`\n```\n     diff address1 address2\n```\n   输出结果：\n```\n 3c3\n < beijing\n ---\n > beijin\n```\n   似乎有点难以理解，输出结果描述了什么呢？其实diff 是在指导如何修改不同的文件使之一致\n   `<` 后接的是文件1中与文件2不同的部分， `>` 后接的是文件2中与文件1不同的部分\n   diff 的输出使用3个不同的单字符指导：a(append,追加),c(change,修改),d(delete,删除)\n   在上面的例子，只是看到一个 *c*,意味着，如果想把 *address1* 修改成 *address2*\n   只需将 *address1* 的第三行修改成 *address2* 的第三行\n## 用法2\n   现在把 *address2* 的最后一行删除，看看运行 *diff* 结果如何：\n   address1 包含：\n```\n guangdong \n shanghai \n beijing\n chengdu\n```\n   address2 包含：\n```\n guangdong \n shanghai \n beijing\n```\n```\n     diff address1 address2\n```\n   输出结果：\n```\n 4d3\n < chengdu\n```\n   在该例子中，为了将 *address1* 变成 *address2* 只需删除 *address1* 的第四行\n## 用法3 \n   现在把 *address1* 的最后一行删除，看看运行 *diff* 结果如何：\n   address1 包含：\n```\n guangdong \n shanghai \n beijing\n```\n   address2 包含：\n```\n guangdong\n shanghai\n beijing\n chengdu\n```\n```\n     diff address1 address2\n```\n   输出结果：\n```\n3a4\n> chengdu\n```\n   想将第一个文件转换成第二个文件，只需在第一个文件追加第二个文件的第四行(即在\n   第一个文件的第 `3` 行之后追加第二个文件的第 `4` 行)\n# diff 选项\n  因为diff 是一个相当强大也是一个相当复杂的命令，所以我没办法将所有的用法一一道\n  尽所以笔者将比较常用的选项列举出来\n  + `-b`:忽略制表符(不忽略所有的空白符，指忽略空白符数量的差异),例如下面的两行是相同的\n```\na    a\na a\n```\n  + `-B(blank lines)`:忽略所有的空白行\n  + `-c(context)`:以上下文的形式显示差异内容，对比默认输出更加容易理解(但是也更加繁杂)\n  + `-q(quiet)`: diff 静默设置，即只有文件file1和file2有差异，diff 才会显示内容 \n  + `-w(whitespace)`:忽略所有的空白符\n  + `-u(unified output)`: 上下文形式显示的改进，不会输出重复行\n  + `-y`:将文件分成两列或多列并排进行输出(非常直观，但是输出很繁杂)\n    \n  还是那句老话，更多的用法就需要：\n```\n    man diff\n```\n  \n    \n	<p>如果你有使用过git,那么你一定不会对diff 陌生，因为对你源文件和修改后的文件进行比较\n的就是 <code>diff</code> 这个大名鼎鼎的家伙了。多年以来， <code>diff</code> 都一直是非常重要的工具，上古大神\n都是使用 <code>diff</code> 和 <code>patch</code> 对程序进行差分和打补丁滴(现在有git了，但是diff 同样\n发挥着重要作用)</p>\n<h1>语法</h1>\n<p>diff 的语法如下</p>\n<pre><code>    diff [OPTION].... file1 file2\n</code></pre>\n<p><em>OPTION</em> 指不同的选项参数，file1,file2 是文本文件的名字，如果比较的两个文件相同\ndiff 将不输出任何东西。如果两个文件有差异，diff 会显示一系列的指示，让你可以把\n第一个文件修改为与第二个文件一致</p>\n<h1>用法</h1>\n<h2>用法1</h2>\n<p>现在有两个文件，分别保存着不同的地址。\naddress1 包含：</p>\n<pre><code> guangdong \n shanghai \n beijing\n chengdu\n</code></pre>\n<p>address2 包含：</p>\n<pre><code> guangdong \n shanghai \n beijin\n chengdu\n</code></pre>\n<p>你可以注意到两个文件的区别就是第三行的 <em>beijing</em>. 然后运行 <code>diff</code></p>\n<pre><code>     diff address1 address2\n</code></pre>\n<p>输出结果：</p>\n<pre><code> 3c3\n &lt; beijing\n ---\n &gt; beijin\n</code></pre>\n<p>似乎有点难以理解，输出结果描述了什么呢？其实diff 是在指导如何修改不同的文件使之一致\n<code>&lt;</code> 后接的是文件1中与文件2不同的部分， <code>&gt;</code> 后接的是文件2中与文件1不同的部分\ndiff 的输出使用3个不同的单字符指导：a(append,追加),c(change,修改),d(delete,删除)\n在上面的例子，只是看到一个 <em>c</em>,意味着，如果想把 <em>address1</em> 修改成 <em>address2</em>\n只需将 <em>address1</em> 的第三行修改成 <em>address2</em> 的第三行</p>\n<h2>用法2</h2>\n<p>现在把 <em>address2</em> 的最后一行删除，看看运行 <em>diff</em> 结果如何：\naddress1 包含：</p>\n<pre><code> guangdong \n shanghai \n beijing\n chengdu\n</code></pre>\n<p>address2 包含：</p>\n<pre><code> guangdong \n shanghai \n beijing\n</code></pre>\n<pre><code>     diff address1 address2\n</code></pre>\n<p>输出结果：</p>\n<pre><code> 4d3\n &lt; chengdu\n</code></pre>\n<p>在该例子中，为了将 <em>address1</em> 变成 <em>address2</em> 只需删除 <em>address1</em> 的第四行</p>\n<h2>用法3</h2>\n<p>现在把 <em>address1</em> 的最后一行删除，看看运行 <em>diff</em> 结果如何：\naddress1 包含：</p>\n<pre><code> guangdong \n shanghai \n beijing\n</code></pre>\n<p>address2 包含：</p>\n<pre><code> guangdong\n shanghai\n beijing\n chengdu\n</code></pre>\n<pre><code>     diff address1 address2\n</code></pre>\n<p>输出结果：</p>\n<pre><code>3a4\n&gt; chengdu\n</code></pre>\n<p>想将第一个文件转换成第二个文件，只需在第一个文件追加第二个文件的第四行(即在\n第一个文件的第 <code>3</code> 行之后追加第二个文件的第 <code>4</code> 行)</p>\n<h1>diff 选项</h1>\n<p>因为diff 是一个相当强大也是一个相当复杂的命令，所以我没办法将所有的用法一一道\n尽所以笔者将比较常用的选项列举出来</p>\n<ul>\n<li><code>-b</code>:忽略制表符(不忽略所有的空白符，指忽略空白符数量的差异),例如下面的两行是相同的</li>\n</ul>\n<pre><code>a    a\na a\n</code></pre>\n<ul>\n<li><code>-B(blank lines)</code>:忽略所有的空白行</li>\n<li><code>-c(context)</code>:以上下文的形式显示差异内容，对比默认输出更加容易理解(但是也更加繁杂)</li>\n<li><code>-q(quiet)</code>: diff 静默设置，即只有文件file1和file2有差异，diff 才会显示内容</li>\n<li><code>-w(whitespace)</code>:忽略所有的空白符</li>\n<li><code>-u(unified output)</code>: 上下文形式显示的改进，不会输出重复行</li>\n<li><code>-y</code>:将文件分成两列或多列并排进行输出(非常直观，但是输出很繁杂)</li>\n</ul>\n<p>还是那句老话，更多的用法就需要：</p>\n<pre><code>    man diff\n</code></pre>\n	2019-05-29 10:06:23	2019-05-29 10:06:23	2	1	t	Diff	t
2	浅谈Java公平锁与内存模型	如题	浅谈Java公平锁与内存模型\n## 前言\n春天来了，春招还会远么? 又到了春招的季节，随之而来的是各种的面试题。今天就看到组内大佬面试实习生的一道Java题目:\n\n>编写一个程序，开启 3 个线程A,B,C，这三个线程的输出分别为 A、B、C，每个线程将自己的 输出在屏幕上打印 10 遍，要求输出的结果必须按顺序显示。如：ABCABCABC....\n\n## 经过\n出于好奇的心态，我花了点时间来尝试解决这个问题, 主要的难点是让线程顺序地如何顺序地输出，线程之间如何交换。很快就按着思路写出了一个版本，用Lock 来控制线程的顺序，A,B,C线程依次启动，因为A 线程先启动，所以A线程会最先拿到锁，B,C阻塞；但是A输出完字符串，释放锁，B 线程获得锁，C,A线程阻塞; 依此循环: \n```java\npublic void Test(){\n    private static Integer index = 0;\n\n    Lock lock = new ReentrantLock();\n\t\n\t@Test\n    public void testLock(){\n        Thread threadA = work(i -> i % 3 == 0, () -> System.out.println("A"));\n        Thread threadB = work(i -> i % 3 == 1, () -> System.out.println("B"));\n        Thread threadC = work(i -> i % 3 == 2, () -> System.out.println("C"));\n        threadA.start();\n        threadB.start();\n        threadC.start();\n    }\n\n    private Thread work(Predicate<Integer> condition, Runnable function) {\n        return new Thread(() -> {\n            while (index < 30) {\n                lock.lock();\n                if (condition.test(index)) {\n                    function.run();\n                    index++;\n                }\n                lock.unlock();\n            }\n        });\n    }\n}\n```\n输入结果如我预期那般，ABCABC交替输出，也成功输出了10次，奇怪的是A,B却多输出了一次？\n![](https://imgur.com/3lolbwK.png)\n\n为什么会多输出一次，不是应该恰好是输出30次么, 为什么会多输出一次A,B 真的百思不得其解. 所以我把`index` 也打印出来查看, 结果相当奇怪:\n```java\n...\nfunction.run();\nSystem.out.println(index);\n....\n```\n\n为什么A 会是30, B会是31, 不是有(index.intvalue<30) 的条件判断么, 为什么还会出现这样的数据？灵异事件?\n![](https://imgur.com/fhurKt5.png)\n## 解惑\n灵异事件自然是不存在的，仔细分析了一番代码之后，发现了问题：\n```java\nwhile (index.intValue() < 30) {  // 1\n    lock.lock(); // 2\n    if (condition.test(index.intValue())) {\n        function.run();\n        index++;\n    }\n    lock.unlock();\n}\n```\n将1，2行的操作做了这三件事，如下:\n1. **线程读取index的值**\n2. 比较index的值是否大于30\n3. 如果小于30, 尝试获取锁\n\n换言之，当index=29时，线程C持有锁，**但是锁只能阻止线程A,线程B修改index的值，并不能阻止线程A, 线程B在获取锁之前读取index的值，所以线程A读取index=29, 并把值保持到线程的内部**，如下图:\n\n![](https://imgur.com/tI8NTgO.png)\n\n**当线程C执行完，还没释放锁的时候，线程A的index值为29；当线程C释放锁，线程A获取锁，进入同步块的时候，因为 [Java内存模型有内存可见性的要求](https://en.wikipedia.org/wiki/Memory_barrier#Multithreaded_programming_and_memory_visibility), 兼之Lock的实现类实现了[内存可见](https://stackoverflow.com/questions/12429818/does-explicit-lock-automatically-provide-memory-visibility)，所以线程A的index值会变成30**, 这就解析了为什么线程A index=30的时候能跳过`(index.intValue<30)`的判断条件，因为执行这个判断条件的时候线程A index=29, 进入同步块之后变成了30:\n\n![](https://imgur.com/xaA4Q0y.png)\n\n把问题剖析清楚之后，解决方案就呼之欲出了:\n```java\nwhile (index.intValue() < 30) {  // 1\n    lock.lock(); // 2\n\tif(index>=30){\n\t\tcontinue;\n\t}\n    if (condition.test(index.intValue())) {\n        function.run();\n        index++;\n    }\n    lock.unlock();\n}\n\n```\n\n这种解决方法不禁让我想起单例模式里面的双重校验:\n```java\npublic static Singleton getSingleton() {\n    if (instance == null) {                         //Single Checked\n        synchronized (Singleton.class) {\n            if (instance == null) {                 //Double Checked\n                instance = new Singleton();\n            }\n        }\n    }\n    return instance ;\n}\n```\n只是当时并不清楚Double Checked的作用，究竟解决了什么问题？只是知道不加这条语句就会造成初始化多个示例，的确是需要**知其然知其所以然**.\n## 公平锁问题\n前文说到，\n> 这个程序是用Lock 来控制线程的顺序，A,B,C线程依次启动，因为A 线程先启动，所以A线程会最先拿到锁，B,C阻塞；但是A输出完字符串，释放锁，B 线程获得锁，C,A线程阻塞; 依此循环。\n\n粗看似乎没什么问题, 但是这里是存在着一个问题: 当线程A 释放锁的时候，获取锁的是否一定是线程B, 而不是线程C, 线程C是否能够"插队"抢占锁? 这个就涉及到了公平锁和非公平锁的定义了:\n公平锁: 线程C不能抢占，只能排队等待线程B 获取并释放锁\n非公平锁：线程C能抢占，抢到锁之后线程B只能继续等(有点惨!)\n\n而ReentrantLock默认恰好是非公平锁, 查看源码可知:\n```java\n/**\n     * Creates an instance of {@code ReentrantLock}.\n     * This is equivalent to using {@code ReentrantLock(false)}.\n     */\n    public ReentrantLock() {\n        sync = new NonfairSync();\n    }\n```\n因此为了规避非公平锁抢占的问题, 上述的代码在同步块增加了判断条件:\n```java\n if (condition.test(index.intValue())) {\n   ....\n }\n```\n只有符合条件的线程才能进行操作，否则就是线程自旋.(但是加锁+自旋实现起来，效率不会太高效!)\n## 小结\n写一条面试题的答案都写得是问题多多的，不禁令人沮丧，说明自己对Java的并发模型理解还有很大的提高。不过在排查问题的过程中，通过实践有体感地理解了Java的内存模型，发现Java内存模型并不是那么地曲高和寡，在日常的开发中也是很常见的. 费了一番工夫排查之后，终究是有新的收获的\n![Imgs](https://cdn.nlark.com/yuque/0/2019/png/359453/1559036764952-d3ee9b83-c8a5-45f0-b6f1-04d8d3c01896.png)\n	<p>浅谈Java公平锁与内存模型</p>\n<h2>前言</h2>\n<p>春天来了，春招还会远么? 又到了春招的季节，随之而来的是各种的面试题。今天就看到组内大佬面试实习生的一道Java题目:</p>\n<blockquote>\n<p>编写一个程序，开启 3 个线程A,B,C，这三个线程的输出分别为 A、B、C，每个线程将自己的 输出在屏幕上打印 10 遍，要求输出的结果必须按顺序显示。如：ABCABCABC....</p>\n</blockquote>\n<h2>经过</h2>\n<p>出于好奇的心态，我花了点时间来尝试解决这个问题, 主要的难点是让线程顺序地如何顺序地输出，线程之间如何交换。很快就按着思路写出了一个版本，用Lock 来控制线程的顺序，A,B,C线程依次启动，因为A 线程先启动，所以A线程会最先拿到锁，B,C阻塞；但是A输出完字符串，释放锁，B 线程获得锁，C,A线程阻塞; 依此循环:</p>\n<pre><code class="language-java">public void Test(){\n    private static Integer index = 0;\n\n    Lock lock = new ReentrantLock();\n\t\n\t@Test\n    public void testLock(){\n        Thread threadA = work(i -&gt; i % 3 == 0, () -&gt; System.out.println(&quot;A&quot;));\n        Thread threadB = work(i -&gt; i % 3 == 1, () -&gt; System.out.println(&quot;B&quot;));\n        Thread threadC = work(i -&gt; i % 3 == 2, () -&gt; System.out.println(&quot;C&quot;));\n        threadA.start();\n        threadB.start();\n        threadC.start();\n    }\n\n    private Thread work(Predicate&lt;Integer&gt; condition, Runnable function) {\n        return new Thread(() -&gt; {\n            while (index &lt; 30) {\n                lock.lock();\n                if (condition.test(index)) {\n                    function.run();\n                    index++;\n                }\n                lock.unlock();\n            }\n        });\n    }\n}\n</code></pre>\n<p>输入结果如我预期那般，ABCABC交替输出，也成功输出了10次，奇怪的是A,B却多输出了一次？\n<img src="https://imgur.com/3lolbwK.png" alt=""></p>\n<p>为什么会多输出一次，不是应该恰好是输出30次么, 为什么会多输出一次A,B 真的百思不得其解. 所以我把<code>index</code> 也打印出来查看, 结果相当奇怪:</p>\n<pre><code class="language-java">...\nfunction.run();\nSystem.out.println(index);\n....\n</code></pre>\n<p>为什么A 会是30, B会是31, 不是有(index.intvalue&lt;30) 的条件判断么, 为什么还会出现这样的数据？灵异事件?\n<img src="https://imgur.com/fhurKt5.png" alt=""></p>\n<h2>解惑</h2>\n<p>灵异事件自然是不存在的，仔细分析了一番代码之后，发现了问题：</p>\n<pre><code class="language-java">while (index.intValue() &lt; 30) {  // 1\n    lock.lock(); // 2\n    if (condition.test(index.intValue())) {\n        function.run();\n        index++;\n    }\n    lock.unlock();\n}\n</code></pre>\n<p>将1，2行的操作做了这三件事，如下:</p>\n<ol>\n<li><strong>线程读取index的值</strong></li>\n<li>比较index的值是否大于30</li>\n<li>如果小于30, 尝试获取锁</li>\n</ol>\n<p>换言之，当index=29时，线程C持有锁，<strong>但是锁只能阻止线程A,线程B修改index的值，并不能阻止线程A, 线程B在获取锁之前读取index的值，所以线程A读取index=29, 并把值保持到线程的内部</strong>，如下图:</p>\n<p><img src="https://imgur.com/tI8NTgO.png" alt=""></p>\n<p><strong>当线程C执行完，还没释放锁的时候，线程A的index值为29；当线程C释放锁，线程A获取锁，进入同步块的时候，因为 <a href="https://en.wikipedia.org/wiki/Memory_barrier#Multithreaded_programming_and_memory_visibility">Java内存模型有内存可见性的要求</a>, 兼之Lock的实现类实现了<a href="https://stackoverflow.com/questions/12429818/does-explicit-lock-automatically-provide-memory-visibility">内存可见</a>，所以线程A的index值会变成30</strong>, 这就解析了为什么线程A index=30的时候能跳过<code>(index.intValue&lt;30)</code>的判断条件，因为执行这个判断条件的时候线程A index=29, 进入同步块之后变成了30:</p>\n<p><img src="https://imgur.com/xaA4Q0y.png" alt=""></p>\n<p>把问题剖析清楚之后，解决方案就呼之欲出了:</p>\n<pre><code class="language-java">while (index.intValue() &lt; 30) {  // 1\n    lock.lock(); // 2\n\tif(index&gt;=30){\n\t\tcontinue;\n\t}\n    if (condition.test(index.intValue())) {\n        function.run();\n        index++;\n    }\n    lock.unlock();\n}\n\n</code></pre>\n<p>这种解决方法不禁让我想起单例模式里面的双重校验:</p>\n<pre><code class="language-java">public static Singleton getSingleton() {\n    if (instance == null) {                         //Single Checked\n        synchronized (Singleton.class) {\n            if (instance == null) {                 //Double Checked\n                instance = new Singleton();\n            }\n        }\n    }\n    return instance ;\n}\n</code></pre>\n<p>只是当时并不清楚Double Checked的作用，究竟解决了什么问题？只是知道不加这条语句就会造成初始化多个示例，的确是需要<strong>知其然知其所以然</strong>.</p>\n<h2>公平锁问题</h2>\n<p>前文说到，</p>\n<blockquote>\n<p>这个程序是用Lock 来控制线程的顺序，A,B,C线程依次启动，因为A 线程先启动，所以A线程会最先拿到锁，B,C阻塞；但是A输出完字符串，释放锁，B 线程获得锁，C,A线程阻塞; 依此循环。</p>\n</blockquote>\n<p>粗看似乎没什么问题, 但是这里是存在着一个问题: 当线程A 释放锁的时候，获取锁的是否一定是线程B, 而不是线程C, 线程C是否能够&quot;插队&quot;抢占锁? 这个就涉及到了公平锁和非公平锁的定义了:\n公平锁: 线程C不能抢占，只能排队等待线程B 获取并释放锁\n非公平锁：线程C能抢占，抢到锁之后线程B只能继续等(有点惨!)</p>\n<p>而ReentrantLock默认恰好是非公平锁, 查看源码可知:</p>\n<pre><code class="language-java">/**\n     * Creates an instance of {@code ReentrantLock}.\n     * This is equivalent to using {@code ReentrantLock(false)}.\n     */\n    public ReentrantLock() {\n        sync = new NonfairSync();\n    }\n</code></pre>\n<p>因此为了规避非公平锁抢占的问题, 上述的代码在同步块增加了判断条件:</p>\n<pre><code class="language-java"> if (condition.test(index.intValue())) {\n   ....\n }\n</code></pre>\n<p>只有符合条件的线程才能进行操作，否则就是线程自旋.(但是加锁+自旋实现起来，效率不会太高效!)</p>\n<h2>小结</h2>\n<p>写一条面试题的答案都写得是问题多多的，不禁令人沮丧，说明自己对Java的并发模型理解还有很大的提高。不过在排查问题的过程中，通过实践有体感地理解了Java的内存模型，发现Java内存模型并不是那么地曲高和寡，在日常的开发中也是很常见的. 费了一番工夫排查之后，终究是有新的收获的\n<img src="https://cdn.nlark.com/yuque/0/2019/png/359453/1559036764952-d3ee9b83-c8a5-45f0-b6f1-04d8d3c01896.png" alt="Imgs"></p>\n	2019-05-28 17:05:59	2019-05-28 17:47:16	2	24	t	浅谈Java公平锁与内存模型	t
3	优化Emacs启动速度	Tips	之前在 [xkcd](https://xkcd.com/) 上看到个笑话：\n<A:在等待emacs 加载的时间里，你会干什么？\n<B:打开Vim,修改代码，保存，退出\n\n有时候，经常看到社区里面有人吐嘈 Emacs 什么都好，就是启动时间太长了.虽说 Emacs\n加载速度实在有点慢，但是 还是存在一些技巧来缩短加载时间的\n## 技巧1\n  在你的 `.emacs` 或者相应的初始化文件里面添加如下代码\n  ```\n    # Increase the garbage collection threshold to 128 MB to ease startup\n    (setq gc-cons-threshold (* 128 1024 1024 ))\n    # your configuration code \n    # ......\n    # Garbage collector-decrease threshold to 5 MB\n    (add-hook 'after-init-hook (lambda () (setq gc-cons-threshold (* 5 1024 1024))))\n    # init.el ends here\n\t```\n  `gc-cons-threshold` 指定了emacs 进行垃圾回收的阀值，默认值是 `800000byte`,实在\n  是太小了，所以Emacs 会在启动期间进行非常多次的垃圾回收，启动时间自然长了，(题外话，其实，Lisp 是第一种实现了垃圾回收机制算法的编程语言)。在加载\n  完以后，再把 `gc-cons-threshold` 的值调低，当然，如果你的内存很大，也可以不改回来\n## 技巧2\n  `(let((file-name-hander-alist nil))init.file)` 包裹住(wrap)你的初始化文件，即：\n  ```emacs-lisp\n    (setq gc-cons-threshold (* 500 1024 1024))\n    (let ((file-name-handler-alist nil))\n      ...\n\n      ,** your config goes here **\n\n      ...\n      )\n    (add-hook 'after-init-hook (lambda () (setq gc-cons-threshold (* 5 1024 1024))))\n    (provide 'init)\n    ;;; init.el ends here\n```\n  因为 `file-name-handler-alist` 的默认值是一些正则表达式，也就是说Emacs 在启动\n  过程中加载el和elc 文件都会将文件名和正则表达式进行匹配\n## 技巧3\nEmacs lisp 有一项 `auto-load` 的技术，类似延迟加载，合理运用延迟，让笔者的 Emacs\n  启动加载时间减少一半，因为笔者用 `use-package` 了这个插件,而 `use-package` 又集\n  成了延迟加载的功能，所以笔者就直接拿自己的代码举例了\n### :after\n```emacs-lisp\n     ;;; Export to twitter bootstrap\n     (use-package ox-twbs\n       :after org\n       :ensure ox-twbs\n       )\n```\n   `:after` 关键字的作用基本跟 `with-eval-after-load` 的作用是相同的，所以笔者所\n   有类似的org-mode 插件包都会在org-mode 加载以后才会加载\n### :commands\n```emacs-lisp\n     (use-package avy\n       :commands (avy-goto-char avy-goto-line)\n       :ensure t)\n```\n   这里就直接贴上 [use-package](https://github.com/jwiegley/use-package)文档的说明了\n   \n\t<When you use the :commands keyword, it creates autoloads for those commands\n   <and defers loading of the module until they are used\n   也就是 `:commands` 关键字就创建了后面所接的命令的 `autoloads` 机制了\n### :bind :mode \n```emacs-lisp\n     (use-package hi-lock\n       :bind (("M-o l" . highlight-lines-matching-regexp)\n              ("M-o r" . highlight-regexp)\n              ("M-o w" . highlight-phrase)))\n\n     (use-package vue-mode\n       :ensure t\n       :mode ("\\\\.vue\\\\'" . vue-mode)\n       :config (progn\n                 (setq mmm-submode-decoration-level 0)\n                 ))\n```\n   附上文档说明\n   <In almost all cases you don't need to manually specify :defer t. This is implied\n   <whenever :bind or :mode or :interpreter is used\n   也就是说，当你使用了 `:bind` 或者 `:mode` 关键字的时候，不用明确指定 `:defer`\n   也可以实现延迟加载机制。当然你也可以，直接使用 `:defer` 关键字来指定延迟加载\n   不过前提是，你要明确它加载的时机\n   <Typically, you only need to specify :defer if you know for a fact that some\n   <other package will do something to cause your package to load at the appropriate\n   <time, and thus you would like to defer loading even though use-package isn't\n   <creating any autoloads for you.\n   贴上笔者自己的代码，可以更加清晰\n   ```emacs-lisp\n     (use-package anaconda-mode\n       :defer t\n       :ensure t\n       :init(progn\n              (add-hook 'python-mode-hook 'anaconda-mode)\n              (add-hook 'python-mode-hook 'anaconda-eldoc-mode)\n              ))\n\t\t\t  ```\n   这样 `anaconda-mode` 就会在 `python-mode` 加载以后被加载\n   \n   Enjoy Emacs :)\n\n\n\n之前在 [xkcd](https://xkcd.com/) 上看到个笑话：\n>A:在等待emacs 加载的时间里，你会干什么？\nB:打开Vim,修改代码，保存，退出\n\n有时候，经常看到社区里面有人吐嘈 Emacs 什么都好，就是启动时间太长了.虽说 Emacs\n加载速度实在有点慢，但是 还是存在一些技巧来缩短加载时间的\n## 技巧1\n  在你的 `.emacs` 或者相应的初始化文件里面添加如下代码\n\t```\n    # Increase the garbage collection threshold to 128 MB to ease startup\n    (setq gc-cons-threshold (* 128 1024 1024 ))\n    # your configuration code \n    # ......\n    # Garbage collector-decrease threshold to 5 MB\n    (add-hook 'after-init-hook (lambda () (setq gc-cons-threshold (* 5 1024 1024))))\n    # init.el ends here\n```\n\t\n  `gc-cons-threshold` 指定了emacs 进行垃圾回收的阀值，默认值是 `800000byte`,实在\n  是太小了，所以Emacs 会在启动期间进行非常多次的垃圾回收，启动时间自然长了，(题外话，其实，Lisp 是第一种实现了垃圾回收机制算法的编程语言)。在加载\n  完以后，再把 `gc-cons-threshold` 的值调低，当然，如果你的内存很大，也可以不改回来\n## 技巧2\n  `(let((file-name-hander-alist nil))init.file)` 包裹住(wrap)你的初始化文件，即：\n  ```emacs-lisp\n    (setq gc-cons-threshold (* 500 1024 1024))\n    (let ((file-name-handler-alist nil))\n      ...\n\n      ,** your config goes here **\n\n      ...\n      )\n    (add-hook 'after-init-hook (lambda () (setq gc-cons-threshold (* 5 1024 1024))))\n    (provide 'init)\n    ;;; init.el ends here\n```\n  因为 `file-name-handler-alist` 的默认值是一些正则表达式，也就是说Emacs 在启动\n  过程中加载el和elc 文件都会将文件名和正则表达式进行匹配\n## 技巧3\nEmacs lisp 有一项 `auto-load` 的技术，类似延迟加载，合理运用延迟，让笔者的 Emacs\n  启动加载时间减少一半，因为笔者用 `use-package` 了这个插件,而 `use-package` 又集\n  成了延迟加载的功能，所以笔者就直接拿自己的代码举例了\n### :after\n```emacs-lisp\n     ;;; Export to twitter bootstrap\n     (use-package ox-twbs\n       :after org\n       :ensure ox-twbs\n       )\n```\n   `:after` 关键字的作用基本跟 `with-eval-after-load` 的作用是相同的，所以笔者所\n   有类似的org-mode 插件包都会在org-mode 加载以后才会加载\n### :commands\n\n```emacs-lisp\n     (use-package avy\n       :commands (avy-goto-char avy-goto-line)\n       :ensure t)\n```\n   这里就直接贴上 [use-package](https://github.com/jwiegley/use-package)文档的说明了:\n>When you use the :commands keyword, it creates autoloads for those commands\n    and defers loading of the module until they are used\n\t\t\n   也就是 `:commands` 关键字就创建了后面所接的命令的 `autoloads` 机制了\n### :bind :mode \n```emacs-lisp\n     (use-package hi-lock\n       :bind (("M-o l" . highlight-lines-matching-regexp)\n              ("M-o r" . highlight-regexp)\n              ("M-o w" . highlight-phrase)))\n\n     (use-package vue-mode\n       :ensure t\n       :mode ("\\\\.vue\\\\'" . vue-mode)\n       :config (progn\n                 (setq mmm-submode-decoration-level 0)\n                 ))\n```\n   附上文档说明\n>In almost all cases you don't need to manually specify `:defer t`. This is implied\n   whenever `:bind` or `:mode` or `:interpreter` is used\n\t \n   也就是说，当你使用了 `:bind` 或者 `:mode` 关键字的时候，不用明确指定 `:defer`\n   也可以实现延迟加载机制。当然你也可以，直接使用 `:defer` 关键字来指定延迟加载\n   不过前提是，你要明确它加载的时机\n>Typically, you only need to specify :defer if you know for a fact that some\n   other package will do something to cause your package to load at the appropriate\n   time, and thus you would like to defer loading even though use-package isn't\n   creating any autoloads for you.\n\t \n   贴上笔者自己的代码，可以更加清晰\n   ```\n     (use-package anaconda-mode\n       :defer t\n       :ensure t\n       :init(progn\n              (add-hook 'python-mode-hook 'anaconda-mode)\n              (add-hook 'python-mode-hook 'anaconda-eldoc-mode)\n              ))\n\t\t\t  ```\n   这样 `anaconda-mode` 就会在 `python-mode` 加载以后被加载\n   \n   Enjoy Emacs :)\n	<p>之前在 <a href="https://xkcd.com/">xkcd</a> 上看到个笑话：\n&lt;A:在等待emacs 加载的时间里，你会干什么？\n&lt;B:打开Vim,修改代码，保存，退出</p>\n<p>有时候，经常看到社区里面有人吐嘈 Emacs 什么都好，就是启动时间太长了.虽说 Emacs\n加载速度实在有点慢，但是 还是存在一些技巧来缩短加载时间的</p>\n<h2>技巧1</h2>\n<p>在你的 <code>.emacs</code> 或者相应的初始化文件里面添加如下代码</p>\n<pre><code>  # Increase the garbage collection threshold to 128 MB to ease startup\n  (setq gc-cons-threshold (* 128 1024 1024 ))\n  # your configuration code \n  # ......\n  # Garbage collector-decrease threshold to 5 MB\n  (add-hook 'after-init-hook (lambda () (setq gc-cons-threshold (* 5 1024 1024))))\n  # init.el ends here\n  ```\n`gc-cons-threshold` 指定了emacs 进行垃圾回收的阀值，默认值是 `800000byte`,实在\n是太小了，所以Emacs 会在启动期间进行非常多次的垃圾回收，启动时间自然长了，(题外话，其实，Lisp 是第一种实现了垃圾回收机制算法的编程语言)。在加载\n完以后，再把 `gc-cons-threshold` 的值调低，当然，如果你的内存很大，也可以不改回来\n## 技巧2\n`(let((file-name-hander-alist nil))init.file)` 包裹住(wrap)你的初始化文件，即：\n```emacs-lisp\n  (setq gc-cons-threshold (* 500 1024 1024))\n  (let ((file-name-handler-alist nil))\n    ...\n\n    ,** your config goes here **\n\n    ...\n    )\n  (add-hook 'after-init-hook (lambda () (setq gc-cons-threshold (* 5 1024 1024))))\n  (provide 'init)\n  ;;; init.el ends here\n</code></pre>\n<p>因为 <code>file-name-handler-alist</code> 的默认值是一些正则表达式，也就是说Emacs 在启动\n过程中加载el和elc 文件都会将文件名和正则表达式进行匹配</p>\n<h2>技巧3</h2>\n<p>Emacs lisp 有一项 <code>auto-load</code> 的技术，类似延迟加载，合理运用延迟，让笔者的 Emacs\n启动加载时间减少一半，因为笔者用 <code>use-package</code> 了这个插件,而 <code>use-package</code> 又集\n成了延迟加载的功能，所以笔者就直接拿自己的代码举例了</p>\n<h3>:after</h3>\n<pre><code class="language-emacs-lisp">     ;;; Export to twitter bootstrap\n     (use-package ox-twbs\n       :after org\n       :ensure ox-twbs\n       )\n</code></pre>\n<p><code>:after</code> 关键字的作用基本跟 <code>with-eval-after-load</code> 的作用是相同的，所以笔者所\n有类似的org-mode 插件包都会在org-mode 加载以后才会加载</p>\n<h3>:commands</h3>\n<pre><code class="language-emacs-lisp">     (use-package avy\n       :commands (avy-goto-char avy-goto-line)\n       :ensure t)\n</code></pre>\n<p>这里就直接贴上 <a href="https://github.com/jwiegley/use-package">use-package</a>文档的说明了</p>\n<pre><code>&lt;When you use the :commands keyword, it creates autoloads for those commands\n</code></pre>\n<p>&lt;and defers loading of the module until they are used\n也就是 <code>:commands</code> 关键字就创建了后面所接的命令的 <code>autoloads</code> 机制了</p>\n<h3>:bind :mode</h3>\n<pre><code class="language-emacs-lisp">     (use-package hi-lock\n       :bind ((&quot;M-o l&quot; . highlight-lines-matching-regexp)\n              (&quot;M-o r&quot; . highlight-regexp)\n              (&quot;M-o w&quot; . highlight-phrase)))\n\n     (use-package vue-mode\n       :ensure t\n       :mode (&quot;\\\\.vue\\\\'&quot; . vue-mode)\n       :config (progn\n                 (setq mmm-submode-decoration-level 0)\n                 ))\n</code></pre>\n<p>附上文档说明\n&lt;In almost all cases you don't need to manually specify :defer t. This is implied\n&lt;whenever :bind or :mode or :interpreter is used\n也就是说，当你使用了 <code>:bind</code> 或者 <code>:mode</code> 关键字的时候，不用明确指定 <code>:defer</code>\n也可以实现延迟加载机制。当然你也可以，直接使用 <code>:defer</code> 关键字来指定延迟加载\n不过前提是，你要明确它加载的时机\n&lt;Typically, you only need to specify :defer if you know for a fact that some\n&lt;other package will do something to cause your package to load at the appropriate\n&lt;time, and thus you would like to defer loading even though use-package isn't\n&lt;creating any autoloads for you.\n贴上笔者自己的代码，可以更加清晰</p>\n<pre><code class="language-emacs-lisp">  (use-package anaconda-mode\n    :defer t\n    :ensure t\n    :init(progn\n           (add-hook 'python-mode-hook 'anaconda-mode)\n           (add-hook 'python-mode-hook 'anaconda-eldoc-mode)\n           ))\n \t\t  ```\n这样 `anaconda-mode` 就会在 `python-mode` 加载以后被加载\n\nEnjoy Emacs :)\n\n\n\n之前在 [xkcd](https://xkcd.com/) 上看到个笑话：\n&gt;A:在等待emacs 加载的时间里，你会干什么？\nB:打开Vim,修改代码，保存，退出\n\n有时候，经常看到社区里面有人吐嘈 Emacs 什么都好，就是启动时间太长了.虽说 Emacs\n加载速度实在有点慢，但是 还是存在一些技巧来缩短加载时间的\n## 技巧1\n在你的 `.emacs` 或者相应的初始化文件里面添加如下代码\n ```\n # Increase the garbage collection threshold to 128 MB to ease startup\n (setq gc-cons-threshold (* 128 1024 1024 ))\n # your configuration code \n # ......\n # Garbage collector-decrease threshold to 5 MB\n (add-hook 'after-init-hook (lambda () (setq gc-cons-threshold (* 5 1024 1024))))\n # init.el ends here\n</code></pre>\n<p><code>gc-cons-threshold</code> 指定了emacs 进行垃圾回收的阀值，默认值是 <code>800000byte</code>,实在\n是太小了，所以Emacs 会在启动期间进行非常多次的垃圾回收，启动时间自然长了，(题外话，其实，Lisp 是第一种实现了垃圾回收机制算法的编程语言)。在加载\n完以后，再把 <code>gc-cons-threshold</code> 的值调低，当然，如果你的内存很大，也可以不改回来</p>\n<h2>技巧2</h2>\n<p><code>(let((file-name-hander-alist nil))init.file)</code> 包裹住(wrap)你的初始化文件，即：</p>\n<pre><code class="language-emacs-lisp">  (setq gc-cons-threshold (* 500 1024 1024))\n  (let ((file-name-handler-alist nil))\n    ...\n\n    ,** your config goes here **\n\n    ...\n    )\n  (add-hook 'after-init-hook (lambda () (setq gc-cons-threshold (* 5 1024 1024))))\n  (provide 'init)\n  ;;; init.el ends here\n</code></pre>\n<p>因为 <code>file-name-handler-alist</code> 的默认值是一些正则表达式，也就是说Emacs 在启动\n过程中加载el和elc 文件都会将文件名和正则表达式进行匹配</p>\n<h2>技巧3</h2>\n<p>Emacs lisp 有一项 <code>auto-load</code> 的技术，类似延迟加载，合理运用延迟，让笔者的 Emacs\n启动加载时间减少一半，因为笔者用 <code>use-package</code> 了这个插件,而 <code>use-package</code> 又集\n成了延迟加载的功能，所以笔者就直接拿自己的代码举例了</p>\n<h3>:after</h3>\n<pre><code class="language-emacs-lisp">     ;;; Export to twitter bootstrap\n     (use-package ox-twbs\n       :after org\n       :ensure ox-twbs\n       )\n</code></pre>\n<p><code>:after</code> 关键字的作用基本跟 <code>with-eval-after-load</code> 的作用是相同的，所以笔者所\n有类似的org-mode 插件包都会在org-mode 加载以后才会加载</p>\n<h3>:commands</h3>\n<pre><code class="language-emacs-lisp">     (use-package avy\n       :commands (avy-goto-char avy-goto-line)\n       :ensure t)\n</code></pre>\n<p>这里就直接贴上 <a href="https://github.com/jwiegley/use-package">use-package</a>文档的说明了:</p>\n<blockquote>\n<p>When you use the :commands keyword, it creates autoloads for those commands\nand defers loading of the module until they are used</p>\n</blockquote>\n<p>也就是 <code>:commands</code> 关键字就创建了后面所接的命令的 <code>autoloads</code> 机制了</p>\n<h3>:bind :mode</h3>\n<pre><code class="language-emacs-lisp">     (use-package hi-lock\n       :bind ((&quot;M-o l&quot; . highlight-lines-matching-regexp)\n              (&quot;M-o r&quot; . highlight-regexp)\n              (&quot;M-o w&quot; . highlight-phrase)))\n\n     (use-package vue-mode\n       :ensure t\n       :mode (&quot;\\\\.vue\\\\'&quot; . vue-mode)\n       :config (progn\n                 (setq mmm-submode-decoration-level 0)\n                 ))\n</code></pre>\n<p>附上文档说明</p>\n<blockquote>\n<p>In almost all cases you don't need to manually specify <code>:defer t</code>. This is implied\nwhenever <code>:bind</code> or <code>:mode</code> or <code>:interpreter</code> is used</p>\n</blockquote>\n<p>也就是说，当你使用了 <code>:bind</code> 或者 <code>:mode</code> 关键字的时候，不用明确指定 <code>:defer</code>\n也可以实现延迟加载机制。当然你也可以，直接使用 <code>:defer</code> 关键字来指定延迟加载\n不过前提是，你要明确它加载的时机</p>\n<blockquote>\n<p>Typically, you only need to specify :defer if you know for a fact that some\nother package will do something to cause your package to load at the appropriate\ntime, and thus you would like to defer loading even though use-package isn't\ncreating any autoloads for you.</p>\n</blockquote>\n<p>贴上笔者自己的代码，可以更加清晰</p>\n<pre><code>  (use-package anaconda-mode\n    :defer t\n    :ensure t\n    :init(progn\n           (add-hook 'python-mode-hook 'anaconda-mode)\n           (add-hook 'python-mode-hook 'anaconda-eldoc-mode)\n           ))\n \t\t  ```\n这样 `anaconda-mode` 就会在 `python-mode` 加载以后被加载\n\nEnjoy Emacs :)\n</code></pre>\n	2019-05-28 20:58:46	2019-05-28 20:58:46	2	3	t	优化Emacs启动速度	t
4	迟来的2018年总结	既然选择了远方, 便只顾风雨兼程	# 迟来的2018年总结\n一晃，2018年已经过去了\n\n6月25日，拖着行李，从广州来了杭州\n\n告别了学校，从学生变成了一个社会人\n\n------\n> 既然选择了远方, 便只顾风雨兼程 -- 汪国真\n\n# 工作\n从工作上来说，我"换"了两份工作，阿里大文娱和蚂蚁金服; 阿里大文娱-UC 2017.11-2018.5 实习，然后毕业之后入职蚂蚁金服-微贷-网商银行，主要是负责客户相关的业务；工作很累，但是总归是有收获的.\n\n入职蚂蚁之后，感觉就是忙，很忙。从新人培训的近卫军到回归日常业务，每天都有各种各样的事情需要处理，加班已经成为了工作中"不可磨灭的一部分"了\n\n刚入职的时候，给自己定了目标：业务上熟悉自己客户相关的业务，熟悉领域模型，继而从客户延伸了解整个网商银行的业务，学习金融知识；技术上学习组里的高可用架构，如何实现分布式系统的高可用，学习高并发-高可用-分布式-Java/蚂蚁中间件 生态；争取一年P6\n\n但是大半年下来，基本都是没有达到自己的预期目标，目测升P6的目标基本也是凉了。反思没有达到预期的原因；自身原因有之，外部原因亦有之. 10，11月这两个月，组里的同事被拉去做各种项目，之剩下包括我在内的两个开发，面对一堆需求，资源最紧张的时候，我们每个人，每个迭代需要开发3个需求，然而一个迭代开发加自测只有一周多的时间，实在是忙。忙导致的副作用就是累，而后下班回家只想睡觉，每天学习一个小时的目标早已抛之脑外。每天被需求推着走，没有对需求后面的意义进行思考，只是简单的需求翻译器，并不会有多少成长，兼之对需求不了解，导致需求发现变更的时候手忙脚乱. 12月之后需求缓下来之后，就开始有时间对之前做的事做个总结，可以对之前完成需求时积下来的问题进行反刍，结合现有的模型进行理解，过程虽费时，总归有收获；现有的业务开发开始渐入佳境，然后就开始"拥抱变化"，客户的业务全部交接别的团队，客户的团队被分流到其他团队，负责别的业务。以前总是听说阿里的"拥抱变化"，没想到来得如此之快，这么快就有了体感。\n\n# 生活\n2017.12.31-2018.12.31, 单身, 按下不表\n\n2018.6, 从UC 离职之后，趁着还有些许学生时光，就和两个好友去了趟顺德，品尝一下顺德的美食，所谓食在广州 厨出凤城，广州生活了四年，是时候去尝尝凤城(顺德)的滋味。4天的微游，终究是不虚此行，在蝦炳海鲜吃到了最好吃的烧鹅，每天去公寓对面的茶楼喝早茶，去了清晖园游园，也去了民信老店尝了各式甜品(感觉民信真的不咋地，贵且不说，味道还不咋地，不值特意来)\n\n2018.8, 团队outing去了趟庐山(基本自费)，庐山果真是个避暑圣地，把穿着短袖短裤，并只带了短袖短裤的这个广东人冻成dog。不过无论如何，庐山还是不虚此行的\n\n2018.10, 害怕挂了，开始重新踢球当作运动. 开始只有十分钟体能，全程只能散步，真的是丢人。过了两个多月，体能提升到了三十分钟，优秀\n\n2018.12, 毕业半年, 轻了十斤左右。看来工作真的是烧脑，占体重8%的器官，消费了超过20%的能量\n\n# 读书\n工作之后，买了两打书，分类大概是计算机相关/非计算书籍=3/1，然而过去近四个月，也只是读了不到三分之一的书，快餐文化盛行的今天，看来很难沉下心看书(不要甩锅呢)\n\n计算机相关: 重温了《Effective Java》, 《java并发编程实战》, 《 深入理解Java虚拟机》和《Java8 in action》《CSAPP》(没读完)，新入了《C++ primer》和 《UNIX环境高级编程》(没读完)\n非计算书籍大概读了《活着》，《追风筝的人》\n\n饭仍是要继续吃，书也是要继续读的.\n\n# 美食\n来了杭州之后，只做过一次饭，做给舍友吃, 幸好舍友还是吃得挺开心的 :) 而后再也没有做过。 看了三部美食纪录片聊以自慰 《人生一串》，《人间风味》，《寻味顺德》。 在次感谢《人生一串》，为广东人洗白，看来广东人吃得真的很正常，一点也不重口。\n\n嗯，在杭州想念广东的味道了，想念不吃辣的味道，想家了.\n\n# 展望\n1. 一年P6(感觉没戏了，那就两年P6)\n2. 了解分布式, 高可用的知识，争取通过实战掌握; 读完《netty in action》, 通过许家纯大大的教程，自己实现一个Rpc 框架；读Sofa-rpc 和 Netty 的源码\n3. 成为一个掌握金融知识的计算机从业人员\n4. 读完20本书\n5. 结束单身狗的生活(估计也没戏了)\n\n\n	<h1>迟来的2018年总结</h1>\n<p>一晃，2018年已经过去了</p>\n<p>6月25日，拖着行李，从广州来了杭州</p>\n<p>告别了学校，从学生变成了一个社会人</p>\n<hr>\n<blockquote>\n<p>既然选择了远方, 便只顾风雨兼程 -- 汪国真</p>\n</blockquote>\n<h1>工作</h1>\n<p>从工作上来说，我&quot;换&quot;了两份工作，阿里大文娱和蚂蚁金服; 阿里大文娱-UC 2017.11-2018.5 实习，然后毕业之后入职蚂蚁金服-微贷-网商银行，主要是负责客户相关的业务；工作很累，但是总归是有收获的.</p>\n<p>入职蚂蚁之后，感觉就是忙，很忙。从新人培训的近卫军到回归日常业务，每天都有各种各样的事情需要处理，加班已经成为了工作中&quot;不可磨灭的一部分&quot;了</p>\n<p>刚入职的时候，给自己定了目标：业务上熟悉自己客户相关的业务，熟悉领域模型，继而从客户延伸了解整个网商银行的业务，学习金融知识；技术上学习组里的高可用架构，如何实现分布式系统的高可用，学习高并发-高可用-分布式-Java/蚂蚁中间件 生态；争取一年P6</p>\n<p>但是大半年下来，基本都是没有达到自己的预期目标，目测升P6的目标基本也是凉了。反思没有达到预期的原因；自身原因有之，外部原因亦有之. 10，11月这两个月，组里的同事被拉去做各种项目，之剩下包括我在内的两个开发，面对一堆需求，资源最紧张的时候，我们每个人，每个迭代需要开发3个需求，然而一个迭代开发加自测只有一周多的时间，实在是忙。忙导致的副作用就是累，而后下班回家只想睡觉，每天学习一个小时的目标早已抛之脑外。每天被需求推着走，没有对需求后面的意义进行思考，只是简单的需求翻译器，并不会有多少成长，兼之对需求不了解，导致需求发现变更的时候手忙脚乱. 12月之后需求缓下来之后，就开始有时间对之前做的事做个总结，可以对之前完成需求时积下来的问题进行反刍，结合现有的模型进行理解，过程虽费时，总归有收获；现有的业务开发开始渐入佳境，然后就开始&quot;拥抱变化&quot;，客户的业务全部交接别的团队，客户的团队被分流到其他团队，负责别的业务。以前总是听说阿里的&quot;拥抱变化&quot;，没想到来得如此之快，这么快就有了体感。</p>\n<h1>生活</h1>\n<p>2017.12.31-2018.12.31, 单身, 按下不表</p>\n<p>2018.6, 从UC 离职之后，趁着还有些许学生时光，就和两个好友去了趟顺德，品尝一下顺德的美食，所谓食在广州 厨出凤城，广州生活了四年，是时候去尝尝凤城(顺德)的滋味。4天的微游，终究是不虚此行，在蝦炳海鲜吃到了最好吃的烧鹅，每天去公寓对面的茶楼喝早茶，去了清晖园游园，也去了民信老店尝了各式甜品(感觉民信真的不咋地，贵且不说，味道还不咋地，不值特意来)</p>\n<p>2018.8, 团队outing去了趟庐山(基本自费)，庐山果真是个避暑圣地，把穿着短袖短裤，并只带了短袖短裤的这个广东人冻成dog。不过无论如何，庐山还是不虚此行的</p>\n<p>2018.10, 害怕挂了，开始重新踢球当作运动. 开始只有十分钟体能，全程只能散步，真的是丢人。过了两个多月，体能提升到了三十分钟，优秀</p>\n<p>2018.12, 毕业半年, 轻了十斤左右。看来工作真的是烧脑，占体重8%的器官，消费了超过20%的能量</p>\n<h1>读书</h1>\n<p>工作之后，买了两打书，分类大概是计算机相关/非计算书籍=3/1，然而过去近四个月，也只是读了不到三分之一的书，快餐文化盛行的今天，看来很难沉下心看书(不要甩锅呢)</p>\n<p>计算机相关: 重温了《Effective Java》, 《java并发编程实战》, 《 深入理解Java虚拟机》和《Java8 in action》《CSAPP》(没读完)，新入了《C++ primer》和 《UNIX环境高级编程》(没读完)\n非计算书籍大概读了《活着》，《追风筝的人》</p>\n<p>饭仍是要继续吃，书也是要继续读的.</p>\n<h1>美食</h1>\n<p>来了杭州之后，只做过一次饭，做给舍友吃, 幸好舍友还是吃得挺开心的 :) 而后再也没有做过。 看了三部美食纪录片聊以自慰 《人生一串》，《人间风味》，《寻味顺德》。 在次感谢《人生一串》，为广东人洗白，看来广东人吃得真的很正常，一点也不重口。</p>\n<p>嗯，在杭州想念广东的味道了，想念不吃辣的味道，想家了.</p>\n<h1>展望</h1>\n<ol>\n<li>一年P6(感觉没戏了，那就两年P6)</li>\n<li>了解分布式, 高可用的知识，争取通过实战掌握; 读完《netty in action》, 通过许家纯大大的教程，自己实现一个Rpc 框架；读Sofa-rpc 和 Netty 的源码</li>\n<li>成为一个掌握金融知识的计算机从业人员</li>\n<li>读完20本书</li>\n<li>结束单身狗的生活(估计也没戏了)</li>\n</ol>\n	2019-05-28 21:44:49	2019-05-28 21:52:42	2	1	t	迟来的2018年总结	t
\.


--
-- Data for Name: user; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public."user" (id, username, hashed_password, create_time, modify_time, email, avatar_url) FROM stdin;
1	admin	$2y$10$QFkjuKUBF3s1ldzPFB8/WejgRJ9nW2CdXOSxfQJdnYXDcoIzrBkzS	2019-05-21 07:13:39.781829	2019-05-21 07:13:39.781829	admin@samray.xyz	https://imgur.com/a/N6Y97
\.


--
-- Data for Name: visitor_log; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.visitor_log (id, ip, access_time, user_id) FROM stdin;
1	::1	2019-05-21 15:13:53	0
2	127.0.0.1	2019-05-28 17:00:29	0
3	127.0.0.1	2019-05-28 17:00:53	0
4	127.0.0.1	2019-05-28 17:01:11	1
5	127.0.0.1	2019-05-28 17:01:13	1
6	127.0.0.1	2019-05-28 17:02:10	0
7	127.0.0.1	2019-05-28 17:02:30	0
8	127.0.0.1	2019-05-28 17:02:31	0
9	127.0.0.1	2019-05-28 17:02:33	0
10	127.0.0.1	2019-05-28 17:02:34	0
11	127.0.0.1	2019-05-28 17:02:35	0
12	127.0.0.1	2019-05-28 17:02:38	0
13	127.0.0.1	2019-05-28 17:03:31	0
14	127.0.0.1	2019-05-28 17:03:33	0
15	127.0.0.1	2019-05-28 17:03:34	0
16	127.0.0.1	2019-05-28 17:03:34	0
17	127.0.0.1	2019-05-28 17:03:41	0
18	127.0.0.1	2019-05-28 17:03:43	0
19	127.0.0.1	2019-05-28 17:03:44	0
20	127.0.0.1	2019-05-28 17:03:45	0
21	127.0.0.1	2019-05-28 17:03:46	0
22	127.0.0.1	2019-05-28 17:03:48	0
23	127.0.0.1	2019-05-28 17:03:48	0
24	127.0.0.1	2019-05-28 17:03:55	0
25	127.0.0.1	2019-05-28 17:04:08	0
26	127.0.0.1	2019-05-28 17:04:15	0
27	127.0.0.1	2019-05-28 17:04:15	0
28	127.0.0.1	2019-05-28 17:04:16	0
29	127.0.0.1	2019-05-28 17:04:17	0
30	127.0.0.1	2019-05-28 17:04:18	0
31	127.0.0.1	2019-05-28 17:04:19	0
32	127.0.0.1	2019-05-28 17:04:21	0
33	127.0.0.1	2019-05-28 17:04:22	0
34	127.0.0.1	2019-05-28 17:04:22	0
35	127.0.0.1	2019-05-28 17:04:28	0
36	127.0.0.1	2019-05-28 17:04:36	0
37	127.0.0.1	2019-05-28 17:06:06	0
38	127.0.0.1	2019-05-28 17:06:07	0
39	127.0.0.1	2019-05-28 17:09:39	0
40	127.0.0.1	2019-05-28 17:21:06	0
41	127.0.0.1	2019-05-28 17:25:37	0
42	127.0.0.1	2019-05-28 17:26:59	0
43	127.0.0.1	2019-05-28 17:34:36	0
44	127.0.0.1	2019-05-28 17:37:27	0
45	127.0.0.1	2019-05-28 17:46:34	0
46	127.0.0.1	2019-05-28 17:46:48	1
47	127.0.0.1	2019-05-28 17:47:20	0
48	127.0.0.1	2019-05-28 17:47:21	0
49	127.0.0.1	2019-05-28 17:50:40	0
50	127.0.0.1	2019-05-28 17:53:31	0
51	127.0.0.1	2019-05-28 18:00:31	0
52	127.0.0.1	2019-05-28 18:00:49	0
53	127.0.0.1	2019-05-28 18:01:39	0
54	127.0.0.1	2019-05-28 18:02:52	0
55	127.0.0.1	2019-05-28 18:04:13	0
56	127.0.0.1	2019-05-28 19:24:39	0
57	127.0.0.1	2019-05-28 19:24:41	0
58	127.0.0.1	2019-05-28 20:41:36	0
59	127.0.0.1	2019-05-28 20:41:38	0
60	127.0.0.1	2019-05-28 20:41:56	0
61	127.0.0.1	2019-05-28 20:42:53	0
62	127.0.0.1	2019-05-28 20:44:20	0
63	127.0.0.1	2019-05-28 20:44:22	0
64	127.0.0.1	2019-05-28 20:44:23	0
65	127.0.0.1	2019-05-28 20:44:42	0
66	127.0.0.1	2019-05-28 20:44:43	0
67	127.0.0.1	2019-05-28 20:46:18	0
68	127.0.0.1	2019-05-28 20:46:46	0
69	127.0.0.1	2019-05-28 20:47:33	0
70	127.0.0.1	2019-05-28 20:47:46	0
71	127.0.0.1	2019-05-28 20:51:53	0
72	127.0.0.1	2019-05-28 20:52:04	0
73	127.0.0.1	2019-05-28 20:52:12	0
74	127.0.0.1	2019-05-28 20:52:14	0
75	127.0.0.1	2019-05-28 20:52:18	0
76	127.0.0.1	2019-05-28 20:56:58	0
77	127.0.0.1	2019-05-28 20:58:51	0
78	127.0.0.1	2019-05-28 20:58:53	0
79	127.0.0.1	2019-05-28 21:02:38	0
80	127.0.0.1	2019-05-28 21:04:30	0
81	127.0.0.1	2019-05-28 21:04:52	0
82	127.0.0.1	2019-05-28 21:04:57	0
83	127.0.0.1	2019-05-28 21:05:10	0
84	127.0.0.1	2019-05-28 21:05:25	0
85	127.0.0.1	2019-05-28 21:06:53	0
86	127.0.0.1	2019-05-28 21:07:04	1
87	127.0.0.1	2019-05-28 21:45:01	0
88	127.0.0.1	2019-05-28 21:45:07	0
89	127.0.0.1	2019-05-28 21:54:00	0
90	127.0.0.1	2019-05-28 21:54:06	0
91	127.0.0.1	2019-05-28 21:56:02	0
92	127.0.0.1	2019-05-28 21:56:04	0
93	127.0.0.1	2019-05-28 21:59:53	0
94	127.0.0.1	2019-05-28 21:59:56	0
95	127.0.0.1	2019-05-28 22:01:30	0
96	127.0.0.1	2019-05-28 22:01:31	0
97	::1	2019-05-28 22:24:57	0
98	::1	2019-05-28 22:24:59	0
99	::1	2019-05-28 22:25:04	0
100	::1	2019-05-29 10:06:34	0
101	::1	2019-05-29 10:06:36	0
102	::1	2019-05-29 10:07:07	0
103	::1	2019-05-29 10:07:30	0
104	::1	2019-05-29 10:07:34	0
105	127.0.0.1	2019-05-29 10:08:00	0
106	127.0.0.1	2019-05-29 10:08:20	0
107	::1	2019-05-29 10:12:04	0
\.


--
-- Name: post_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.post_id_seq', 9, true);


--
-- Name: user_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.user_id_seq', 1, false);


--
-- Name: visitor_log_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.visitor_log_id_seq', 107, true);


--
-- Name: __diesel_schema_migrations __diesel_schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.__diesel_schema_migrations
    ADD CONSTRAINT __diesel_schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: post post_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.post
    ADD CONSTRAINT post_pkey PRIMARY KEY (id);


--
-- Name: user user_email_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."user"
    ADD CONSTRAINT user_email_key UNIQUE (email);


--
-- Name: user user_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."user"
    ADD CONSTRAINT user_pkey PRIMARY KEY (id);


--
-- Name: visitor_log visitor_log_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.visitor_log
    ADD CONSTRAINT visitor_log_pkey PRIMARY KEY (id);


--
-- PostgreSQL database dump complete
--

