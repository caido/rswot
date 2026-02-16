use std::borrow::Cow;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Tld(Cow<'static, str>);

impl Tld {
    pub const fn new(tld: &'static str) -> Tld {
        Tld(Cow::Borrowed(tld))
    }

    pub fn new_owned(tld: String) -> Tld {
        Tld(Cow::Owned(tld))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl Tld {
    // Countries and territories (alphabetical by constant name)
    pub const ALAND_ISLANDS: Tld = Tld::new("ax");
    pub const ALBANIA: Tld = Tld::new("al");
    pub const ALGERIA: Tld = Tld::new("dz");
    pub const ANDORRA: Tld = Tld::new("ad");
    pub const ANGOLA: Tld = Tld::new("ao");
    pub const ANGUILLA: Tld = Tld::new("ai");
    pub const ANTIGUA_AND_BARBUDA: Tld = Tld::new("ag");
    pub const ARGENTINA: Tld = Tld::new("ar");
    pub const ARMENIA: Tld = Tld::new("am");
    pub const AUSTRALIA: Tld = Tld::new("au");
    pub const AUSTRIA: Tld = Tld::new("at");
    pub const AZERBAIJAN: Tld = Tld::new("az");
    pub const BAHAMAS: Tld = Tld::new("bs");
    pub const BAHRAIN: Tld = Tld::new("bh");
    pub const BANGLADESH: Tld = Tld::new("bd");
    pub const BARBADOS: Tld = Tld::new("bb");
    pub const BASQUE_COUNTRY: Tld = Tld::new("eus");
    pub const BELARUS: Tld = Tld::new("by");
    pub const BELGIUM: Tld = Tld::new("be");
    pub const BELIZE: Tld = Tld::new("bz");
    pub const BENIN: Tld = Tld::new("bj");
    pub const BERMUDA: Tld = Tld::new("bm");
    pub const BHUTAN: Tld = Tld::new("bt");
    pub const BOLIVIA: Tld = Tld::new("bo");
    pub const BOSNIA_AND_HERZEGOVINA: Tld = Tld::new("ba");
    pub const BOTSWANA: Tld = Tld::new("bw");
    pub const BRAZIL: Tld = Tld::new("br");
    pub const BRITISH_VIRGIN_ISLANDS: Tld = Tld::new("vg");
    pub const BRITTANY: Tld = Tld::new("bzh");
    pub const BRUNEI: Tld = Tld::new("bn");
    pub const BULGARIA: Tld = Tld::new("bg");
    pub const BURKINA_FASO: Tld = Tld::new("bf");
    pub const BURUNDI: Tld = Tld::new("bi");
    pub const CAMBODIA: Tld = Tld::new("kh");
    pub const CAMEROON: Tld = Tld::new("cm");
    pub const CANADA: Tld = Tld::new("ca");
    pub const CAPE_VERDE: Tld = Tld::new("cv");
    pub const CAYMAN_ISLANDS: Tld = Tld::new("ky");
    pub const CHILE: Tld = Tld::new("cl");
    pub const CHINA: Tld = Tld::new("cn");
    pub const COCOS_ISLANDS: Tld = Tld::new("cc");
    pub const COLOMBIA: Tld = Tld::new("co");
    pub const COMOROS: Tld = Tld::new("km");
    pub const CONGO_DR: Tld = Tld::new("cd");
    pub const COSTA_RICA: Tld = Tld::new("cr");
    pub const CROATIA: Tld = Tld::new("hr");
    pub const CUBA: Tld = Tld::new("cu");
    pub const CURACAO: Tld = Tld::new("cw");
    pub const CYPRUS: Tld = Tld::new("cy");
    pub const CZECH_REPUBLIC: Tld = Tld::new("cz");
    pub const DENMARK: Tld = Tld::new("dk");
    pub const DJIBOUTI: Tld = Tld::new("dj");
    pub const DOMINICAN_REPUBLIC: Tld = Tld::new("do");
    pub const ECUADOR: Tld = Tld::new("ec");
    pub const EGYPT: Tld = Tld::new("eg");
    pub const EL_SALVADOR: Tld = Tld::new("sv");
    pub const ESTONIA: Tld = Tld::new("ee");
    pub const ESWATINI: Tld = Tld::new("sz");
    pub const ETHIOPIA: Tld = Tld::new("et");
    pub const FAROE_ISLANDS: Tld = Tld::new("fo");
    pub const FIJI: Tld = Tld::new("fj");
    pub const FINLAND: Tld = Tld::new("fi");
    pub const FRANCE: Tld = Tld::new("fr");
    pub const FRENCH_POLYNESIA: Tld = Tld::new("pf");
    pub const GAMBIA: Tld = Tld::new("gm");
    pub const GEORGIA: Tld = Tld::new("ge");
    pub const GERMANY: Tld = Tld::new("de");
    pub const GHANA: Tld = Tld::new("gh");
    pub const GIBRALTAR: Tld = Tld::new("gi");
    pub const GREECE: Tld = Tld::new("gr");
    pub const GREENLAND: Tld = Tld::new("gl");
    pub const GRENADA: Tld = Tld::new("gd");
    pub const GUATEMALA: Tld = Tld::new("gt");
    pub const GUYANA: Tld = Tld::new("gy");
    pub const HAITI: Tld = Tld::new("ht");
    pub const HONDURAS: Tld = Tld::new("hn");
    pub const HONG_KONG: Tld = Tld::new("hk");
    pub const HUNGARY: Tld = Tld::new("hu");
    pub const ICELAND: Tld = Tld::new("is");
    pub const INDIA: Tld = Tld::new("in");
    pub const INDONESIA: Tld = Tld::new("id");
    pub const IRAN: Tld = Tld::new("ir");
    pub const IRAQ: Tld = Tld::new("iq");
    pub const IRELAND: Tld = Tld::new("ie");
    pub const ISLE_OF_MAN: Tld = Tld::new("im");
    pub const ISRAEL: Tld = Tld::new("il");
    pub const ITALY: Tld = Tld::new("it");
    pub const IVORY_COAST: Tld = Tld::new("ci");
    pub const JAMAICA: Tld = Tld::new("jm");
    pub const JAPAN: Tld = Tld::new("jp");
    pub const JORDAN: Tld = Tld::new("jo");
    pub const KAZAKHSTAN: Tld = Tld::new("kz");
    pub const KENYA: Tld = Tld::new("ke");
    pub const KUWAIT: Tld = Tld::new("kw");
    pub const KYRGYZSTAN: Tld = Tld::new("kg");
    pub const LAOS: Tld = Tld::new("la");
    pub const LATVIA: Tld = Tld::new("lv");
    pub const LEBANON: Tld = Tld::new("lb");
    pub const LESOTHO: Tld = Tld::new("ls");
    pub const LIBYA: Tld = Tld::new("ly");
    pub const LIECHTENSTEIN: Tld = Tld::new("li");
    pub const LITHUANIA: Tld = Tld::new("lt");
    pub const LUXEMBOURG: Tld = Tld::new("lu");
    pub const MACAU: Tld = Tld::new("mo");
    pub const MADAGASCAR: Tld = Tld::new("mg");
    pub const MALAWI: Tld = Tld::new("mw");
    pub const MALAYSIA: Tld = Tld::new("my");
    pub const MALDIVES: Tld = Tld::new("mv");
    pub const MALTA: Tld = Tld::new("mt");
    pub const MAURITANIA: Tld = Tld::new("mr");
    pub const MAURITIUS: Tld = Tld::new("mu");
    pub const MEXICO: Tld = Tld::new("mx");
    pub const MICRONESIA: Tld = Tld::new("fm");
    pub const MOLDOVA: Tld = Tld::new("md");
    pub const MONGOLIA: Tld = Tld::new("mn");
    pub const MONTENEGRO: Tld = Tld::new("me");
    pub const MOROCCO: Tld = Tld::new("ma");
    pub const MOZAMBIQUE: Tld = Tld::new("mz");
    pub const MYANMAR: Tld = Tld::new("mm");
    pub const NAMIBIA: Tld = Tld::new("na");
    pub const NEPAL: Tld = Tld::new("np");
    pub const NETHERLANDS: Tld = Tld::new("nl");
    pub const NEW_CALEDONIA: Tld = Tld::new("nc");
    pub const NEW_ZEALAND: Tld = Tld::new("nz");
    pub const NICARAGUA: Tld = Tld::new("ni");
    pub const NIGER: Tld = Tld::new("ne");
    pub const NIGERIA: Tld = Tld::new("ng");
    pub const NIUE: Tld = Tld::new("nu");
    pub const NORTH_MACEDONIA: Tld = Tld::new("mk");
    pub const NORWAY: Tld = Tld::new("no");
    pub const OMAN: Tld = Tld::new("om");
    pub const PAKISTAN: Tld = Tld::new("pk");
    pub const PALESTINE: Tld = Tld::new("ps");
    pub const PANAMA: Tld = Tld::new("pa");
    pub const PAPUA_NEW_GUINEA: Tld = Tld::new("pg");
    pub const PARAGUAY: Tld = Tld::new("py");
    pub const PERU: Tld = Tld::new("pe");
    pub const PHILIPPINES: Tld = Tld::new("ph");
    pub const POLAND: Tld = Tld::new("pl");
    pub const PORTUGAL: Tld = Tld::new("pt");
    pub const PUERTO_RICO: Tld = Tld::new("pr");
    pub const QATAR: Tld = Tld::new("qa");
    pub const ROMANIA: Tld = Tld::new("ro");
    pub const RUSSIA: Tld = Tld::new("ru");
    pub const RWANDA: Tld = Tld::new("rw");
    pub const SAINT_HELENA: Tld = Tld::new("sh");
    pub const SAINT_KITTS_AND_NEVIS: Tld = Tld::new("kn");
    pub const SAINT_LUCIA: Tld = Tld::new("lc");
    pub const SAINT_VINCENT_AND_THE_GRENADINES: Tld = Tld::new("vc");
    pub const SAN_MARINO: Tld = Tld::new("sm");
    pub const SAUDI_ARABIA: Tld = Tld::new("sa");
    pub const SENEGAL: Tld = Tld::new("sn");
    pub const SERBIA: Tld = Tld::new("rs");
    pub const SINGAPORE: Tld = Tld::new("sg");
    pub const SLOVAKIA: Tld = Tld::new("sk");
    pub const SLOVENIA: Tld = Tld::new("si");
    pub const SOMALIA: Tld = Tld::new("so");
    pub const SOUTH_AFRICA: Tld = Tld::new("za");
    pub const SOUTH_KOREA: Tld = Tld::new("kr");
    pub const SOVIET_UNION: Tld = Tld::new("su");
    pub const SPAIN: Tld = Tld::new("es");
    pub const SRI_LANKA: Tld = Tld::new("lk");
    pub const SUDAN: Tld = Tld::new("sd");
    pub const SURINAME: Tld = Tld::new("sr");
    pub const SWEDEN: Tld = Tld::new("se");
    pub const SWITZERLAND: Tld = Tld::new("ch");
    pub const SYRIA: Tld = Tld::new("sy");
    pub const TAIWAN: Tld = Tld::new("tw");
    pub const TAJIKISTAN: Tld = Tld::new("tj");
    pub const TANZANIA: Tld = Tld::new("tz");
    pub const THAILAND: Tld = Tld::new("th");
    pub const TOGO: Tld = Tld::new("tg");
    pub const TRINIDAD_AND_TOBAGO: Tld = Tld::new("tt");
    pub const TUNISIA: Tld = Tld::new("tn");
    pub const TURKEY: Tld = Tld::new("tr");
    pub const TURKMENISTAN: Tld = Tld::new("tm");
    pub const UGANDA: Tld = Tld::new("ug");
    pub const UKRAINE: Tld = Tld::new("ua");
    pub const UNITED_ARAB_EMIRATES: Tld = Tld::new("ae");
    pub const UNITED_KINGDOM: Tld = Tld::new("uk");
    pub const UNITED_STATES: Tld = Tld::new("us");
    pub const URUGUAY: Tld = Tld::new("uy");
    pub const UZBEKISTAN: Tld = Tld::new("uz");
    pub const VENEZUELA: Tld = Tld::new("ve");
    pub const VIETNAM: Tld = Tld::new("vn");
    pub const YEMEN: Tld = Tld::new("ye");
    pub const ZAMBIA: Tld = Tld::new("zm");
    pub const ZIMBABWE: Tld = Tld::new("zw");

    // Other TLDs (alphabetical by constant name)
    pub const AC: Tld = Tld::new("ac");
    pub const ACADEMY: Tld = Tld::new("academy");
    pub const AERO: Tld = Tld::new("aero");
    pub const ART: Tld = Tld::new("art");
    pub const ASIA: Tld = Tld::new("asia");
    pub const BARCELONA: Tld = Tld::new("barcelona");
    pub const BAYERN: Tld = Tld::new("bayern");
    pub const BERLIN: Tld = Tld::new("berlin");
    pub const CAT: Tld = Tld::new("cat");
    pub const CLOUD: Tld = Tld::new("cloud");
    pub const COLLEGE: Tld = Tld::new("college");
    pub const COM: Tld = Tld::new("com");
    pub const CORSICA: Tld = Tld::new("corsica");
    pub const CYMRU: Tld = Tld::new("cymru");
    pub const DIGITAL: Tld = Tld::new("digital");
    pub const EDU: Tld = Tld::new("edu");
    pub const EDUCATION: Tld = Tld::new("education");
    pub const EMAIL: Tld = Tld::new("email");
    pub const EU: Tld = Tld::new("eu");
    pub const GAL: Tld = Tld::new("gal");
    pub const GLOBAL: Tld = Tld::new("global");
    pub const GOV: Tld = Tld::new("gov");
    pub const HAMBURG: Tld = Tld::new("hamburg");
    pub const HOUSE: Tld = Tld::new("house");
    pub const INFO: Tld = Tld::new("info");
    pub const INSTITUTE: Tld = Tld::new("institute");
    pub const IO: Tld = Tld::new("io");
    pub const KOELN: Tld = Tld::new("koeln");
    pub const KRD: Tld = Tld::new("krd");
    pub const LONDON: Tld = Tld::new("london");
    pub const MIL: Tld = Tld::new("mil");
    pub const MOBI: Tld = Tld::new("mobi");
    pub const MOSCOW: Tld = Tld::new("moscow");
    pub const NAME: Tld = Tld::new("name");
    pub const NET: Tld = Tld::new("net");
    pub const NETWORK: Tld = Tld::new("network");
    pub const NRW: Tld = Tld::new("nrw");
    pub const ONE: Tld = Tld::new("one");
    pub const ONLINE: Tld = Tld::new("online");
    pub const ORG: Tld = Tld::new("org");
    pub const PARIS: Tld = Tld::new("paris");
    pub const QUEBEC: Tld = Tld::new("quebec");
    pub const RIO: Tld = Tld::new("rio");
    pub const ROCKS: Tld = Tld::new("rocks");
    pub const SCHOOL: Tld = Tld::new("school");
    pub const SCHULE: Tld = Tld::new("schule");
    pub const SCOT: Tld = Tld::new("scot");
    pub const SPACE: Tld = Tld::new("space");
    pub const STUDY: Tld = Tld::new("study");
    pub const SWISS: Tld = Tld::new("swiss");
    pub const TECH: Tld = Tld::new("tech");
    pub const UNIVERSITY: Tld = Tld::new("university");
    pub const WALES: Tld = Tld::new("wales");
    pub const WORLD: Tld = Tld::new("world");
    pub const WS: Tld = Tld::new("ws");
    pub const XYZ: Tld = Tld::new("xyz");
}

impl AsRef<str> for Tld {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for Tld {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0.as_ref())
    }
}
