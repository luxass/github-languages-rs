use std::collections::HashMap;
#[derive(Debug, Clone, PartialEq)]
pub struct Language {
    pub name: &'static str,
    pub type_: &'static str,
    pub color: &'static str,
    pub extensions: &'static [&'static str],
    pub tm_scope: &'static str,
    pub ace_mode: &'static str,
    pub language_id: u64,
}
pub struct _1CEnterprise;
impl _1CEnterprise {
    pub fn info() -> Language {
        Language {
            name: "1C Enterprise",
            type_: "programming",
            color: "#814CCC",
            extensions: &[".bsl", ".os"],
            tm_scope: "source.bsl",
            ace_mode: "text",
            language_id: 0,
        }
    }
}
pub struct _2DimensionalArray;
impl _2DimensionalArray {
    pub fn info() -> Language {
        Language {
            name: "2-Dimensional Array",
            type_: "data",
            color: "#38761D",
            extensions: &[".2da"],
            tm_scope: "source.2da",
            ace_mode: "text",
            language_id: 387204628,
        }
    }
}
pub struct _4D;
impl _4D {
    pub fn info() -> Language {
        Language {
            name: "4D",
            type_: "programming",
            color: "#004289",
            extensions: &[".4dm"],
            tm_scope: "source.4dm",
            ace_mode: "text",
            language_id: 577529595,
        }
    }
}
pub struct ABAP;
impl ABAP {
    pub fn info() -> Language {
        Language {
            name: "ABAP",
            type_: "programming",
            color: "#E8274B",
            extensions: &[".abap"],
            tm_scope: "source.abap",
            ace_mode: "abap",
            language_id: 1,
        }
    }
}
pub struct ABAPCDS;
impl ABAPCDS {
    pub fn info() -> Language {
        Language {
            name: "ABAP CDS",
            type_: "programming",
            color: "#555e25",
            extensions: &[".asddls"],
            tm_scope: "source.abapcds",
            ace_mode: "text",
            language_id: 452681853,
        }
    }
}
pub struct ABNF;
impl ABNF {
    pub fn info() -> Language {
        Language {
            name: "ABNF",
            type_: "data",
            color: "",
            extensions: &[".abnf"],
            tm_scope: "source.abnf",
            ace_mode: "text",
            language_id: 429,
        }
    }
}
pub struct AGSScript;
impl AGSScript {
    pub fn info() -> Language {
        Language {
            name: "AGS Script",
            type_: "programming",
            color: "#B9D9FF",
            extensions: &[".asc", ".ash"],
            tm_scope: "source.c++",
            ace_mode: "c_cpp",
            language_id: 2,
        }
    }
}
pub struct AIDL;
impl AIDL {
    pub fn info() -> Language {
        Language {
            name: "AIDL",
            type_: "programming",
            color: "#34EB6B",
            extensions: &[".aidl"],
            tm_scope: "source.aidl",
            ace_mode: "text",
            language_id: 451700185,
        }
    }
}
pub struct AL;
impl AL {
    pub fn info() -> Language {
        Language {
            name: "AL",
            type_: "programming",
            color: "#3AA2B5",
            extensions: &[".al"],
            tm_scope: "source.al",
            ace_mode: "text",
            language_id: 658971832,
        }
    }
}
pub struct AMPL;
impl AMPL {
    pub fn info() -> Language {
        Language {
            name: "AMPL",
            type_: "programming",
            color: "#E6EFBB",
            extensions: &[".ampl", ".mod"],
            tm_scope: "source.ampl",
            ace_mode: "text",
            language_id: 3,
        }
    }
}
pub struct ANTLR;
impl ANTLR {
    pub fn info() -> Language {
        Language {
            name: "ANTLR",
            type_: "programming",
            color: "#9DC3FF",
            extensions: &[".g4"],
            tm_scope: "source.antlr",
            ace_mode: "text",
            language_id: 4,
        }
    }
}
pub struct APIBlueprint;
impl APIBlueprint {
    pub fn info() -> Language {
        Language {
            name: "API Blueprint",
            type_: "markup",
            color: "#2ACCA8",
            extensions: &[".apib"],
            tm_scope: "text.html.markdown.source.gfm.apib",
            ace_mode: "markdown",
            language_id: 5,
        }
    }
}
pub struct APL;
impl APL {
    pub fn info() -> Language {
        Language {
            name: "APL",
            type_: "programming",
            color: "#5A8164",
            extensions: &[".apl", ".dyalog"],
            tm_scope: "source.apl",
            ace_mode: "text",
            language_id: 6,
        }
    }
}
pub struct ASL;
impl ASL {
    pub fn info() -> Language {
        Language {
            name: "ASL",
            type_: "programming",
            color: "",
            extensions: &[".asl", ".dsl"],
            tm_scope: "source.asl",
            ace_mode: "text",
            language_id: 124996147,
        }
    }
}
pub struct ASN1;
impl ASN1 {
    pub fn info() -> Language {
        Language {
            name: "ASN.1",
            type_: "data",
            color: "",
            extensions: &[".asn", ".asn1"],
            tm_scope: "source.asn",
            ace_mode: "text",
            language_id: 7,
        }
    }
}
pub struct ASPNET;
impl ASPNET {
    pub fn info() -> Language {
        Language {
            name: "ASP.NET",
            type_: "programming",
            color: "#9400ff",
            extensions: &[".asax", ".ascx", ".ashx", ".asmx", ".aspx", ".axd"],
            tm_scope: "text.html.asp",
            ace_mode: "text",
            language_id: 564186416,
        }
    }
}
pub struct ATS;
impl ATS {
    pub fn info() -> Language {
        Language {
            name: "ATS",
            type_: "programming",
            color: "#1ac620",
            extensions: &[".dats", ".hats", ".sats"],
            tm_scope: "source.ats",
            ace_mode: "ocaml",
            language_id: 9,
        }
    }
}
pub struct ActionScript;
impl ActionScript {
    pub fn info() -> Language {
        Language {
            name: "ActionScript",
            type_: "programming",
            color: "#882B0F",
            extensions: &[".as"],
            tm_scope: "source.actionscript.3",
            ace_mode: "actionscript",
            language_id: 10,
        }
    }
}
pub struct Ada;
impl Ada {
    pub fn info() -> Language {
        Language {
            name: "Ada",
            type_: "programming",
            color: "#02f88c",
            extensions: &[".adb", ".ada", ".ads"],
            tm_scope: "source.ada",
            ace_mode: "ada",
            language_id: 11,
        }
    }
}
pub struct AdblockFilterList;
impl AdblockFilterList {
    pub fn info() -> Language {
        Language {
            name: "Adblock Filter List",
            type_: "data",
            color: "#800000",
            extensions: &[".txt"],
            tm_scope: "text.adblock",
            ace_mode: "text",
            language_id: 884614762,
        }
    }
}
pub struct AdobeFontMetrics;
impl AdobeFontMetrics {
    pub fn info() -> Language {
        Language {
            name: "Adobe Font Metrics",
            type_: "data",
            color: "#fa0f00",
            extensions: &[".afm"],
            tm_scope: "source.afm",
            ace_mode: "text",
            language_id: 147198098,
        }
    }
}
pub struct Agda;
impl Agda {
    pub fn info() -> Language {
        Language {
            name: "Agda",
            type_: "programming",
            color: "#315665",
            extensions: &[".agda"],
            tm_scope: "source.agda",
            ace_mode: "text",
            language_id: 12,
        }
    }
}
pub struct Alloy;
impl Alloy {
    pub fn info() -> Language {
        Language {
            name: "Alloy",
            type_: "programming",
            color: "#64C800",
            extensions: &[".als"],
            tm_scope: "source.alloy",
            ace_mode: "text",
            language_id: 13,
        }
    }
}
pub struct AlpineAbuild;
impl AlpineAbuild {
    pub fn info() -> Language {
        Language {
            name: "Alpine Abuild",
            type_: "programming",
            color: "#0D597F",
            extensions: &[],
            tm_scope: "source.shell",
            ace_mode: "sh",
            language_id: 14,
        }
    }
}
pub struct AltiumDesigner;
impl AltiumDesigner {
    pub fn info() -> Language {
        Language {
            name: "Altium Designer",
            type_: "data",
            color: "#A89663",
            extensions: &[".OutJob", ".PcbDoc", ".PrjPCB", ".SchDoc"],
            tm_scope: "source.ini",
            ace_mode: "ini",
            language_id: 187772328,
        }
    }
}
pub struct AngelScript;
impl AngelScript {
    pub fn info() -> Language {
        Language {
            name: "AngelScript",
            type_: "programming",
            color: "#C7D7DC",
            extensions: &[".as", ".angelscript"],
            tm_scope: "source.angelscript",
            ace_mode: "text",
            language_id: 389477596,
        }
    }
}
pub struct AntBuildSystem;
impl AntBuildSystem {
    pub fn info() -> Language {
        Language {
            name: "Ant Build System",
            type_: "data",
            color: "#A9157E",
            extensions: &[],
            tm_scope: "text.xml.ant",
            ace_mode: "xml",
            language_id: 15,
        }
    }
}
pub struct Antlers;
impl Antlers {
    pub fn info() -> Language {
        Language {
            name: "Antlers",
            type_: "markup",
            color: "#ff269e",
            extensions: &[".antlers.html", ".antlers.php", ".antlers.xml"],
            tm_scope: "text.html.statamic",
            ace_mode: "text",
            language_id: 1067292663,
        }
    }
}
pub struct ApacheConf;
impl ApacheConf {
    pub fn info() -> Language {
        Language {
            name: "ApacheConf",
            type_: "data",
            color: "#d12127",
            extensions: &[".apacheconf", ".vhost"],
            tm_scope: "source.apache-config",
            ace_mode: "apache_conf",
            language_id: 16,
        }
    }
}
pub struct Apex;
impl Apex {
    pub fn info() -> Language {
        Language {
            name: "Apex",
            type_: "programming",
            color: "#1797c0",
            extensions: &[".cls", ".trigger"],
            tm_scope: "source.apex",
            ace_mode: "java",
            language_id: 17,
        }
    }
}
pub struct ApolloGuidanceComputer;
impl ApolloGuidanceComputer {
    pub fn info() -> Language {
        Language {
            name: "Apollo Guidance Computer",
            type_: "programming",
            color: "#0B3D91",
            extensions: &[".agc"],
            tm_scope: "source.agc",
            ace_mode: "assembly_x86",
            language_id: 18,
        }
    }
}
pub struct AppleScript;
impl AppleScript {
    pub fn info() -> Language {
        Language {
            name: "AppleScript",
            type_: "programming",
            color: "#101F1F",
            extensions: &[".applescript", ".scpt"],
            tm_scope: "source.applescript",
            ace_mode: "applescript",
            language_id: 19,
        }
    }
}
pub struct Arc;
impl Arc {
    pub fn info() -> Language {
        Language {
            name: "Arc",
            type_: "programming",
            color: "#aa2afe",
            extensions: &[".arc"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 20,
        }
    }
}
pub struct AsciiDoc;
impl AsciiDoc {
    pub fn info() -> Language {
        Language {
            name: "AsciiDoc",
            type_: "prose",
            color: "#73a0c5",
            extensions: &[".asciidoc", ".adoc", ".asc"],
            tm_scope: "text.html.asciidoc",
            ace_mode: "asciidoc",
            language_id: 22,
        }
    }
}
pub struct AspectJ;
impl AspectJ {
    pub fn info() -> Language {
        Language {
            name: "AspectJ",
            type_: "programming",
            color: "#a957b0",
            extensions: &[".aj"],
            tm_scope: "source.aspectj",
            ace_mode: "text",
            language_id: 23,
        }
    }
}
pub struct Assembly;
impl Assembly {
    pub fn info() -> Language {
        Language {
            name: "Assembly",
            type_: "programming",
            color: "#6E4C13",
            extensions: &[".asm", ".a51", ".i", ".inc", ".nas", ".nasm"],
            tm_scope: "source.assembly",
            ace_mode: "assembly_x86",
            language_id: 24,
        }
    }
}
pub struct Astro;
impl Astro {
    pub fn info() -> Language {
        Language {
            name: "Astro",
            type_: "markup",
            color: "#ff5a03",
            extensions: &[".astro"],
            tm_scope: "source.astro",
            ace_mode: "html",
            language_id: 578209015,
        }
    }
}
pub struct Asymptote;
impl Asymptote {
    pub fn info() -> Language {
        Language {
            name: "Asymptote",
            type_: "programming",
            color: "#ff0000",
            extensions: &[".asy"],
            tm_scope: "source.c++",
            ace_mode: "c_cpp",
            language_id: 591605007,
        }
    }
}
pub struct Augeas;
impl Augeas {
    pub fn info() -> Language {
        Language {
            name: "Augeas",
            type_: "programming",
            color: "#9CC134",
            extensions: &[".aug"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 25,
        }
    }
}
pub struct AutoHotkey;
impl AutoHotkey {
    pub fn info() -> Language {
        Language {
            name: "AutoHotkey",
            type_: "programming",
            color: "#6594b9",
            extensions: &[".ahk", ".ahkl"],
            tm_scope: "source.ahk",
            ace_mode: "autohotkey",
            language_id: 26,
        }
    }
}
pub struct AutoIt;
impl AutoIt {
    pub fn info() -> Language {
        Language {
            name: "AutoIt",
            type_: "programming",
            color: "#1C3552",
            extensions: &[".au3"],
            tm_scope: "source.autoit",
            ace_mode: "autohotkey",
            language_id: 27,
        }
    }
}
pub struct AvroIDL;
impl AvroIDL {
    pub fn info() -> Language {
        Language {
            name: "Avro IDL",
            type_: "data",
            color: "#0040FF",
            extensions: &[".avdl"],
            tm_scope: "source.avro",
            ace_mode: "text",
            language_id: 785497837,
        }
    }
}
pub struct Awk;
impl Awk {
    pub fn info() -> Language {
        Language {
            name: "Awk",
            type_: "programming",
            color: "#c30e9b",
            extensions: &[".awk", ".auk", ".gawk", ".mawk", ".nawk"],
            tm_scope: "source.awk",
            ace_mode: "text",
            language_id: 28,
        }
    }
}
pub struct B4X;
impl B4X {
    pub fn info() -> Language {
        Language {
            name: "B4X",
            type_: "programming",
            color: "#00e4ff",
            extensions: &[".bas"],
            tm_scope: "source.vba",
            ace_mode: "text",
            language_id: 96642275,
        }
    }
}
pub struct BASIC;
impl BASIC {
    pub fn info() -> Language {
        Language {
            name: "BASIC",
            type_: "programming",
            color: "#ff0000",
            extensions: &[".bas"],
            tm_scope: "source.basic",
            ace_mode: "text",
            language_id: 28923963,
        }
    }
}
pub struct BQN;
impl BQN {
    pub fn info() -> Language {
        Language {
            name: "BQN",
            type_: "programming",
            color: "#2b7067",
            extensions: &[".bqn"],
            tm_scope: "source.bqn",
            ace_mode: "text",
            language_id: 330386870,
        }
    }
}
pub struct Ballerina;
impl Ballerina {
    pub fn info() -> Language {
        Language {
            name: "Ballerina",
            type_: "programming",
            color: "#FF5000",
            extensions: &[".bal"],
            tm_scope: "source.ballerina",
            ace_mode: "text",
            language_id: 720859680,
        }
    }
}
pub struct Batchfile;
impl Batchfile {
    pub fn info() -> Language {
        Language {
            name: "Batchfile",
            type_: "programming",
            color: "#C1F12E",
            extensions: &[".bat", ".cmd"],
            tm_scope: "source.batchfile",
            ace_mode: "batchfile",
            language_id: 29,
        }
    }
}
pub struct Beef;
impl Beef {
    pub fn info() -> Language {
        Language {
            name: "Beef",
            type_: "programming",
            color: "#a52f4e",
            extensions: &[".bf"],
            tm_scope: "source.cs",
            ace_mode: "csharp",
            language_id: 545626333,
        }
    }
}
pub struct Befunge;
impl Befunge {
    pub fn info() -> Language {
        Language {
            name: "Befunge",
            type_: "programming",
            color: "",
            extensions: &[".befunge", ".bf"],
            tm_scope: "source.befunge",
            ace_mode: "text",
            language_id: 30,
        }
    }
}
pub struct Berry;
impl Berry {
    pub fn info() -> Language {
        Language {
            name: "Berry",
            type_: "programming",
            color: "#15A13C",
            extensions: &[".be"],
            tm_scope: "source.berry",
            ace_mode: "text",
            language_id: 121855308,
        }
    }
}
pub struct BibTeX;
impl BibTeX {
    pub fn info() -> Language {
        Language {
            name: "BibTeX",
            type_: "markup",
            color: "#778899",
            extensions: &[".bib", ".bibtex"],
            tm_scope: "text.bibtex",
            ace_mode: "tex",
            language_id: 982188347,
        }
    }
}
pub struct Bicep;
impl Bicep {
    pub fn info() -> Language {
        Language {
            name: "Bicep",
            type_: "programming",
            color: "#519aba",
            extensions: &[".bicep", ".bicepparam"],
            tm_scope: "source.bicep",
            ace_mode: "text",
            language_id: 321200902,
        }
    }
}
pub struct Bikeshed;
impl Bikeshed {
    pub fn info() -> Language {
        Language {
            name: "Bikeshed",
            type_: "markup",
            color: "#5562ac",
            extensions: &[".bs"],
            tm_scope: "source.csswg",
            ace_mode: "html",
            language_id: 1055528081,
        }
    }
}
pub struct Bison;
impl Bison {
    pub fn info() -> Language {
        Language {
            name: "Bison",
            type_: "programming",
            color: "#6A463F",
            extensions: &[".bison"],
            tm_scope: "source.yacc",
            ace_mode: "text",
            language_id: 31,
        }
    }
}
pub struct BitBake;
impl BitBake {
    pub fn info() -> Language {
        Language {
            name: "BitBake",
            type_: "programming",
            color: "#00bce4",
            extensions: &[".bb", ".bbappend", ".bbclass", ".inc"],
            tm_scope: "source.bb",
            ace_mode: "text",
            language_id: 32,
        }
    }
}
pub struct Blade;
impl Blade {
    pub fn info() -> Language {
        Language {
            name: "Blade",
            type_: "markup",
            color: "#f7523f",
            extensions: &[".blade", ".blade.php"],
            tm_scope: "text.html.php.blade",
            ace_mode: "text",
            language_id: 33,
        }
    }
}
pub struct BlitzBasic;
impl BlitzBasic {
    pub fn info() -> Language {
        Language {
            name: "BlitzBasic",
            type_: "programming",
            color: "#00FFAE",
            extensions: &[".bb", ".decls"],
            tm_scope: "source.blitzmax",
            ace_mode: "text",
            language_id: 34,
        }
    }
}
pub struct BlitzMax;
impl BlitzMax {
    pub fn info() -> Language {
        Language {
            name: "BlitzMax",
            type_: "programming",
            color: "#cd6400",
            extensions: &[".bmx"],
            tm_scope: "source.blitzmax",
            ace_mode: "text",
            language_id: 35,
        }
    }
}
pub struct Bluespec;
impl Bluespec {
    pub fn info() -> Language {
        Language {
            name: "Bluespec",
            type_: "programming",
            color: "#12223c",
            extensions: &[".bsv"],
            tm_scope: "source.bsv",
            ace_mode: "verilog",
            language_id: 36,
        }
    }
}
pub struct BluespecBH;
impl BluespecBH {
    pub fn info() -> Language {
        Language {
            name: "Bluespec BH",
            type_: "programming",
            color: "#12223c",
            extensions: &[".bs"],
            tm_scope: "source.bh",
            ace_mode: "haskell",
            language_id: 641580358,
        }
    }
}
pub struct Boo;
impl Boo {
    pub fn info() -> Language {
        Language {
            name: "Boo",
            type_: "programming",
            color: "#d4bec1",
            extensions: &[".boo"],
            tm_scope: "source.boo",
            ace_mode: "text",
            language_id: 37,
        }
    }
}
pub struct Boogie;
impl Boogie {
    pub fn info() -> Language {
        Language {
            name: "Boogie",
            type_: "programming",
            color: "#c80fa0",
            extensions: &[".bpl"],
            tm_scope: "source.boogie",
            ace_mode: "text",
            language_id: 955017407,
        }
    }
}
pub struct Brainfuck;
impl Brainfuck {
    pub fn info() -> Language {
        Language {
            name: "Brainfuck",
            type_: "programming",
            color: "#2F2530",
            extensions: &[".b", ".bf"],
            tm_scope: "source.bf",
            ace_mode: "text",
            language_id: 38,
        }
    }
}
pub struct BrighterScript;
impl BrighterScript {
    pub fn info() -> Language {
        Language {
            name: "BrighterScript",
            type_: "programming",
            color: "#66AABB",
            extensions: &[".bs"],
            tm_scope: "source.brs",
            ace_mode: "text",
            language_id: 943571030,
        }
    }
}
pub struct Brightscript;
impl Brightscript {
    pub fn info() -> Language {
        Language {
            name: "Brightscript",
            type_: "programming",
            color: "#662D91",
            extensions: &[".brs"],
            tm_scope: "source.brs",
            ace_mode: "text",
            language_id: 39,
        }
    }
}
pub struct Browserslist;
impl Browserslist {
    pub fn info() -> Language {
        Language {
            name: "Browserslist",
            type_: "data",
            color: "#ffd539",
            extensions: &[],
            tm_scope: "text.browserslist",
            ace_mode: "text",
            language_id: 153503348,
        }
    }
}
pub struct C;
impl C {
    pub fn info() -> Language {
        Language {
            name: "C",
            type_: "programming",
            color: "#555555",
            extensions: &[".c", ".cats", ".h", ".idc"],
            tm_scope: "source.c",
            ace_mode: "c_cpp",
            language_id: 41,
        }
    }
}
pub struct Csharp;
impl Csharp {
    pub fn info() -> Language {
        Language {
            name: "C#",
            type_: "programming",
            color: "#178600",
            extensions: &[".cs", ".cake", ".cs.pp", ".csx", ".linq"],
            tm_scope: "source.cs",
            ace_mode: "csharp",
            language_id: 42,
        }
    }
}
pub struct Cpp;
impl Cpp {
    pub fn info() -> Language {
        Language {
            name: "C++",
            type_: "programming",
            color: "#f34b7d",
            extensions: &[
                ".cpp",
                ".c++",
                ".cc",
                ".cp",
                ".cppm",
                ".cxx",
                ".h",
                ".h++",
                ".hh",
                ".hpp",
                ".hxx",
                ".inc",
                ".inl",
                ".ino",
                ".ipp",
                ".ixx",
                ".re",
                ".tcc",
                ".tpp",
                ".txx",
            ],
            tm_scope: "source.c++",
            ace_mode: "c_cpp",
            language_id: 43,
        }
    }
}
pub struct CObjDump;
impl CObjDump {
    pub fn info() -> Language {
        Language {
            name: "C-ObjDump",
            type_: "data",
            color: "",
            extensions: &[".c-objdump"],
            tm_scope: "objdump.x86asm",
            ace_mode: "assembly_x86",
            language_id: 44,
        }
    }
}
pub struct C2hsHaskell;
impl C2hsHaskell {
    pub fn info() -> Language {
        Language {
            name: "C2hs Haskell",
            type_: "programming",
            color: "",
            extensions: &[".chs"],
            tm_scope: "source.haskell",
            ace_mode: "haskell",
            language_id: 45,
        }
    }
}
pub struct CAPCDS;
impl CAPCDS {
    pub fn info() -> Language {
        Language {
            name: "CAP CDS",
            type_: "programming",
            color: "#0092d1",
            extensions: &[".cds"],
            tm_scope: "source.cds",
            ace_mode: "text",
            language_id: 390788699,
        }
    }
}
pub struct CIL;
impl CIL {
    pub fn info() -> Language {
        Language {
            name: "CIL",
            type_: "data",
            color: "",
            extensions: &[".cil"],
            tm_scope: "source.cil",
            ace_mode: "text",
            language_id: 29176339,
        }
    }
}
pub struct CLIPS;
impl CLIPS {
    pub fn info() -> Language {
        Language {
            name: "CLIPS",
            type_: "programming",
            color: "#00A300",
            extensions: &[".clp"],
            tm_scope: "source.clips",
            ace_mode: "text",
            language_id: 46,
        }
    }
}
pub struct CMake;
impl CMake {
    pub fn info() -> Language {
        Language {
            name: "CMake",
            type_: "programming",
            color: "#DA3434",
            extensions: &[".cmake", ".cmake.in"],
            tm_scope: "source.cmake",
            ace_mode: "text",
            language_id: 47,
        }
    }
}
pub struct COBOL;
impl COBOL {
    pub fn info() -> Language {
        Language {
            name: "COBOL",
            type_: "programming",
            color: "",
            extensions: &[".cob", ".cbl", ".ccp", ".cobol", ".cpy"],
            tm_scope: "source.cobol",
            ace_mode: "cobol",
            language_id: 48,
        }
    }
}
pub struct CODEOWNERS;
impl CODEOWNERS {
    pub fn info() -> Language {
        Language {
            name: "CODEOWNERS",
            type_: "data",
            color: "",
            extensions: &[],
            tm_scope: "text.codeowners",
            ace_mode: "gitignore",
            language_id: 321684729,
        }
    }
}
pub struct COLLADA;
impl COLLADA {
    pub fn info() -> Language {
        Language {
            name: "COLLADA",
            type_: "data",
            color: "#F1A42B",
            extensions: &[".dae"],
            tm_scope: "text.xml",
            ace_mode: "xml",
            language_id: 49,
        }
    }
}
pub struct CSON;
impl CSON {
    pub fn info() -> Language {
        Language {
            name: "CSON",
            type_: "data",
            color: "#244776",
            extensions: &[".cson"],
            tm_scope: "source.coffee",
            ace_mode: "coffee",
            language_id: 424,
        }
    }
}
pub struct CSS;
impl CSS {
    pub fn info() -> Language {
        Language {
            name: "CSS",
            type_: "markup",
            color: "#563d7c",
            extensions: &[".css"],
            tm_scope: "source.css",
            ace_mode: "css",
            language_id: 50,
        }
    }
}
pub struct CSV;
impl CSV {
    pub fn info() -> Language {
        Language {
            name: "CSV",
            type_: "data",
            color: "#237346",
            extensions: &[".csv"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 51,
        }
    }
}
pub struct CUE;
impl CUE {
    pub fn info() -> Language {
        Language {
            name: "CUE",
            type_: "programming",
            color: "#5886E1",
            extensions: &[".cue"],
            tm_scope: "source.cue",
            ace_mode: "text",
            language_id: 356063509,
        }
    }
}
pub struct CWeb;
impl CWeb {
    pub fn info() -> Language {
        Language {
            name: "CWeb",
            type_: "programming",
            color: "#00007a",
            extensions: &[".w"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 657332628,
        }
    }
}
pub struct CabalConfig;
impl CabalConfig {
    pub fn info() -> Language {
        Language {
            name: "Cabal Config",
            type_: "data",
            color: "#483465",
            extensions: &[".cabal"],
            tm_scope: "source.cabal",
            ace_mode: "haskell",
            language_id: 677095381,
        }
    }
}
pub struct Caddyfile;
impl Caddyfile {
    pub fn info() -> Language {
        Language {
            name: "Caddyfile",
            type_: "data",
            color: "#22b638",
            extensions: &[".caddyfile"],
            tm_scope: "source.Caddyfile",
            ace_mode: "text",
            language_id: 615465151,
        }
    }
}
pub struct Cadence;
impl Cadence {
    pub fn info() -> Language {
        Language {
            name: "Cadence",
            type_: "programming",
            color: "#00ef8b",
            extensions: &[".cdc"],
            tm_scope: "source.cadence",
            ace_mode: "text",
            language_id: 270184138,
        }
    }
}
pub struct Cairo;
impl Cairo {
    pub fn info() -> Language {
        Language {
            name: "Cairo",
            type_: "programming",
            color: "#ff4a48",
            extensions: &[".cairo"],
            tm_scope: "source.cairo",
            ace_mode: "text",
            language_id: 620599567,
        }
    }
}
pub struct CairoZero;
impl CairoZero {
    pub fn info() -> Language {
        Language {
            name: "Cairo Zero",
            type_: "programming",
            color: "#ff4a48",
            extensions: &[".cairo"],
            tm_scope: "source.cairo0",
            ace_mode: "text",
            language_id: 891399890,
        }
    }
}
pub struct CameLIGO;
impl CameLIGO {
    pub fn info() -> Language {
        Language {
            name: "CameLIGO",
            type_: "programming",
            color: "#3be133",
            extensions: &[".mligo"],
            tm_scope: "source.mligo",
            ace_mode: "ocaml",
            language_id: 829207807,
        }
    }
}
pub struct CapnProto;
impl CapnProto {
    pub fn info() -> Language {
        Language {
            name: "Cap'n Proto",
            type_: "programming",
            color: "#c42727",
            extensions: &[".capnp"],
            tm_scope: "source.capnp",
            ace_mode: "text",
            language_id: 52,
        }
    }
}
pub struct Carbon;
impl Carbon {
    pub fn info() -> Language {
        Language {
            name: "Carbon",
            type_: "programming",
            color: "#222222",
            extensions: &[".carbon"],
            tm_scope: "source.v",
            ace_mode: "golang",
            language_id: 55627273,
        }
    }
}
pub struct CartoCSS;
impl CartoCSS {
    pub fn info() -> Language {
        Language {
            name: "CartoCSS",
            type_: "programming",
            color: "",
            extensions: &[".mss"],
            tm_scope: "source.css.mss",
            ace_mode: "text",
            language_id: 53,
        }
    }
}
pub struct Ceylon;
impl Ceylon {
    pub fn info() -> Language {
        Language {
            name: "Ceylon",
            type_: "programming",
            color: "#dfa535",
            extensions: &[".ceylon"],
            tm_scope: "source.ceylon",
            ace_mode: "text",
            language_id: 54,
        }
    }
}
pub struct Chapel;
impl Chapel {
    pub fn info() -> Language {
        Language {
            name: "Chapel",
            type_: "programming",
            color: "#8dc63f",
            extensions: &[".chpl"],
            tm_scope: "source.chapel",
            ace_mode: "text",
            language_id: 55,
        }
    }
}
pub struct Charity;
impl Charity {
    pub fn info() -> Language {
        Language {
            name: "Charity",
            type_: "programming",
            color: "",
            extensions: &[".ch"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 56,
        }
    }
}
pub struct Checksums;
impl Checksums {
    pub fn info() -> Language {
        Language {
            name: "Checksums",
            type_: "data",
            color: "",
            extensions: &[
                ".crc32",
                ".md2",
                ".md4",
                ".md5",
                ".sha1",
                ".sha2",
                ".sha224",
                ".sha256",
                ".sha256sum",
                ".sha3",
                ".sha384",
                ".sha512",
            ],
            tm_scope: "text.checksums",
            ace_mode: "text",
            language_id: 372063053,
        }
    }
}
pub struct ChucK;
impl ChucK {
    pub fn info() -> Language {
        Language {
            name: "ChucK",
            type_: "programming",
            color: "#3f8000",
            extensions: &[".ck"],
            tm_scope: "source.java",
            ace_mode: "java",
            language_id: 57,
        }
    }
}
pub struct Circom;
impl Circom {
    pub fn info() -> Language {
        Language {
            name: "Circom",
            type_: "programming",
            color: "#707575",
            extensions: &[".circom"],
            tm_scope: "source.circom",
            ace_mode: "text",
            language_id: 1042332086,
        }
    }
}
pub struct Cirru;
impl Cirru {
    pub fn info() -> Language {
        Language {
            name: "Cirru",
            type_: "programming",
            color: "#ccccff",
            extensions: &[".cirru"],
            tm_scope: "source.cirru",
            ace_mode: "cirru",
            language_id: 58,
        }
    }
}
pub struct Clarion;
impl Clarion {
    pub fn info() -> Language {
        Language {
            name: "Clarion",
            type_: "programming",
            color: "#db901e",
            extensions: &[".clw"],
            tm_scope: "source.clarion",
            ace_mode: "text",
            language_id: 59,
        }
    }
}
pub struct Clarity;
impl Clarity {
    pub fn info() -> Language {
        Language {
            name: "Clarity",
            type_: "programming",
            color: "#5546ff",
            extensions: &[".clar"],
            tm_scope: "source.clar",
            ace_mode: "lisp",
            language_id: 91493841,
        }
    }
}
pub struct ClassicASP;
impl ClassicASP {
    pub fn info() -> Language {
        Language {
            name: "Classic ASP",
            type_: "programming",
            color: "#6a40fd",
            extensions: &[".asp"],
            tm_scope: "text.html.asp",
            ace_mode: "text",
            language_id: 8,
        }
    }
}
pub struct Clean;
impl Clean {
    pub fn info() -> Language {
        Language {
            name: "Clean",
            type_: "programming",
            color: "#3F85AF",
            extensions: &[".icl", ".dcl"],
            tm_scope: "source.clean",
            ace_mode: "text",
            language_id: 60,
        }
    }
}
pub struct Click;
impl Click {
    pub fn info() -> Language {
        Language {
            name: "Click",
            type_: "programming",
            color: "#E4E6F3",
            extensions: &[".click"],
            tm_scope: "source.click",
            ace_mode: "text",
            language_id: 61,
        }
    }
}
pub struct Clojure;
impl Clojure {
    pub fn info() -> Language {
        Language {
            name: "Clojure",
            type_: "programming",
            color: "#db5855",
            extensions: &[
                ".clj",
                ".bb",
                ".boot",
                ".cl2",
                ".cljc",
                ".cljs",
                ".cljs.hl",
                ".cljscm",
                ".cljx",
                ".hic",
            ],
            tm_scope: "source.clojure",
            ace_mode: "clojure",
            language_id: 62,
        }
    }
}
pub struct ClosureTemplates;
impl ClosureTemplates {
    pub fn info() -> Language {
        Language {
            name: "Closure Templates",
            type_: "markup",
            color: "#0d948f",
            extensions: &[".soy"],
            tm_scope: "text.html.soy",
            ace_mode: "soy_template",
            language_id: 357046146,
        }
    }
}
pub struct CloudFirestoreSecurityRules;
impl CloudFirestoreSecurityRules {
    pub fn info() -> Language {
        Language {
            name: "Cloud Firestore Security Rules",
            type_: "data",
            color: "#FFA000",
            extensions: &[],
            tm_scope: "source.firestore",
            ace_mode: "less",
            language_id: 407996372,
        }
    }
}
pub struct CoNLLU;
impl CoNLLU {
    pub fn info() -> Language {
        Language {
            name: "CoNLL-U",
            type_: "data",
            color: "",
            extensions: &[".conllu", ".conll"],
            tm_scope: "text.conllu",
            ace_mode: "text",
            language_id: 421026389,
        }
    }
}
pub struct CodeQL;
impl CodeQL {
    pub fn info() -> Language {
        Language {
            name: "CodeQL",
            type_: "programming",
            color: "#140f46",
            extensions: &[".ql", ".qll"],
            tm_scope: "source.ql",
            ace_mode: "text",
            language_id: 424259634,
        }
    }
}
pub struct CoffeeScript;
impl CoffeeScript {
    pub fn info() -> Language {
        Language {
            name: "CoffeeScript",
            type_: "programming",
            color: "#244776",
            extensions: &[".coffee", "._coffee", ".cake", ".cjsx", ".iced"],
            tm_scope: "source.coffee",
            ace_mode: "coffee",
            language_id: 63,
        }
    }
}
pub struct ColdFusion;
impl ColdFusion {
    pub fn info() -> Language {
        Language {
            name: "ColdFusion",
            type_: "programming",
            color: "#ed2cd6",
            extensions: &[".cfm", ".cfml"],
            tm_scope: "text.html.cfm",
            ace_mode: "coldfusion",
            language_id: 64,
        }
    }
}
pub struct ColdFusionCFC;
impl ColdFusionCFC {
    pub fn info() -> Language {
        Language {
            name: "ColdFusion CFC",
            type_: "programming",
            color: "#ed2cd6",
            extensions: &[".cfc"],
            tm_scope: "source.cfscript",
            ace_mode: "coldfusion",
            language_id: 65,
        }
    }
}
pub struct CommonLisp;
impl CommonLisp {
    pub fn info() -> Language {
        Language {
            name: "Common Lisp",
            type_: "programming",
            color: "#3fb68b",
            extensions: &[
                ".lisp",
                ".asd",
                ".cl",
                ".l",
                ".lsp",
                ".ny",
                ".podsl",
                ".sexp",
            ],
            tm_scope: "source.commonlisp",
            ace_mode: "lisp",
            language_id: 66,
        }
    }
}
pub struct CommonWorkflowLanguage;
impl CommonWorkflowLanguage {
    pub fn info() -> Language {
        Language {
            name: "Common Workflow Language",
            type_: "programming",
            color: "#B5314C",
            extensions: &[".cwl"],
            tm_scope: "source.cwl",
            ace_mode: "yaml",
            language_id: 988547172,
        }
    }
}
pub struct ComponentPascal;
impl ComponentPascal {
    pub fn info() -> Language {
        Language {
            name: "Component Pascal",
            type_: "programming",
            color: "#B0CE4E",
            extensions: &[".cp", ".cps"],
            tm_scope: "source.pascal",
            ace_mode: "pascal",
            language_id: 67,
        }
    }
}
pub struct Cool;
impl Cool {
    pub fn info() -> Language {
        Language {
            name: "Cool",
            type_: "programming",
            color: "",
            extensions: &[".cl"],
            tm_scope: "source.cool",
            ace_mode: "text",
            language_id: 68,
        }
    }
}
pub struct Coq;
impl Coq {
    pub fn info() -> Language {
        Language {
            name: "Coq",
            type_: "programming",
            color: "#d0b68c",
            extensions: &[".coq", ".v"],
            tm_scope: "source.coq",
            ace_mode: "text",
            language_id: 69,
        }
    }
}
pub struct CppObjDump;
impl CppObjDump {
    pub fn info() -> Language {
        Language {
            name: "Cpp-ObjDump",
            type_: "data",
            color: "",
            extensions: &[
                ".cppobjdump",
                ".c++-objdump",
                ".c++objdump",
                ".cpp-objdump",
                ".cxx-objdump",
            ],
            tm_scope: "objdump.x86asm",
            ace_mode: "assembly_x86",
            language_id: 70,
        }
    }
}
pub struct Creole;
impl Creole {
    pub fn info() -> Language {
        Language {
            name: "Creole",
            type_: "prose",
            color: "",
            extensions: &[".creole"],
            tm_scope: "text.html.creole",
            ace_mode: "text",
            language_id: 71,
        }
    }
}
pub struct Crystal;
impl Crystal {
    pub fn info() -> Language {
        Language {
            name: "Crystal",
            type_: "programming",
            color: "#000100",
            extensions: &[".cr"],
            tm_scope: "source.crystal",
            ace_mode: "ruby",
            language_id: 72,
        }
    }
}
pub struct Csound;
impl Csound {
    pub fn info() -> Language {
        Language {
            name: "Csound",
            type_: "programming",
            color: "#1a1a1a",
            extensions: &[".orc", ".udo"],
            tm_scope: "source.csound",
            ace_mode: "csound_orchestra",
            language_id: 73,
        }
    }
}
pub struct CsoundDocument;
impl CsoundDocument {
    pub fn info() -> Language {
        Language {
            name: "Csound Document",
            type_: "programming",
            color: "#1a1a1a",
            extensions: &[".csd"],
            tm_scope: "source.csound-document",
            ace_mode: "csound_document",
            language_id: 74,
        }
    }
}
pub struct CsoundScore;
impl CsoundScore {
    pub fn info() -> Language {
        Language {
            name: "Csound Score",
            type_: "programming",
            color: "#1a1a1a",
            extensions: &[".sco"],
            tm_scope: "source.csound-score",
            ace_mode: "csound_score",
            language_id: 75,
        }
    }
}
pub struct Cuda;
impl Cuda {
    pub fn info() -> Language {
        Language {
            name: "Cuda",
            type_: "programming",
            color: "#3A4E3A",
            extensions: &[".cu", ".cuh"],
            tm_scope: "source.cuda-c++",
            ace_mode: "c_cpp",
            language_id: 77,
        }
    }
}
pub struct CueSheet;
impl CueSheet {
    pub fn info() -> Language {
        Language {
            name: "Cue Sheet",
            type_: "data",
            color: "",
            extensions: &[".cue"],
            tm_scope: "source.cuesheet",
            ace_mode: "text",
            language_id: 942714150,
        }
    }
}
pub struct Curry;
impl Curry {
    pub fn info() -> Language {
        Language {
            name: "Curry",
            type_: "programming",
            color: "#531242",
            extensions: &[".curry"],
            tm_scope: "source.curry",
            ace_mode: "haskell",
            language_id: 439829048,
        }
    }
}
pub struct Cycript;
impl Cycript {
    pub fn info() -> Language {
        Language {
            name: "Cycript",
            type_: "programming",
            color: "",
            extensions: &[".cy"],
            tm_scope: "source.js",
            ace_mode: "javascript",
            language_id: 78,
        }
    }
}
pub struct Cylc;
impl Cylc {
    pub fn info() -> Language {
        Language {
            name: "Cylc",
            type_: "data",
            color: "#00b3fd",
            extensions: &[".cylc"],
            tm_scope: "source.cylc",
            ace_mode: "ini",
            language_id: 476447814,
        }
    }
}
pub struct Cypher;
impl Cypher {
    pub fn info() -> Language {
        Language {
            name: "Cypher",
            type_: "programming",
            color: "#34c0eb",
            extensions: &[".cyp", ".cypher"],
            tm_scope: "source.cypher",
            ace_mode: "text",
            language_id: 850806976,
        }
    }
}
pub struct Cython;
impl Cython {
    pub fn info() -> Language {
        Language {
            name: "Cython",
            type_: "programming",
            color: "#fedf5b",
            extensions: &[".pyx", ".pxd", ".pxi"],
            tm_scope: "source.cython",
            ace_mode: "text",
            language_id: 79,
        }
    }
}
pub struct D;
impl D {
    pub fn info() -> Language {
        Language {
            name: "D",
            type_: "programming",
            color: "#ba595e",
            extensions: &[".d", ".di"],
            tm_scope: "source.d",
            ace_mode: "d",
            language_id: 80,
        }
    }
}
pub struct DObjDump;
impl DObjDump {
    pub fn info() -> Language {
        Language {
            name: "D-ObjDump",
            type_: "data",
            color: "",
            extensions: &[".d-objdump"],
            tm_scope: "objdump.x86asm",
            ace_mode: "assembly_x86",
            language_id: 81,
        }
    }
}
pub struct D2;
impl D2 {
    pub fn info() -> Language {
        Language {
            name: "D2",
            type_: "markup",
            color: "#526ee8",
            extensions: &[".d2"],
            tm_scope: "source.d2",
            ace_mode: "text",
            language_id: 37531557,
        }
    }
}
pub struct DIGITALCommandLanguage;
impl DIGITALCommandLanguage {
    pub fn info() -> Language {
        Language {
            name: "DIGITAL Command Language",
            type_: "programming",
            color: "",
            extensions: &[".com"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 82,
        }
    }
}
pub struct DM;
impl DM {
    pub fn info() -> Language {
        Language {
            name: "DM",
            type_: "programming",
            color: "#447265",
            extensions: &[".dm"],
            tm_scope: "source.dm",
            ace_mode: "c_cpp",
            language_id: 83,
        }
    }
}
pub struct DNSZone;
impl DNSZone {
    pub fn info() -> Language {
        Language {
            name: "DNS Zone",
            type_: "data",
            color: "",
            extensions: &[".zone", ".arpa"],
            tm_scope: "text.zone_file",
            ace_mode: "text",
            language_id: 84,
        }
    }
}
pub struct DTrace;
impl DTrace {
    pub fn info() -> Language {
        Language {
            name: "DTrace",
            type_: "programming",
            color: "",
            extensions: &[".d"],
            tm_scope: "source.c",
            ace_mode: "c_cpp",
            language_id: 85,
        }
    }
}
pub struct Dafny;
impl Dafny {
    pub fn info() -> Language {
        Language {
            name: "Dafny",
            type_: "programming",
            color: "#FFEC25",
            extensions: &[".dfy"],
            tm_scope: "text.dfy.dafny",
            ace_mode: "text",
            language_id: 969323346,
        }
    }
}
pub struct DarcsPatch;
impl DarcsPatch {
    pub fn info() -> Language {
        Language {
            name: "Darcs Patch",
            type_: "data",
            color: "#8eff23",
            extensions: &[".darcspatch", ".dpatch"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 86,
        }
    }
}
pub struct Dart;
impl Dart {
    pub fn info() -> Language {
        Language {
            name: "Dart",
            type_: "programming",
            color: "#00B4AB",
            extensions: &[".dart"],
            tm_scope: "source.dart",
            ace_mode: "dart",
            language_id: 87,
        }
    }
}
pub struct DataWeave;
impl DataWeave {
    pub fn info() -> Language {
        Language {
            name: "DataWeave",
            type_: "programming",
            color: "#003a52",
            extensions: &[".dwl"],
            tm_scope: "source.data-weave",
            ace_mode: "text",
            language_id: 974514097,
        }
    }
}
pub struct DebianPackageControlFile;
impl DebianPackageControlFile {
    pub fn info() -> Language {
        Language {
            name: "Debian Package Control File",
            type_: "data",
            color: "#D70751",
            extensions: &[".dsc"],
            tm_scope: "source.deb-control",
            ace_mode: "text",
            language_id: 527438264,
        }
    }
}
pub struct DenizenScript;
impl DenizenScript {
    pub fn info() -> Language {
        Language {
            name: "DenizenScript",
            type_: "programming",
            color: "#FBEE96",
            extensions: &[".dsc"],
            tm_scope: "source.denizenscript",
            ace_mode: "yaml",
            language_id: 435000929,
        }
    }
}
pub struct Dhall;
impl Dhall {
    pub fn info() -> Language {
        Language {
            name: "Dhall",
            type_: "programming",
            color: "#dfafff",
            extensions: &[".dhall"],
            tm_scope: "source.haskell",
            ace_mode: "haskell",
            language_id: 793969321,
        }
    }
}
pub struct Diff;
impl Diff {
    pub fn info() -> Language {
        Language {
            name: "Diff",
            type_: "data",
            color: "",
            extensions: &[".diff", ".patch"],
            tm_scope: "source.diff",
            ace_mode: "diff",
            language_id: 88,
        }
    }
}
pub struct DirectX3DFile;
impl DirectX3DFile {
    pub fn info() -> Language {
        Language {
            name: "DirectX 3D File",
            type_: "data",
            color: "#aace60",
            extensions: &[".x"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 201049282,
        }
    }
}
pub struct Dockerfile;
impl Dockerfile {
    pub fn info() -> Language {
        Language {
            name: "Dockerfile",
            type_: "programming",
            color: "#384d54",
            extensions: &[".dockerfile"],
            tm_scope: "source.dockerfile",
            ace_mode: "dockerfile",
            language_id: 89,
        }
    }
}
pub struct Dogescript;
impl Dogescript {
    pub fn info() -> Language {
        Language {
            name: "Dogescript",
            type_: "programming",
            color: "#cca760",
            extensions: &[".djs"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 90,
        }
    }
}
pub struct Dotenv;
impl Dotenv {
    pub fn info() -> Language {
        Language {
            name: "Dotenv",
            type_: "data",
            color: "#e5d559",
            extensions: &[".env"],
            tm_scope: "source.dotenv",
            ace_mode: "text",
            language_id: 111148035,
        }
    }
}
pub struct Dune;
impl Dune {
    pub fn info() -> Language {
        Language {
            name: "Dune",
            type_: "programming",
            color: "#89421e",
            extensions: &[],
            tm_scope: "source.dune",
            ace_mode: "lisp",
            language_id: 754574151,
        }
    }
}
pub struct Dylan;
impl Dylan {
    pub fn info() -> Language {
        Language {
            name: "Dylan",
            type_: "programming",
            color: "#6c616e",
            extensions: &[".dylan", ".dyl", ".intr", ".lid"],
            tm_scope: "source.dylan",
            ace_mode: "text",
            language_id: 91,
        }
    }
}
pub struct E;
impl E {
    pub fn info() -> Language {
        Language {
            name: "E",
            type_: "programming",
            color: "#ccce35",
            extensions: &[".e"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 92,
        }
    }
}
pub struct EMail;
impl EMail {
    pub fn info() -> Language {
        Language {
            name: "E-mail",
            type_: "data",
            color: "",
            extensions: &[".eml", ".mbox"],
            tm_scope: "text.eml.basic",
            ace_mode: "text",
            language_id: 529653389,
        }
    }
}
pub struct EBNF;
impl EBNF {
    pub fn info() -> Language {
        Language {
            name: "EBNF",
            type_: "data",
            color: "",
            extensions: &[".ebnf"],
            tm_scope: "source.ebnf",
            ace_mode: "text",
            language_id: 430,
        }
    }
}
pub struct ECL;
impl ECL {
    pub fn info() -> Language {
        Language {
            name: "ECL",
            type_: "programming",
            color: "#8a1267",
            extensions: &[".ecl", ".eclxml"],
            tm_scope: "source.ecl",
            ace_mode: "text",
            language_id: 93,
        }
    }
}
pub struct ECLiPSe;
impl ECLiPSe {
    pub fn info() -> Language {
        Language {
            name: "ECLiPSe",
            type_: "programming",
            color: "#001d9d",
            extensions: &[".ecl"],
            tm_scope: "source.prolog.eclipse",
            ace_mode: "prolog",
            language_id: 94,
        }
    }
}
pub struct EJS;
impl EJS {
    pub fn info() -> Language {
        Language {
            name: "EJS",
            type_: "markup",
            color: "#a91e50",
            extensions: &[".ejs", ".ect", ".ejs.t", ".jst"],
            tm_scope: "text.html.js",
            ace_mode: "ejs",
            language_id: 95,
        }
    }
}
pub struct EQ;
impl EQ {
    pub fn info() -> Language {
        Language {
            name: "EQ",
            type_: "programming",
            color: "#a78649",
            extensions: &[".eq"],
            tm_scope: "source.cs",
            ace_mode: "csharp",
            language_id: 96,
        }
    }
}
pub struct Eagle;
impl Eagle {
    pub fn info() -> Language {
        Language {
            name: "Eagle",
            type_: "data",
            color: "",
            extensions: &[".sch", ".brd"],
            tm_scope: "text.xml",
            ace_mode: "xml",
            language_id: 97,
        }
    }
}
pub struct Earthly;
impl Earthly {
    pub fn info() -> Language {
        Language {
            name: "Earthly",
            type_: "programming",
            color: "#2af0ff",
            extensions: &[],
            tm_scope: "source.earthfile",
            ace_mode: "text",
            language_id: 963512632,
        }
    }
}
pub struct Easybuild;
impl Easybuild {
    pub fn info() -> Language {
        Language {
            name: "Easybuild",
            type_: "data",
            color: "#069406",
            extensions: &[".eb"],
            tm_scope: "source.python",
            ace_mode: "python",
            language_id: 342840477,
        }
    }
}
pub struct EcereProjects;
impl EcereProjects {
    pub fn info() -> Language {
        Language {
            name: "Ecere Projects",
            type_: "data",
            color: "#913960",
            extensions: &[".epj"],
            tm_scope: "source.json",
            ace_mode: "json",
            language_id: 98,
        }
    }
}
pub struct Ecmarkup;
impl Ecmarkup {
    pub fn info() -> Language {
        Language {
            name: "Ecmarkup",
            type_: "markup",
            color: "#eb8131",
            extensions: &[".html"],
            tm_scope: "text.html.ecmarkup",
            ace_mode: "html",
            language_id: 844766630,
        }
    }
}
pub struct Edge;
impl Edge {
    pub fn info() -> Language {
        Language {
            name: "Edge",
            type_: "markup",
            color: "#0dffe0",
            extensions: &[".edge"],
            tm_scope: "text.html.edge",
            ace_mode: "html",
            language_id: 460509620,
        }
    }
}
pub struct EdgeQL;
impl EdgeQL {
    pub fn info() -> Language {
        Language {
            name: "EdgeQL",
            type_: "programming",
            color: "#31A7FF",
            extensions: &[".edgeql", ".esdl"],
            tm_scope: "source.edgeql",
            ace_mode: "text",
            language_id: 925235833,
        }
    }
}
pub struct EditorConfig;
impl EditorConfig {
    pub fn info() -> Language {
        Language {
            name: "EditorConfig",
            type_: "data",
            color: "#fff1f2",
            extensions: &[".editorconfig"],
            tm_scope: "source.editorconfig",
            ace_mode: "ini",
            language_id: 96139566,
        }
    }
}
pub struct EdjeDataCollection;
impl EdjeDataCollection {
    pub fn info() -> Language {
        Language {
            name: "Edje Data Collection",
            type_: "data",
            color: "",
            extensions: &[".edc"],
            tm_scope: "source.c++",
            ace_mode: "c_cpp",
            language_id: 342840478,
        }
    }
}
pub struct Eiffel;
impl Eiffel {
    pub fn info() -> Language {
        Language {
            name: "Eiffel",
            type_: "programming",
            color: "#4d6977",
            extensions: &[".e"],
            tm_scope: "source.eiffel",
            ace_mode: "eiffel",
            language_id: 99,
        }
    }
}
pub struct Elixir;
impl Elixir {
    pub fn info() -> Language {
        Language {
            name: "Elixir",
            type_: "programming",
            color: "#6e4a7e",
            extensions: &[".ex", ".exs"],
            tm_scope: "source.elixir",
            ace_mode: "elixir",
            language_id: 100,
        }
    }
}
pub struct Elm;
impl Elm {
    pub fn info() -> Language {
        Language {
            name: "Elm",
            type_: "programming",
            color: "#60B5CC",
            extensions: &[".elm"],
            tm_scope: "source.elm",
            ace_mode: "elm",
            language_id: 101,
        }
    }
}
pub struct Elvish;
impl Elvish {
    pub fn info() -> Language {
        Language {
            name: "Elvish",
            type_: "programming",
            color: "#55BB55",
            extensions: &[".elv"],
            tm_scope: "source.elvish",
            ace_mode: "text",
            language_id: 570996448,
        }
    }
}
pub struct ElvishTranscript;
impl ElvishTranscript {
    pub fn info() -> Language {
        Language {
            name: "Elvish Transcript",
            type_: "programming",
            color: "#55BB55",
            extensions: &[],
            tm_scope: "source.elvish-transcript",
            ace_mode: "text",
            language_id: 452025714,
        }
    }
}
pub struct EmacsLisp;
impl EmacsLisp {
    pub fn info() -> Language {
        Language {
            name: "Emacs Lisp",
            type_: "programming",
            color: "#c065db",
            extensions: &[".el", ".emacs", ".emacs.desktop"],
            tm_scope: "source.emacs.lisp",
            ace_mode: "lisp",
            language_id: 102,
        }
    }
}
pub struct EmberScript;
impl EmberScript {
    pub fn info() -> Language {
        Language {
            name: "EmberScript",
            type_: "programming",
            color: "#FFF4F3",
            extensions: &[".em", ".emberscript"],
            tm_scope: "source.coffee",
            ace_mode: "coffee",
            language_id: 103,
        }
    }
}
pub struct Erlang;
impl Erlang {
    pub fn info() -> Language {
        Language {
            name: "Erlang",
            type_: "programming",
            color: "#B83998",
            extensions: &[
                ".erl",
                ".app",
                ".app.src",
                ".es",
                ".escript",
                ".hrl",
                ".xrl",
                ".yrl",
            ],
            tm_scope: "source.erlang",
            ace_mode: "erlang",
            language_id: 104,
        }
    }
}
pub struct Euphoria;
impl Euphoria {
    pub fn info() -> Language {
        Language {
            name: "Euphoria",
            type_: "programming",
            color: "#FF790B",
            extensions: &[".e", ".ex"],
            tm_scope: "source.euphoria",
            ace_mode: "text",
            language_id: 880693982,
        }
    }
}
pub struct Fsharp;
impl Fsharp {
    pub fn info() -> Language {
        Language {
            name: "F#",
            type_: "programming",
            color: "#b845fc",
            extensions: &[".fs", ".fsi", ".fsx"],
            tm_scope: "source.fsharp",
            ace_mode: "text",
            language_id: 105,
        }
    }
}
pub struct Fstar;
impl Fstar {
    pub fn info() -> Language {
        Language {
            name: "F*",
            type_: "programming",
            color: "#572e30",
            extensions: &[".fst", ".fsti"],
            tm_scope: "source.fstar",
            ace_mode: "text",
            language_id: 336943375,
        }
    }
}
pub struct FIGletFont;
impl FIGletFont {
    pub fn info() -> Language {
        Language {
            name: "FIGlet Font",
            type_: "data",
            color: "#FFDDBB",
            extensions: &[".flf"],
            tm_scope: "source.figfont",
            ace_mode: "text",
            language_id: 686129783,
        }
    }
}
pub struct FIRRTL;
impl FIRRTL {
    pub fn info() -> Language {
        Language {
            name: "FIRRTL",
            type_: "programming",
            color: "#2f632f",
            extensions: &[".fir"],
            tm_scope: "source.firrtl",
            ace_mode: "text",
            language_id: 906694254,
        }
    }
}
pub struct FLUX;
impl FLUX {
    pub fn info() -> Language {
        Language {
            name: "FLUX",
            type_: "programming",
            color: "#88ccff",
            extensions: &[".fx", ".flux"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 106,
        }
    }
}
pub struct Factor;
impl Factor {
    pub fn info() -> Language {
        Language {
            name: "Factor",
            type_: "programming",
            color: "#636746",
            extensions: &[".factor"],
            tm_scope: "source.factor",
            ace_mode: "text",
            language_id: 108,
        }
    }
}
pub struct Fancy;
impl Fancy {
    pub fn info() -> Language {
        Language {
            name: "Fancy",
            type_: "programming",
            color: "#7b9db4",
            extensions: &[".fy", ".fancypack"],
            tm_scope: "source.fancy",
            ace_mode: "text",
            language_id: 109,
        }
    }
}
pub struct Fantom;
impl Fantom {
    pub fn info() -> Language {
        Language {
            name: "Fantom",
            type_: "programming",
            color: "#14253c",
            extensions: &[".fan"],
            tm_scope: "source.fan",
            ace_mode: "text",
            language_id: 110,
        }
    }
}
pub struct Faust;
impl Faust {
    pub fn info() -> Language {
        Language {
            name: "Faust",
            type_: "programming",
            color: "#c37240",
            extensions: &[".dsp"],
            tm_scope: "source.faust",
            ace_mode: "text",
            language_id: 622529198,
        }
    }
}
pub struct Fennel;
impl Fennel {
    pub fn info() -> Language {
        Language {
            name: "Fennel",
            type_: "programming",
            color: "#fff3d7",
            extensions: &[".fnl"],
            tm_scope: "source.fnl",
            ace_mode: "text",
            language_id: 239946126,
        }
    }
}
pub struct FilebenchWML;
impl FilebenchWML {
    pub fn info() -> Language {
        Language {
            name: "Filebench WML",
            type_: "programming",
            color: "#F6B900",
            extensions: &[".f"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 111,
        }
    }
}
pub struct Filterscript;
impl Filterscript {
    pub fn info() -> Language {
        Language {
            name: "Filterscript",
            type_: "programming",
            color: "",
            extensions: &[".fs"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 112,
        }
    }
}
pub struct Fluent;
impl Fluent {
    pub fn info() -> Language {
        Language {
            name: "Fluent",
            type_: "programming",
            color: "#ffcc33",
            extensions: &[".ftl"],
            tm_scope: "source.ftl",
            ace_mode: "text",
            language_id: 206353404,
        }
    }
}
pub struct Formatted;
impl Formatted {
    pub fn info() -> Language {
        Language {
            name: "Formatted",
            type_: "data",
            color: "",
            extensions: &[".for", ".eam.fs"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 113,
        }
    }
}
pub struct Forth;
impl Forth {
    pub fn info() -> Language {
        Language {
            name: "Forth",
            type_: "programming",
            color: "#341708",
            extensions: &[".fth", ".4th", ".f", ".for", ".forth", ".fr", ".frt", ".fs"],
            tm_scope: "source.forth",
            ace_mode: "forth",
            language_id: 114,
        }
    }
}
pub struct Fortran;
impl Fortran {
    pub fn info() -> Language {
        Language {
            name: "Fortran",
            type_: "programming",
            color: "#4d41b1",
            extensions: &[".f", ".f77", ".for", ".fpp"],
            tm_scope: "source.fortran",
            ace_mode: "text",
            language_id: 107,
        }
    }
}
pub struct FortranFreeForm;
impl FortranFreeForm {
    pub fn info() -> Language {
        Language {
            name: "Fortran Free Form",
            type_: "programming",
            color: "#4d41b1",
            extensions: &[".f90", ".f03", ".f08", ".f95"],
            tm_scope: "source.fortran.modern",
            ace_mode: "text",
            language_id: 761352333,
        }
    }
}
pub struct FreeBasic;
impl FreeBasic {
    pub fn info() -> Language {
        Language {
            name: "FreeBasic",
            type_: "programming",
            color: "#141AC9",
            extensions: &[".bi", ".bas"],
            tm_scope: "source.vbnet",
            ace_mode: "text",
            language_id: 472896659,
        }
    }
}
pub struct FreeMarker;
impl FreeMarker {
    pub fn info() -> Language {
        Language {
            name: "FreeMarker",
            type_: "programming",
            color: "#0050b2",
            extensions: &[".ftl"],
            tm_scope: "text.html.ftl",
            ace_mode: "ftl",
            language_id: 115,
        }
    }
}
pub struct Frege;
impl Frege {
    pub fn info() -> Language {
        Language {
            name: "Frege",
            type_: "programming",
            color: "#00cafe",
            extensions: &[".fr"],
            tm_scope: "source.haskell",
            ace_mode: "haskell",
            language_id: 116,
        }
    }
}
pub struct Futhark;
impl Futhark {
    pub fn info() -> Language {
        Language {
            name: "Futhark",
            type_: "programming",
            color: "#5f021f",
            extensions: &[".fut"],
            tm_scope: "source.futhark",
            ace_mode: "text",
            language_id: 97358117,
        }
    }
}
pub struct GCode;
impl GCode {
    pub fn info() -> Language {
        Language {
            name: "G-code",
            type_: "programming",
            color: "#D08CF2",
            extensions: &[".g", ".cnc", ".gco", ".gcode"],
            tm_scope: "source.gcode",
            ace_mode: "gcode",
            language_id: 117,
        }
    }
}
pub struct GAML;
impl GAML {
    pub fn info() -> Language {
        Language {
            name: "GAML",
            type_: "programming",
            color: "#FFC766",
            extensions: &[".gaml"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 290345951,
        }
    }
}
pub struct GAMS;
impl GAMS {
    pub fn info() -> Language {
        Language {
            name: "GAMS",
            type_: "programming",
            color: "#f49a22",
            extensions: &[".gms"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 118,
        }
    }
}
pub struct GAP;
impl GAP {
    pub fn info() -> Language {
        Language {
            name: "GAP",
            type_: "programming",
            color: "#0000cc",
            extensions: &[".g", ".gap", ".gd", ".gi", ".tst"],
            tm_scope: "source.gap",
            ace_mode: "text",
            language_id: 119,
        }
    }
}
pub struct GCCMachineDescription;
impl GCCMachineDescription {
    pub fn info() -> Language {
        Language {
            name: "GCC Machine Description",
            type_: "programming",
            color: "#FFCFAB",
            extensions: &[".md"],
            tm_scope: "source.lisp",
            ace_mode: "lisp",
            language_id: 121,
        }
    }
}
pub struct GDB;
impl GDB {
    pub fn info() -> Language {
        Language {
            name: "GDB",
            type_: "programming",
            color: "",
            extensions: &[".gdb", ".gdbinit"],
            tm_scope: "source.gdb",
            ace_mode: "text",
            language_id: 122,
        }
    }
}
pub struct GDScript;
impl GDScript {
    pub fn info() -> Language {
        Language {
            name: "GDScript",
            type_: "programming",
            color: "#355570",
            extensions: &[".gd"],
            tm_scope: "source.gdscript",
            ace_mode: "text",
            language_id: 123,
        }
    }
}
pub struct GEDCOM;
impl GEDCOM {
    pub fn info() -> Language {
        Language {
            name: "GEDCOM",
            type_: "data",
            color: "#003058",
            extensions: &[".ged"],
            tm_scope: "source.gedcom",
            ace_mode: "text",
            language_id: 459577965,
        }
    }
}
pub struct GLSL;
impl GLSL {
    pub fn info() -> Language {
        Language {
            name: "GLSL",
            type_: "programming",
            color: "#5686a5",
            extensions: &[
                ".glsl",
                ".fp",
                ".frag",
                ".frg",
                ".fs",
                ".fsh",
                ".fshader",
                ".geo",
                ".geom",
                ".glslf",
                ".glslv",
                ".gs",
                ".gshader",
                ".rchit",
                ".rmiss",
                ".shader",
                ".tesc",
                ".tese",
                ".vert",
                ".vrx",
                ".vs",
                ".vsh",
                ".vshader",
            ],
            tm_scope: "source.glsl",
            ace_mode: "glsl",
            language_id: 124,
        }
    }
}
pub struct GN;
impl GN {
    pub fn info() -> Language {
        Language {
            name: "GN",
            type_: "data",
            color: "",
            extensions: &[".gn", ".gni"],
            tm_scope: "source.gn",
            ace_mode: "python",
            language_id: 302957008,
        }
    }
}
pub struct GSC;
impl GSC {
    pub fn info() -> Language {
        Language {
            name: "GSC",
            type_: "programming",
            color: "#FF6800",
            extensions: &[".gsc", ".csc", ".gsh"],
            tm_scope: "source.gsc",
            ace_mode: "c_cpp",
            language_id: 257856279,
        }
    }
}
pub struct GameMakerLanguage;
impl GameMakerLanguage {
    pub fn info() -> Language {
        Language {
            name: "Game Maker Language",
            type_: "programming",
            color: "#71b417",
            extensions: &[".gml"],
            tm_scope: "source.c++",
            ace_mode: "c_cpp",
            language_id: 125,
        }
    }
}
pub struct Gemfilelock;
impl Gemfilelock {
    pub fn info() -> Language {
        Language {
            name: "Gemfile.lock",
            type_: "data",
            color: "#701516",
            extensions: &[],
            tm_scope: "source.gemfile-lock",
            ace_mode: "text",
            language_id: 907065713,
        }
    }
}
pub struct Gemini;
impl Gemini {
    pub fn info() -> Language {
        Language {
            name: "Gemini",
            type_: "prose",
            color: "#ff6900",
            extensions: &[".gmi"],
            tm_scope: "source.gemini",
            ace_mode: "text",
            language_id: 310828396,
        }
    }
}
pub struct Genero4gl;
impl Genero4gl {
    pub fn info() -> Language {
        Language {
            name: "Genero 4gl",
            type_: "programming",
            color: "#63408e",
            extensions: &[".4gl"],
            tm_scope: "source.genero-4gl",
            ace_mode: "text",
            language_id: 986054050,
        }
    }
}
pub struct GeneroPer;
impl GeneroPer {
    pub fn info() -> Language {
        Language {
            name: "Genero per",
            type_: "markup",
            color: "#d8df39",
            extensions: &[".per"],
            tm_scope: "source.genero-per",
            ace_mode: "text",
            language_id: 902995658,
        }
    }
}
pub struct Genie;
impl Genie {
    pub fn info() -> Language {
        Language {
            name: "Genie",
            type_: "programming",
            color: "#fb855d",
            extensions: &[".gs"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 792408528,
        }
    }
}
pub struct Genshi;
impl Genshi {
    pub fn info() -> Language {
        Language {
            name: "Genshi",
            type_: "programming",
            color: "#951531",
            extensions: &[".kid"],
            tm_scope: "text.xml.genshi",
            ace_mode: "xml",
            language_id: 126,
        }
    }
}
pub struct GentooEbuild;
impl GentooEbuild {
    pub fn info() -> Language {
        Language {
            name: "Gentoo Ebuild",
            type_: "programming",
            color: "#9400ff",
            extensions: &[".ebuild"],
            tm_scope: "source.shell",
            ace_mode: "sh",
            language_id: 127,
        }
    }
}
pub struct GentooEclass;
impl GentooEclass {
    pub fn info() -> Language {
        Language {
            name: "Gentoo Eclass",
            type_: "programming",
            color: "#9400ff",
            extensions: &[".eclass"],
            tm_scope: "source.shell",
            ace_mode: "sh",
            language_id: 128,
        }
    }
}
pub struct GerberImage;
impl GerberImage {
    pub fn info() -> Language {
        Language {
            name: "Gerber Image",
            type_: "data",
            color: "#d20b00",
            extensions: &[
                ".gbr",
                ".cmp",
                ".gbl",
                ".gbo",
                ".gbp",
                ".gbs",
                ".gko",
                ".gml",
                ".gpb",
                ".gpt",
                ".gtl",
                ".gto",
                ".gtp",
                ".gts",
                ".ncl",
                ".sol",
            ],
            tm_scope: "source.gerber",
            ace_mode: "text",
            language_id: 404627610,
        }
    }
}
pub struct GettextCatalog;
impl GettextCatalog {
    pub fn info() -> Language {
        Language {
            name: "Gettext Catalog",
            type_: "prose",
            color: "",
            extensions: &[".po", ".pot"],
            tm_scope: "source.po",
            ace_mode: "text",
            language_id: 129,
        }
    }
}
pub struct Gherkin;
impl Gherkin {
    pub fn info() -> Language {
        Language {
            name: "Gherkin",
            type_: "programming",
            color: "#5B2063",
            extensions: &[".feature", ".story"],
            tm_scope: "text.gherkin.feature",
            ace_mode: "text",
            language_id: 76,
        }
    }
}
pub struct GitAttributes;
impl GitAttributes {
    pub fn info() -> Language {
        Language {
            name: "Git Attributes",
            type_: "data",
            color: "#F44D27",
            extensions: &[],
            tm_scope: "source.gitattributes",
            ace_mode: "gitignore",
            language_id: 956324166,
        }
    }
}
pub struct GitConfig;
impl GitConfig {
    pub fn info() -> Language {
        Language {
            name: "Git Config",
            type_: "data",
            color: "#F44D27",
            extensions: &[".gitconfig"],
            tm_scope: "source.gitconfig",
            ace_mode: "ini",
            language_id: 807968997,
        }
    }
}
pub struct GitRevisionList;
impl GitRevisionList {
    pub fn info() -> Language {
        Language {
            name: "Git Revision List",
            type_: "data",
            color: "#F44D27",
            extensions: &[],
            tm_scope: "source.git-revlist",
            ace_mode: "text",
            language_id: 461881235,
        }
    }
}
pub struct Gleam;
impl Gleam {
    pub fn info() -> Language {
        Language {
            name: "Gleam",
            type_: "programming",
            color: "#ffaff3",
            extensions: &[".gleam"],
            tm_scope: "source.gleam",
            ace_mode: "text",
            language_id: 1054258749,
        }
    }
}
pub struct GlimmerJS;
impl GlimmerJS {
    pub fn info() -> Language {
        Language {
            name: "Glimmer JS",
            type_: "programming",
            color: "#F5835F",
            extensions: &[".gjs"],
            tm_scope: "source.gjs",
            ace_mode: "javascript",
            language_id: 5523150,
        }
    }
}
pub struct GlimmerTS;
impl GlimmerTS {
    pub fn info() -> Language {
        Language {
            name: "Glimmer TS",
            type_: "programming",
            color: "#3178c6",
            extensions: &[".gts"],
            tm_scope: "source.gts",
            ace_mode: "typescript",
            language_id: 95110458,
        }
    }
}
pub struct Glyph;
impl Glyph {
    pub fn info() -> Language {
        Language {
            name: "Glyph",
            type_: "programming",
            color: "#c1ac7f",
            extensions: &[".glf"],
            tm_scope: "source.tcl",
            ace_mode: "tcl",
            language_id: 130,
        }
    }
}
pub struct GlyphBitmapDistributionFormat;
impl GlyphBitmapDistributionFormat {
    pub fn info() -> Language {
        Language {
            name: "Glyph Bitmap Distribution Format",
            type_: "data",
            color: "",
            extensions: &[".bdf"],
            tm_scope: "source.bdf",
            ace_mode: "text",
            language_id: 997665271,
        }
    }
}
pub struct Gnuplot;
impl Gnuplot {
    pub fn info() -> Language {
        Language {
            name: "Gnuplot",
            type_: "programming",
            color: "#f0a9f0",
            extensions: &[".gp", ".gnu", ".gnuplot", ".p", ".plot", ".plt"],
            tm_scope: "source.gnuplot",
            ace_mode: "text",
            language_id: 131,
        }
    }
}
pub struct Go;
impl Go {
    pub fn info() -> Language {
        Language {
            name: "Go",
            type_: "programming",
            color: "#00ADD8",
            extensions: &[".go"],
            tm_scope: "source.go",
            ace_mode: "golang",
            language_id: 132,
        }
    }
}
pub struct GoChecksums;
impl GoChecksums {
    pub fn info() -> Language {
        Language {
            name: "Go Checksums",
            type_: "data",
            color: "#00ADD8",
            extensions: &[],
            tm_scope: "go.sum",
            ace_mode: "text",
            language_id: 1054391671,
        }
    }
}
pub struct GoModule;
impl GoModule {
    pub fn info() -> Language {
        Language {
            name: "Go Module",
            type_: "data",
            color: "#00ADD8",
            extensions: &[],
            tm_scope: "go.mod",
            ace_mode: "text",
            language_id: 947461016,
        }
    }
}
pub struct GoWorkspace;
impl GoWorkspace {
    pub fn info() -> Language {
        Language {
            name: "Go Workspace",
            type_: "data",
            color: "#00ADD8",
            extensions: &[],
            tm_scope: "go.mod",
            ace_mode: "text",
            language_id: 934546256,
        }
    }
}
pub struct GodotResource;
impl GodotResource {
    pub fn info() -> Language {
        Language {
            name: "Godot Resource",
            type_: "data",
            color: "#355570",
            extensions: &[".gdnlib", ".gdns", ".tres", ".tscn"],
            tm_scope: "source.gdresource",
            ace_mode: "text",
            language_id: 738107771,
        }
    }
}
pub struct Golo;
impl Golo {
    pub fn info() -> Language {
        Language {
            name: "Golo",
            type_: "programming",
            color: "#88562A",
            extensions: &[".golo"],
            tm_scope: "source.golo",
            ace_mode: "text",
            language_id: 133,
        }
    }
}
pub struct Gosu;
impl Gosu {
    pub fn info() -> Language {
        Language {
            name: "Gosu",
            type_: "programming",
            color: "#82937f",
            extensions: &[".gs", ".gst", ".gsx", ".vark"],
            tm_scope: "source.gosu.2",
            ace_mode: "text",
            language_id: 134,
        }
    }
}
pub struct Grace;
impl Grace {
    pub fn info() -> Language {
        Language {
            name: "Grace",
            type_: "programming",
            color: "#615f8b",
            extensions: &[".grace"],
            tm_scope: "source.grace",
            ace_mode: "text",
            language_id: 135,
        }
    }
}
pub struct Gradle;
impl Gradle {
    pub fn info() -> Language {
        Language {
            name: "Gradle",
            type_: "data",
            color: "#02303a",
            extensions: &[".gradle"],
            tm_scope: "source.groovy.gradle",
            ace_mode: "text",
            language_id: 136,
        }
    }
}
pub struct GradleKotlinDSL;
impl GradleKotlinDSL {
    pub fn info() -> Language {
        Language {
            name: "Gradle Kotlin DSL",
            type_: "data",
            color: "#02303a",
            extensions: &[".gradle.kts"],
            tm_scope: "source.kotlin",
            ace_mode: "text",
            language_id: 432600901,
        }
    }
}
pub struct GrammaticalFramework;
impl GrammaticalFramework {
    pub fn info() -> Language {
        Language {
            name: "Grammatical Framework",
            type_: "programming",
            color: "#ff0000",
            extensions: &[".gf"],
            tm_scope: "source.gf",
            ace_mode: "haskell",
            language_id: 137,
        }
    }
}
pub struct GraphModelingLanguage;
impl GraphModelingLanguage {
    pub fn info() -> Language {
        Language {
            name: "Graph Modeling Language",
            type_: "data",
            color: "",
            extensions: &[".gml"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 138,
        }
    }
}
pub struct GraphQL;
impl GraphQL {
    pub fn info() -> Language {
        Language {
            name: "GraphQL",
            type_: "data",
            color: "#e10098",
            extensions: &[".graphql", ".gql", ".graphqls"],
            tm_scope: "source.graphql",
            ace_mode: "text",
            language_id: 139,
        }
    }
}
pub struct GraphvizDOT;
impl GraphvizDOT {
    pub fn info() -> Language {
        Language {
            name: "Graphviz (DOT)",
            type_: "data",
            color: "#2596be",
            extensions: &[".dot", ".gv"],
            tm_scope: "source.dot",
            ace_mode: "text",
            language_id: 140,
        }
    }
}
pub struct Groovy;
impl Groovy {
    pub fn info() -> Language {
        Language {
            name: "Groovy",
            type_: "programming",
            color: "#4298b8",
            extensions: &[".groovy", ".grt", ".gtpl", ".gvy"],
            tm_scope: "source.groovy",
            ace_mode: "groovy",
            language_id: 142,
        }
    }
}
pub struct GroovyServerPages;
impl GroovyServerPages {
    pub fn info() -> Language {
        Language {
            name: "Groovy Server Pages",
            type_: "programming",
            color: "#4298b8",
            extensions: &[".gsp"],
            tm_scope: "text.html.jsp",
            ace_mode: "jsp",
            language_id: 143,
        }
    }
}
pub struct HAProxy;
impl HAProxy {
    pub fn info() -> Language {
        Language {
            name: "HAProxy",
            type_: "data",
            color: "#106da9",
            extensions: &[".cfg"],
            tm_scope: "source.haproxy-config",
            ace_mode: "text",
            language_id: 366607477,
        }
    }
}
pub struct HCL;
impl HCL {
    pub fn info() -> Language {
        Language {
            name: "HCL",
            type_: "programming",
            color: "#844FBA",
            extensions: &[".hcl", ".nomad", ".tf", ".tfvars", ".workflow"],
            tm_scope: "source.hcl",
            ace_mode: "ruby",
            language_id: 144,
        }
    }
}
pub struct HLSL;
impl HLSL {
    pub fn info() -> Language {
        Language {
            name: "HLSL",
            type_: "programming",
            color: "#aace60",
            extensions: &[".hlsl", ".cginc", ".fx", ".fxh", ".hlsli"],
            tm_scope: "source.hlsl",
            ace_mode: "text",
            language_id: 145,
        }
    }
}
pub struct HOCON;
impl HOCON {
    pub fn info() -> Language {
        Language {
            name: "HOCON",
            type_: "data",
            color: "#9ff8ee",
            extensions: &[".hocon"],
            tm_scope: "source.hocon",
            ace_mode: "text",
            language_id: 679725279,
        }
    }
}
pub struct HTML;
impl HTML {
    pub fn info() -> Language {
        Language {
            name: "HTML",
            type_: "markup",
            color: "#e34c26",
            extensions: &[".html", ".hta", ".htm", ".html.hl", ".inc", ".xht", ".xhtml"],
            tm_scope: "text.html.basic",
            ace_mode: "html",
            language_id: 146,
        }
    }
}
pub struct HTMLpECR;
impl HTMLpECR {
    pub fn info() -> Language {
        Language {
            name: "HTML+ECR",
            type_: "markup",
            color: "#2e1052",
            extensions: &[".ecr"],
            tm_scope: "text.html.ecr",
            ace_mode: "text",
            language_id: 148,
        }
    }
}
pub struct HTMLpEEX;
impl HTMLpEEX {
    pub fn info() -> Language {
        Language {
            name: "HTML+EEX",
            type_: "markup",
            color: "#6e4a7e",
            extensions: &[".eex", ".html.heex", ".html.leex"],
            tm_scope: "text.html.elixir",
            ace_mode: "text",
            language_id: 149,
        }
    }
}
pub struct HTMLpERB;
impl HTMLpERB {
    pub fn info() -> Language {
        Language {
            name: "HTML+ERB",
            type_: "markup",
            color: "#701516",
            extensions: &[".erb", ".erb.deface", ".rhtml"],
            tm_scope: "text.html.erb",
            ace_mode: "text",
            language_id: 150,
        }
    }
}
pub struct HTMLpPHP;
impl HTMLpPHP {
    pub fn info() -> Language {
        Language {
            name: "HTML+PHP",
            type_: "markup",
            color: "#4f5d95",
            extensions: &[".phtml"],
            tm_scope: "text.html.php",
            ace_mode: "php",
            language_id: 151,
        }
    }
}
pub struct HTMLpRazor;
impl HTMLpRazor {
    pub fn info() -> Language {
        Language {
            name: "HTML+Razor",
            type_: "markup",
            color: "#512be4",
            extensions: &[".cshtml", ".razor"],
            tm_scope: "text.html.cshtml",
            ace_mode: "razor",
            language_id: 479039817,
        }
    }
}
pub struct HTTP;
impl HTTP {
    pub fn info() -> Language {
        Language {
            name: "HTTP",
            type_: "data",
            color: "#005C9C",
            extensions: &[".http"],
            tm_scope: "source.httpspec",
            ace_mode: "text",
            language_id: 152,
        }
    }
}
pub struct HXML;
impl HXML {
    pub fn info() -> Language {
        Language {
            name: "HXML",
            type_: "data",
            color: "#f68712",
            extensions: &[".hxml"],
            tm_scope: "source.hxml",
            ace_mode: "text",
            language_id: 786683730,
        }
    }
}
pub struct Hack;
impl Hack {
    pub fn info() -> Language {
        Language {
            name: "Hack",
            type_: "programming",
            color: "#878787",
            extensions: &[".hack", ".hh", ".hhi", ".php"],
            tm_scope: "source.hack",
            ace_mode: "php",
            language_id: 153,
        }
    }
}
pub struct Haml;
impl Haml {
    pub fn info() -> Language {
        Language {
            name: "Haml",
            type_: "markup",
            color: "#ece2a9",
            extensions: &[".haml", ".haml.deface"],
            tm_scope: "text.haml",
            ace_mode: "haml",
            language_id: 154,
        }
    }
}
pub struct Handlebars;
impl Handlebars {
    pub fn info() -> Language {
        Language {
            name: "Handlebars",
            type_: "markup",
            color: "#f7931e",
            extensions: &[".handlebars", ".hbs"],
            tm_scope: "text.html.handlebars",
            ace_mode: "handlebars",
            language_id: 155,
        }
    }
}
pub struct Harbour;
impl Harbour {
    pub fn info() -> Language {
        Language {
            name: "Harbour",
            type_: "programming",
            color: "#0e60e3",
            extensions: &[".hb"],
            tm_scope: "source.harbour",
            ace_mode: "text",
            language_id: 156,
        }
    }
}
pub struct Haskell;
impl Haskell {
    pub fn info() -> Language {
        Language {
            name: "Haskell",
            type_: "programming",
            color: "#5e5086",
            extensions: &[".hs", ".hs-boot", ".hsc"],
            tm_scope: "source.haskell",
            ace_mode: "haskell",
            language_id: 157,
        }
    }
}
pub struct Haxe;
impl Haxe {
    pub fn info() -> Language {
        Language {
            name: "Haxe",
            type_: "programming",
            color: "#df7900",
            extensions: &[".hx", ".hxsl"],
            tm_scope: "source.hx",
            ace_mode: "haxe",
            language_id: 158,
        }
    }
}
pub struct HiveQL;
impl HiveQL {
    pub fn info() -> Language {
        Language {
            name: "HiveQL",
            type_: "programming",
            color: "#dce200",
            extensions: &[".q", ".hql"],
            tm_scope: "source.hql",
            ace_mode: "sql",
            language_id: 931814087,
        }
    }
}
pub struct HolyC;
impl HolyC {
    pub fn info() -> Language {
        Language {
            name: "HolyC",
            type_: "programming",
            color: "#ffefaf",
            extensions: &[".hc"],
            tm_scope: "source.hc",
            ace_mode: "c_cpp",
            language_id: 928121743,
        }
    }
}
pub struct HostsFile;
impl HostsFile {
    pub fn info() -> Language {
        Language {
            name: "Hosts File",
            type_: "data",
            color: "#308888",
            extensions: &[],
            tm_scope: "source.hosts",
            ace_mode: "text",
            language_id: 231021894,
        }
    }
}
pub struct Hy;
impl Hy {
    pub fn info() -> Language {
        Language {
            name: "Hy",
            type_: "programming",
            color: "#7790B2",
            extensions: &[".hy"],
            tm_scope: "source.hy",
            ace_mode: "text",
            language_id: 159,
        }
    }
}
pub struct HyPhy;
impl HyPhy {
    pub fn info() -> Language {
        Language {
            name: "HyPhy",
            type_: "programming",
            color: "",
            extensions: &[".bf"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 160,
        }
    }
}
pub struct IDL;
impl IDL {
    pub fn info() -> Language {
        Language {
            name: "IDL",
            type_: "programming",
            color: "#a3522f",
            extensions: &[".pro", ".dlm"],
            tm_scope: "source.idl",
            ace_mode: "text",
            language_id: 161,
        }
    }
}
pub struct IGORPro;
impl IGORPro {
    pub fn info() -> Language {
        Language {
            name: "IGOR Pro",
            type_: "programming",
            color: "#0000cc",
            extensions: &[".ipf"],
            tm_scope: "source.igor",
            ace_mode: "text",
            language_id: 162,
        }
    }
}
pub struct INI;
impl INI {
    pub fn info() -> Language {
        Language {
            name: "INI",
            type_: "data",
            color: "#d1dbe0",
            extensions: &[
                ".ini",
                ".cfg",
                ".cnf",
                ".dof",
                ".lektorproject",
                ".prefs",
                ".pro",
                ".properties",
                ".url",
            ],
            tm_scope: "source.ini",
            ace_mode: "ini",
            language_id: 163,
        }
    }
}
pub struct IRCLog;
impl IRCLog {
    pub fn info() -> Language {
        Language {
            name: "IRC log",
            type_: "data",
            color: "",
            extensions: &[".irclog", ".weechatlog"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 164,
        }
    }
}
pub struct Idris;
impl Idris {
    pub fn info() -> Language {
        Language {
            name: "Idris",
            type_: "programming",
            color: "#b30000",
            extensions: &[".idr", ".lidr"],
            tm_scope: "source.idris",
            ace_mode: "text",
            language_id: 165,
        }
    }
}
pub struct IgnoreList;
impl IgnoreList {
    pub fn info() -> Language {
        Language {
            name: "Ignore List",
            type_: "data",
            color: "#000000",
            extensions: &[".gitignore"],
            tm_scope: "source.gitignore",
            ace_mode: "gitignore",
            language_id: 74444240,
        }
    }
}
pub struct ImageJMacro;
impl ImageJMacro {
    pub fn info() -> Language {
        Language {
            name: "ImageJ Macro",
            type_: "programming",
            color: "#99AAFF",
            extensions: &[".ijm"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 575143428,
        }
    }
}
pub struct Imba;
impl Imba {
    pub fn info() -> Language {
        Language {
            name: "Imba",
            type_: "programming",
            color: "#16cec6",
            extensions: &[".imba"],
            tm_scope: "source.imba",
            ace_mode: "text",
            language_id: 1057618448,
        }
    }
}
pub struct Inform7;
impl Inform7 {
    pub fn info() -> Language {
        Language {
            name: "Inform 7",
            type_: "programming",
            color: "",
            extensions: &[".ni", ".i7x"],
            tm_scope: "source.inform7",
            ace_mode: "text",
            language_id: 166,
        }
    }
}
pub struct Ink;
impl Ink {
    pub fn info() -> Language {
        Language {
            name: "Ink",
            type_: "programming",
            color: "",
            extensions: &[".ink"],
            tm_scope: "source.ink",
            ace_mode: "text",
            language_id: 838252715,
        }
    }
}
pub struct InnoSetup;
impl InnoSetup {
    pub fn info() -> Language {
        Language {
            name: "Inno Setup",
            type_: "programming",
            color: "#264b99",
            extensions: &[".iss", ".isl"],
            tm_scope: "source.inno",
            ace_mode: "text",
            language_id: 167,
        }
    }
}
pub struct Io;
impl Io {
    pub fn info() -> Language {
        Language {
            name: "Io",
            type_: "programming",
            color: "#a9188d",
            extensions: &[".io"],
            tm_scope: "source.io",
            ace_mode: "io",
            language_id: 168,
        }
    }
}
pub struct Ioke;
impl Ioke {
    pub fn info() -> Language {
        Language {
            name: "Ioke",
            type_: "programming",
            color: "#078193",
            extensions: &[".ik"],
            tm_scope: "source.ioke",
            ace_mode: "text",
            language_id: 169,
        }
    }
}
pub struct Isabelle;
impl Isabelle {
    pub fn info() -> Language {
        Language {
            name: "Isabelle",
            type_: "programming",
            color: "#FEFE00",
            extensions: &[".thy"],
            tm_scope: "source.isabelle.theory",
            ace_mode: "text",
            language_id: 170,
        }
    }
}
pub struct IsabelleROOT;
impl IsabelleROOT {
    pub fn info() -> Language {
        Language {
            name: "Isabelle ROOT",
            type_: "programming",
            color: "#FEFE00",
            extensions: &[],
            tm_scope: "source.isabelle.root",
            ace_mode: "text",
            language_id: 171,
        }
    }
}
pub struct J;
impl J {
    pub fn info() -> Language {
        Language {
            name: "J",
            type_: "programming",
            color: "#9EEDFF",
            extensions: &[".ijs"],
            tm_scope: "source.j",
            ace_mode: "text",
            language_id: 172,
        }
    }
}
pub struct JARManifest;
impl JARManifest {
    pub fn info() -> Language {
        Language {
            name: "JAR Manifest",
            type_: "data",
            color: "#b07219",
            extensions: &[],
            tm_scope: "source.yaml",
            ace_mode: "text",
            language_id: 447261135,
        }
    }
}
pub struct JCL;
impl JCL {
    pub fn info() -> Language {
        Language {
            name: "JCL",
            type_: "programming",
            color: "#d90e09",
            extensions: &[".jcl"],
            tm_scope: "source.jcl",
            ace_mode: "text",
            language_id: 316620079,
        }
    }
}
pub struct JFlex;
impl JFlex {
    pub fn info() -> Language {
        Language {
            name: "JFlex",
            type_: "programming",
            color: "#DBCA00",
            extensions: &[".flex", ".jflex"],
            tm_scope: "source.jflex",
            ace_mode: "text",
            language_id: 173,
        }
    }
}
pub struct JSON;
impl JSON {
    pub fn info() -> Language {
        Language {
            name: "JSON",
            type_: "data",
            color: "#292929",
            extensions: &[
                ".json",
                ".4DForm",
                ".4DProject",
                ".avsc",
                ".geojson",
                ".gltf",
                ".har",
                ".ice",
                ".JSON-tmLanguage",
                ".jsonl",
                ".mcmeta",
                ".sarif",
                ".tfstate",
                ".tfstate.backup",
                ".topojson",
                ".webapp",
                ".webmanifest",
                ".yy",
                ".yyp",
            ],
            tm_scope: "source.json",
            ace_mode: "json",
            language_id: 174,
        }
    }
}
pub struct JSONWithComments;
impl JSONWithComments {
    pub fn info() -> Language {
        Language {
            name: "JSON with Comments",
            type_: "data",
            color: "#292929",
            extensions: &[
                ".jsonc",
                ".code-snippets",
                ".code-workspace",
                ".sublime-build",
                ".sublime-color-scheme",
                ".sublime-commands",
                ".sublime-completions",
                ".sublime-keymap",
                ".sublime-macro",
                ".sublime-menu",
                ".sublime-mousemap",
                ".sublime-project",
                ".sublime-settings",
                ".sublime-theme",
                ".sublime-workspace",
                ".sublime_metrics",
                ".sublime_session",
            ],
            tm_scope: "source.js",
            ace_mode: "javascript",
            language_id: 423,
        }
    }
}
pub struct JSON5;
impl JSON5 {
    pub fn info() -> Language {
        Language {
            name: "JSON5",
            type_: "data",
            color: "#267CB9",
            extensions: &[".json5"],
            tm_scope: "source.js",
            ace_mode: "javascript",
            language_id: 175,
        }
    }
}
pub struct JSONLD;
impl JSONLD {
    pub fn info() -> Language {
        Language {
            name: "JSONLD",
            type_: "data",
            color: "#0c479c",
            extensions: &[".jsonld"],
            tm_scope: "source.js",
            ace_mode: "javascript",
            language_id: 176,
        }
    }
}
pub struct JSONiq;
impl JSONiq {
    pub fn info() -> Language {
        Language {
            name: "JSONiq",
            type_: "programming",
            color: "#40d47e",
            extensions: &[".jq"],
            tm_scope: "source.jsoniq",
            ace_mode: "jsoniq",
            language_id: 177,
        }
    }
}
pub struct Janet;
impl Janet {
    pub fn info() -> Language {
        Language {
            name: "Janet",
            type_: "programming",
            color: "#0886a5",
            extensions: &[".janet"],
            tm_scope: "source.janet",
            ace_mode: "scheme",
            language_id: 1028705371,
        }
    }
}
pub struct Jasmin;
impl Jasmin {
    pub fn info() -> Language {
        Language {
            name: "Jasmin",
            type_: "programming",
            color: "#d03600",
            extensions: &[".j"],
            tm_scope: "source.jasmin",
            ace_mode: "java",
            language_id: 180,
        }
    }
}
pub struct Java;
impl Java {
    pub fn info() -> Language {
        Language {
            name: "Java",
            type_: "programming",
            color: "#b07219",
            extensions: &[".java", ".jav", ".jsh"],
            tm_scope: "source.java",
            ace_mode: "java",
            language_id: 181,
        }
    }
}
pub struct JavaProperties;
impl JavaProperties {
    pub fn info() -> Language {
        Language {
            name: "Java Properties",
            type_: "data",
            color: "#2A6277",
            extensions: &[".properties"],
            tm_scope: "source.java-properties",
            ace_mode: "properties",
            language_id: 519377561,
        }
    }
}
pub struct JavaServerPages;
impl JavaServerPages {
    pub fn info() -> Language {
        Language {
            name: "Java Server Pages",
            type_: "programming",
            color: "#2A6277",
            extensions: &[".jsp", ".tag"],
            tm_scope: "text.html.jsp",
            ace_mode: "jsp",
            language_id: 182,
        }
    }
}
pub struct JavaTemplateEngine;
impl JavaTemplateEngine {
    pub fn info() -> Language {
        Language {
            name: "Java Template Engine",
            type_: "programming",
            color: "#2A6277",
            extensions: &[".jte"],
            tm_scope: "text.html.jte",
            ace_mode: "text",
            language_id: 599494012,
        }
    }
}
pub struct JavaScript;
impl JavaScript {
    pub fn info() -> Language {
        Language {
            name: "JavaScript",
            type_: "programming",
            color: "#f1e05a",
            extensions: &[
                ".js",
                "._js",
                ".bones",
                ".cjs",
                ".es",
                ".es6",
                ".frag",
                ".gs",
                ".jake",
                ".javascript",
                ".jsb",
                ".jscad",
                ".jsfl",
                ".jslib",
                ".jsm",
                ".jspre",
                ".jss",
                ".jsx",
                ".mjs",
                ".njs",
                ".pac",
                ".sjs",
                ".ssjs",
                ".xsjs",
                ".xsjslib",
            ],
            tm_scope: "source.js",
            ace_mode: "javascript",
            language_id: 183,
        }
    }
}
pub struct JavaScriptpERB;
impl JavaScriptpERB {
    pub fn info() -> Language {
        Language {
            name: "JavaScript+ERB",
            type_: "programming",
            color: "#f1e05a",
            extensions: &[".js.erb"],
            tm_scope: "source.js",
            ace_mode: "javascript",
            language_id: 914318960,
        }
    }
}
pub struct JestSnapshot;
impl JestSnapshot {
    pub fn info() -> Language {
        Language {
            name: "Jest Snapshot",
            type_: "data",
            color: "#15c213",
            extensions: &[".snap"],
            tm_scope: "source.jest.snap",
            ace_mode: "javascript",
            language_id: 774635084,
        }
    }
}
pub struct JetBrainsMPS;
impl JetBrainsMPS {
    pub fn info() -> Language {
        Language {
            name: "JetBrains MPS",
            type_: "programming",
            color: "#21D789",
            extensions: &[".mps", ".mpl", ".msd"],
            tm_scope: "none",
            ace_mode: "xml",
            language_id: 465165328,
        }
    }
}
pub struct Jinja;
impl Jinja {
    pub fn info() -> Language {
        Language {
            name: "Jinja",
            type_: "markup",
            color: "#a52a22",
            extensions: &[".jinja", ".j2", ".jinja2"],
            tm_scope: "text.html.django",
            ace_mode: "django",
            language_id: 147,
        }
    }
}
pub struct Jison;
impl Jison {
    pub fn info() -> Language {
        Language {
            name: "Jison",
            type_: "programming",
            color: "#56b3cb",
            extensions: &[".jison"],
            tm_scope: "source.jison",
            ace_mode: "text",
            language_id: 284531423,
        }
    }
}
pub struct JisonLex;
impl JisonLex {
    pub fn info() -> Language {
        Language {
            name: "Jison Lex",
            type_: "programming",
            color: "#56b3cb",
            extensions: &[".jisonlex"],
            tm_scope: "source.jisonlex",
            ace_mode: "text",
            language_id: 406395330,
        }
    }
}
pub struct Jolie;
impl Jolie {
    pub fn info() -> Language {
        Language {
            name: "Jolie",
            type_: "programming",
            color: "#843179",
            extensions: &[".ol", ".iol"],
            tm_scope: "source.jolie",
            ace_mode: "text",
            language_id: 998078858,
        }
    }
}
pub struct Jsonnet;
impl Jsonnet {
    pub fn info() -> Language {
        Language {
            name: "Jsonnet",
            type_: "programming",
            color: "#0064bd",
            extensions: &[".jsonnet", ".libsonnet"],
            tm_scope: "source.jsonnet",
            ace_mode: "text",
            language_id: 664885656,
        }
    }
}
pub struct Julia;
impl Julia {
    pub fn info() -> Language {
        Language {
            name: "Julia",
            type_: "programming",
            color: "#a270ba",
            extensions: &[".jl"],
            tm_scope: "source.julia",
            ace_mode: "julia",
            language_id: 184,
        }
    }
}
pub struct JuliaREPL;
impl JuliaREPL {
    pub fn info() -> Language {
        Language {
            name: "Julia REPL",
            type_: "programming",
            color: "#a270ba",
            extensions: &[],
            tm_scope: "source.julia.console",
            ace_mode: "text",
            language_id: 220689142,
        }
    }
}
pub struct JupyterNotebook;
impl JupyterNotebook {
    pub fn info() -> Language {
        Language {
            name: "Jupyter Notebook",
            type_: "markup",
            color: "#DA5B0B",
            extensions: &[".ipynb"],
            tm_scope: "source.json",
            ace_mode: "json",
            language_id: 185,
        }
    }
}
pub struct Just;
impl Just {
    pub fn info() -> Language {
        Language {
            name: "Just",
            type_: "programming",
            color: "#384d54",
            extensions: &[".just"],
            tm_scope: "source.just",
            ace_mode: "text",
            language_id: 128447695,
        }
    }
}
pub struct KRL;
impl KRL {
    pub fn info() -> Language {
        Language {
            name: "KRL",
            type_: "programming",
            color: "#28430A",
            extensions: &[".krl"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 186,
        }
    }
}
pub struct KaitaiStruct;
impl KaitaiStruct {
    pub fn info() -> Language {
        Language {
            name: "Kaitai Struct",
            type_: "programming",
            color: "#773b37",
            extensions: &[".ksy"],
            tm_scope: "source.yaml",
            ace_mode: "yaml",
            language_id: 818804755,
        }
    }
}
pub struct KakouneScript;
impl KakouneScript {
    pub fn info() -> Language {
        Language {
            name: "KakouneScript",
            type_: "programming",
            color: "#6f8042",
            extensions: &[".kak"],
            tm_scope: "source.kakscript",
            ace_mode: "text",
            language_id: 603336474,
        }
    }
}
pub struct KerboScript;
impl KerboScript {
    pub fn info() -> Language {
        Language {
            name: "KerboScript",
            type_: "programming",
            color: "#41adf0",
            extensions: &[".ks"],
            tm_scope: "source.kerboscript",
            ace_mode: "text",
            language_id: 59716426,
        }
    }
}
pub struct KiCadLayout;
impl KiCadLayout {
    pub fn info() -> Language {
        Language {
            name: "KiCad Layout",
            type_: "data",
            color: "#2f4aab",
            extensions: &[".kicad_pcb", ".kicad_mod", ".kicad_wks"],
            tm_scope: "source.pcb.sexp",
            ace_mode: "lisp",
            language_id: 187,
        }
    }
}
pub struct KiCadLegacyLayout;
impl KiCadLegacyLayout {
    pub fn info() -> Language {
        Language {
            name: "KiCad Legacy Layout",
            type_: "data",
            color: "#2f4aab",
            extensions: &[".brd"],
            tm_scope: "source.pcb.board",
            ace_mode: "text",
            language_id: 140848857,
        }
    }
}
pub struct KiCadSchematic;
impl KiCadSchematic {
    pub fn info() -> Language {
        Language {
            name: "KiCad Schematic",
            type_: "data",
            color: "#2f4aab",
            extensions: &[".kicad_sch", ".sch"],
            tm_scope: "source.pcb.schematic",
            ace_mode: "text",
            language_id: 622447435,
        }
    }
}
pub struct Kickstart;
impl Kickstart {
    pub fn info() -> Language {
        Language {
            name: "Kickstart",
            type_: "data",
            color: "",
            extensions: &[".ks"],
            tm_scope: "source.kickstart",
            ace_mode: "text",
            language_id: 692635484,
        }
    }
}
pub struct Kit;
impl Kit {
    pub fn info() -> Language {
        Language {
            name: "Kit",
            type_: "markup",
            color: "",
            extensions: &[".kit"],
            tm_scope: "text.html.basic",
            ace_mode: "html",
            language_id: 188,
        }
    }
}
pub struct Kotlin;
impl Kotlin {
    pub fn info() -> Language {
        Language {
            name: "Kotlin",
            type_: "programming",
            color: "#A97BFF",
            extensions: &[".kt", ".ktm", ".kts"],
            tm_scope: "source.kotlin",
            ace_mode: "text",
            language_id: 189,
        }
    }
}
pub struct Kusto;
impl Kusto {
    pub fn info() -> Language {
        Language {
            name: "Kusto",
            type_: "data",
            color: "",
            extensions: &[".csl", ".kql"],
            tm_scope: "source.kusto",
            ace_mode: "text",
            language_id: 225697190,
        }
    }
}
pub struct LFE;
impl LFE {
    pub fn info() -> Language {
        Language {
            name: "LFE",
            type_: "programming",
            color: "#4C3023",
            extensions: &[".lfe"],
            tm_scope: "source.lisp",
            ace_mode: "lisp",
            language_id: 190,
        }
    }
}
pub struct LLVM;
impl LLVM {
    pub fn info() -> Language {
        Language {
            name: "LLVM",
            type_: "programming",
            color: "#185619",
            extensions: &[".ll"],
            tm_scope: "source.llvm",
            ace_mode: "text",
            language_id: 191,
        }
    }
}
pub struct LOLCODE;
impl LOLCODE {
    pub fn info() -> Language {
        Language {
            name: "LOLCODE",
            type_: "programming",
            color: "#cc9900",
            extensions: &[".lol"],
            tm_scope: "source.lolcode",
            ace_mode: "text",
            language_id: 192,
        }
    }
}
pub struct LSL;
impl LSL {
    pub fn info() -> Language {
        Language {
            name: "LSL",
            type_: "programming",
            color: "#3d9970",
            extensions: &[".lsl", ".lslp"],
            tm_scope: "source.lsl",
            ace_mode: "lsl",
            language_id: 193,
        }
    }
}
pub struct LTspiceSymbol;
impl LTspiceSymbol {
    pub fn info() -> Language {
        Language {
            name: "LTspice Symbol",
            type_: "data",
            color: "",
            extensions: &[".asy"],
            tm_scope: "source.ltspice.symbol",
            ace_mode: "text",
            language_id: 1013566805,
        }
    }
}
pub struct LabVIEW;
impl LabVIEW {
    pub fn info() -> Language {
        Language {
            name: "LabVIEW",
            type_: "programming",
            color: "#fede06",
            extensions: &[".lvproj", ".lvclass", ".lvlib"],
            tm_scope: "text.xml",
            ace_mode: "xml",
            language_id: 194,
        }
    }
}
pub struct Lark;
impl Lark {
    pub fn info() -> Language {
        Language {
            name: "Lark",
            type_: "data",
            color: "#2980B9",
            extensions: &[".lark"],
            tm_scope: "source.lark",
            ace_mode: "text",
            language_id: 758480799,
        }
    }
}
pub struct Lasso;
impl Lasso {
    pub fn info() -> Language {
        Language {
            name: "Lasso",
            type_: "programming",
            color: "#999999",
            extensions: &[".lasso", ".las", ".lasso8", ".lasso9"],
            tm_scope: "file.lasso",
            ace_mode: "text",
            language_id: 195,
        }
    }
}
pub struct Latte;
impl Latte {
    pub fn info() -> Language {
        Language {
            name: "Latte",
            type_: "markup",
            color: "#f2a542",
            extensions: &[".latte"],
            tm_scope: "text.html.smarty",
            ace_mode: "smarty",
            language_id: 196,
        }
    }
}
pub struct Lean;
impl Lean {
    pub fn info() -> Language {
        Language {
            name: "Lean",
            type_: "programming",
            color: "",
            extensions: &[".lean", ".hlean"],
            tm_scope: "source.lean",
            ace_mode: "text",
            language_id: 197,
        }
    }
}
pub struct Lean4;
impl Lean4 {
    pub fn info() -> Language {
        Language {
            name: "Lean 4",
            type_: "programming",
            color: "",
            extensions: &[".lean"],
            tm_scope: "source.lean4",
            ace_mode: "text",
            language_id: 455147478,
        }
    }
}
pub struct Less;
impl Less {
    pub fn info() -> Language {
        Language {
            name: "Less",
            type_: "markup",
            color: "#1d365d",
            extensions: &[".less"],
            tm_scope: "source.css.less",
            ace_mode: "less",
            language_id: 198,
        }
    }
}
pub struct Lex;
impl Lex {
    pub fn info() -> Language {
        Language {
            name: "Lex",
            type_: "programming",
            color: "#DBCA00",
            extensions: &[".l", ".lex"],
            tm_scope: "source.lex",
            ace_mode: "text",
            language_id: 199,
        }
    }
}
pub struct LigoLANG;
impl LigoLANG {
    pub fn info() -> Language {
        Language {
            name: "LigoLANG",
            type_: "programming",
            color: "#0e74ff",
            extensions: &[".ligo"],
            tm_scope: "source.ligo",
            ace_mode: "pascal",
            language_id: 1040646257,
        }
    }
}
pub struct LilyPond;
impl LilyPond {
    pub fn info() -> Language {
        Language {
            name: "LilyPond",
            type_: "programming",
            color: "#9ccc7c",
            extensions: &[".ly", ".ily"],
            tm_scope: "source.lilypond",
            ace_mode: "text",
            language_id: 200,
        }
    }
}
pub struct Limbo;
impl Limbo {
    pub fn info() -> Language {
        Language {
            name: "Limbo",
            type_: "programming",
            color: "",
            extensions: &[".b", ".m"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 201,
        }
    }
}
pub struct LinkerScript;
impl LinkerScript {
    pub fn info() -> Language {
        Language {
            name: "Linker Script",
            type_: "data",
            color: "",
            extensions: &[".ld", ".lds", ".x"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 202,
        }
    }
}
pub struct LinuxKernelModule;
impl LinuxKernelModule {
    pub fn info() -> Language {
        Language {
            name: "Linux Kernel Module",
            type_: "data",
            color: "",
            extensions: &[".mod"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 203,
        }
    }
}
pub struct Liquid;
impl Liquid {
    pub fn info() -> Language {
        Language {
            name: "Liquid",
            type_: "markup",
            color: "#67b8de",
            extensions: &[".liquid"],
            tm_scope: "text.html.liquid",
            ace_mode: "liquid",
            language_id: 204,
        }
    }
}
pub struct LiterateAgda;
impl LiterateAgda {
    pub fn info() -> Language {
        Language {
            name: "Literate Agda",
            type_: "programming",
            color: "#315665",
            extensions: &[".lagda"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 205,
        }
    }
}
pub struct LiterateCoffeeScript;
impl LiterateCoffeeScript {
    pub fn info() -> Language {
        Language {
            name: "Literate CoffeeScript",
            type_: "programming",
            color: "#244776",
            extensions: &[".litcoffee", ".coffee.md"],
            tm_scope: "source.litcoffee",
            ace_mode: "text",
            language_id: 206,
        }
    }
}
pub struct LiterateHaskell;
impl LiterateHaskell {
    pub fn info() -> Language {
        Language {
            name: "Literate Haskell",
            type_: "programming",
            color: "#5e5086",
            extensions: &[".lhs"],
            tm_scope: "text.tex.latex.haskell",
            ace_mode: "text",
            language_id: 207,
        }
    }
}
pub struct LiveCodeScript;
impl LiveCodeScript {
    pub fn info() -> Language {
        Language {
            name: "LiveCode Script",
            type_: "programming",
            color: "#0c5ba5",
            extensions: &[".livecodescript"],
            tm_scope: "source.livecodescript",
            ace_mode: "text",
            language_id: 891017,
        }
    }
}
pub struct LiveScript;
impl LiveScript {
    pub fn info() -> Language {
        Language {
            name: "LiveScript",
            type_: "programming",
            color: "#499886",
            extensions: &[".ls", "._ls"],
            tm_scope: "source.livescript",
            ace_mode: "livescript",
            language_id: 208,
        }
    }
}
pub struct Logos;
impl Logos {
    pub fn info() -> Language {
        Language {
            name: "Logos",
            type_: "programming",
            color: "",
            extensions: &[".xm", ".x", ".xi"],
            tm_scope: "source.logos",
            ace_mode: "text",
            language_id: 209,
        }
    }
}
pub struct Logtalk;
impl Logtalk {
    pub fn info() -> Language {
        Language {
            name: "Logtalk",
            type_: "programming",
            color: "#295b9a",
            extensions: &[".lgt", ".logtalk"],
            tm_scope: "source.logtalk",
            ace_mode: "text",
            language_id: 210,
        }
    }
}
pub struct LookML;
impl LookML {
    pub fn info() -> Language {
        Language {
            name: "LookML",
            type_: "programming",
            color: "#652B81",
            extensions: &[".lkml", ".lookml"],
            tm_scope: "source.yaml",
            ace_mode: "yaml",
            language_id: 211,
        }
    }
}
pub struct LoomScript;
impl LoomScript {
    pub fn info() -> Language {
        Language {
            name: "LoomScript",
            type_: "programming",
            color: "",
            extensions: &[".ls"],
            tm_scope: "source.loomscript",
            ace_mode: "text",
            language_id: 212,
        }
    }
}
pub struct Lua;
impl Lua {
    pub fn info() -> Language {
        Language {
            name: "Lua",
            type_: "programming",
            color: "#000080",
            extensions: &[
                ".lua",
                ".fcgi",
                ".nse",
                ".p8",
                ".pd_lua",
                ".rbxs",
                ".rockspec",
                ".wlua",
            ],
            tm_scope: "source.lua",
            ace_mode: "lua",
            language_id: 213,
        }
    }
}
pub struct Luau;
impl Luau {
    pub fn info() -> Language {
        Language {
            name: "Luau",
            type_: "programming",
            color: "#00A2FF",
            extensions: &[".luau"],
            tm_scope: "source.luau",
            ace_mode: "lua",
            language_id: 365050359,
        }
    }
}
pub struct M;
impl M {
    pub fn info() -> Language {
        Language {
            name: "M",
            type_: "programming",
            color: "",
            extensions: &[".mumps", ".m"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 214,
        }
    }
}
pub struct M4;
impl M4 {
    pub fn info() -> Language {
        Language {
            name: "M4",
            type_: "programming",
            color: "",
            extensions: &[".m4", ".mc"],
            tm_scope: "source.m4",
            ace_mode: "text",
            language_id: 215,
        }
    }
}
pub struct M4Sugar;
impl M4Sugar {
    pub fn info() -> Language {
        Language {
            name: "M4Sugar",
            type_: "programming",
            color: "",
            extensions: &[".m4"],
            tm_scope: "source.m4",
            ace_mode: "text",
            language_id: 216,
        }
    }
}
pub struct MATLAB;
impl MATLAB {
    pub fn info() -> Language {
        Language {
            name: "MATLAB",
            type_: "programming",
            color: "#e16737",
            extensions: &[".matlab", ".m"],
            tm_scope: "source.matlab",
            ace_mode: "matlab",
            language_id: 225,
        }
    }
}
pub struct MAXScript;
impl MAXScript {
    pub fn info() -> Language {
        Language {
            name: "MAXScript",
            type_: "programming",
            color: "#00a6a6",
            extensions: &[".ms", ".mcr"],
            tm_scope: "source.maxscript",
            ace_mode: "text",
            language_id: 217,
        }
    }
}
pub struct MDX;
impl MDX {
    pub fn info() -> Language {
        Language {
            name: "MDX",
            type_: "markup",
            color: "#fcb32c",
            extensions: &[".mdx"],
            tm_scope: "source.mdx",
            ace_mode: "markdown",
            language_id: 512838272,
        }
    }
}
pub struct MLIR;
impl MLIR {
    pub fn info() -> Language {
        Language {
            name: "MLIR",
            type_: "programming",
            color: "#5EC8DB",
            extensions: &[".mlir"],
            tm_scope: "source.mlir",
            ace_mode: "text",
            language_id: 448253929,
        }
    }
}
pub struct MQL4;
impl MQL4 {
    pub fn info() -> Language {
        Language {
            name: "MQL4",
            type_: "programming",
            color: "#62A8D6",
            extensions: &[".mq4", ".mqh"],
            tm_scope: "source.mql5",
            ace_mode: "c_cpp",
            language_id: 426,
        }
    }
}
pub struct MQL5;
impl MQL5 {
    pub fn info() -> Language {
        Language {
            name: "MQL5",
            type_: "programming",
            color: "#4A76B8",
            extensions: &[".mq5", ".mqh"],
            tm_scope: "source.mql5",
            ace_mode: "c_cpp",
            language_id: 427,
        }
    }
}
pub struct MTML;
impl MTML {
    pub fn info() -> Language {
        Language {
            name: "MTML",
            type_: "markup",
            color: "#b7e1f4",
            extensions: &[".mtml"],
            tm_scope: "text.html.basic",
            ace_mode: "html",
            language_id: 218,
        }
    }
}
pub struct MUF;
impl MUF {
    pub fn info() -> Language {
        Language {
            name: "MUF",
            type_: "programming",
            color: "",
            extensions: &[".muf", ".m"],
            tm_scope: "none",
            ace_mode: "forth",
            language_id: 219,
        }
    }
}
pub struct Macaulay2;
impl Macaulay2 {
    pub fn info() -> Language {
        Language {
            name: "Macaulay2",
            type_: "programming",
            color: "#d8ffff",
            extensions: &[".m2"],
            tm_scope: "source.m2",
            ace_mode: "text",
            language_id: 34167825,
        }
    }
}
pub struct Makefile;
impl Makefile {
    pub fn info() -> Language {
        Language {
            name: "Makefile",
            type_: "programming",
            color: "#427819",
            extensions: &[".mak", ".d", ".make", ".makefile", ".mk", ".mkfile"],
            tm_scope: "source.makefile",
            ace_mode: "makefile",
            language_id: 220,
        }
    }
}
pub struct Mako;
impl Mako {
    pub fn info() -> Language {
        Language {
            name: "Mako",
            type_: "programming",
            color: "#7e858d",
            extensions: &[".mako", ".mao"],
            tm_scope: "text.html.mako",
            ace_mode: "text",
            language_id: 221,
        }
    }
}
pub struct Markdown;
impl Markdown {
    pub fn info() -> Language {
        Language {
            name: "Markdown",
            type_: "prose",
            color: "#083fa1",
            extensions: &[
                ".md",
                ".livemd",
                ".markdown",
                ".mdown",
                ".mdwn",
                ".mkd",
                ".mkdn",
                ".mkdown",
                ".ronn",
                ".scd",
                ".workbook",
            ],
            tm_scope: "text.md",
            ace_mode: "markdown",
            language_id: 222,
        }
    }
}
pub struct Marko;
impl Marko {
    pub fn info() -> Language {
        Language {
            name: "Marko",
            type_: "markup",
            color: "#42bff2",
            extensions: &[".marko"],
            tm_scope: "text.marko",
            ace_mode: "text",
            language_id: 932782397,
        }
    }
}
pub struct Mask;
impl Mask {
    pub fn info() -> Language {
        Language {
            name: "Mask",
            type_: "markup",
            color: "#f97732",
            extensions: &[".mask"],
            tm_scope: "source.mask",
            ace_mode: "mask",
            language_id: 223,
        }
    }
}
pub struct Mathematica;
impl Mathematica {
    pub fn info() -> Language {
        Language {
            name: "Mathematica",
            type_: "programming",
            color: "#dd1100",
            extensions: &[
                ".mathematica",
                ".cdf",
                ".m",
                ".ma",
                ".mt",
                ".nb",
                ".nbp",
                ".wl",
                ".wlt",
            ],
            tm_scope: "source.mathematica",
            ace_mode: "text",
            language_id: 224,
        }
    }
}
pub struct MavenPOM;
impl MavenPOM {
    pub fn info() -> Language {
        Language {
            name: "Maven POM",
            type_: "data",
            color: "",
            extensions: &[],
            tm_scope: "text.xml.pom",
            ace_mode: "xml",
            language_id: 226,
        }
    }
}
pub struct Max;
impl Max {
    pub fn info() -> Language {
        Language {
            name: "Max",
            type_: "programming",
            color: "#c4a79c",
            extensions: &[".maxpat", ".maxhelp", ".maxproj", ".mxt", ".pat"],
            tm_scope: "source.json",
            ace_mode: "json",
            language_id: 227,
        }
    }
}
pub struct Mercury;
impl Mercury {
    pub fn info() -> Language {
        Language {
            name: "Mercury",
            type_: "programming",
            color: "#ff2b2b",
            extensions: &[".m", ".moo"],
            tm_scope: "source.mercury",
            ace_mode: "prolog",
            language_id: 229,
        }
    }
}
pub struct Mermaid;
impl Mermaid {
    pub fn info() -> Language {
        Language {
            name: "Mermaid",
            type_: "markup",
            color: "#ff3670",
            extensions: &[".mmd", ".mermaid"],
            tm_scope: "source.mermaid",
            ace_mode: "text",
            language_id: 385992043,
        }
    }
}
pub struct Meson;
impl Meson {
    pub fn info() -> Language {
        Language {
            name: "Meson",
            type_: "programming",
            color: "#007800",
            extensions: &[],
            tm_scope: "source.meson",
            ace_mode: "text",
            language_id: 799141244,
        }
    }
}
pub struct Metal;
impl Metal {
    pub fn info() -> Language {
        Language {
            name: "Metal",
            type_: "programming",
            color: "#8f14e9",
            extensions: &[".metal"],
            tm_scope: "source.c++",
            ace_mode: "c_cpp",
            language_id: 230,
        }
    }
}
pub struct MicrosoftDeveloperStudioProject;
impl MicrosoftDeveloperStudioProject {
    pub fn info() -> Language {
        Language {
            name: "Microsoft Developer Studio Project",
            type_: "data",
            color: "",
            extensions: &[".dsp"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 800983837,
        }
    }
}
pub struct MicrosoftVisualStudioSolution;
impl MicrosoftVisualStudioSolution {
    pub fn info() -> Language {
        Language {
            name: "Microsoft Visual Studio Solution",
            type_: "data",
            color: "",
            extensions: &[".sln"],
            tm_scope: "source.solution",
            ace_mode: "text",
            language_id: 849523096,
        }
    }
}
pub struct MiniD;
impl MiniD {
    pub fn info() -> Language {
        Language {
            name: "MiniD",
            type_: "programming",
            color: "",
            extensions: &[".minid"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 231,
        }
    }
}
pub struct MiniYAML;
impl MiniYAML {
    pub fn info() -> Language {
        Language {
            name: "MiniYAML",
            type_: "data",
            color: "#ff1111",
            extensions: &[".yaml", ".yml"],
            tm_scope: "source.miniyaml",
            ace_mode: "yaml",
            language_id: 4896465,
        }
    }
}
pub struct Mint;
impl Mint {
    pub fn info() -> Language {
        Language {
            name: "Mint",
            type_: "programming",
            color: "#02b046",
            extensions: &[".mint"],
            tm_scope: "source.mint",
            ace_mode: "text",
            language_id: 968740319,
        }
    }
}
pub struct Mirah;
impl Mirah {
    pub fn info() -> Language {
        Language {
            name: "Mirah",
            type_: "programming",
            color: "#c7a938",
            extensions: &[".druby", ".duby", ".mirah"],
            tm_scope: "source.ruby",
            ace_mode: "ruby",
            language_id: 232,
        }
    }
}
pub struct Modelica;
impl Modelica {
    pub fn info() -> Language {
        Language {
            name: "Modelica",
            type_: "programming",
            color: "#de1d31",
            extensions: &[".mo"],
            tm_scope: "source.modelica",
            ace_mode: "text",
            language_id: 233,
        }
    }
}
pub struct Modula2;
impl Modula2 {
    pub fn info() -> Language {
        Language {
            name: "Modula-2",
            type_: "programming",
            color: "#10253f",
            extensions: &[".mod"],
            tm_scope: "source.modula2",
            ace_mode: "text",
            language_id: 234,
        }
    }
}
pub struct Modula3;
impl Modula3 {
    pub fn info() -> Language {
        Language {
            name: "Modula-3",
            type_: "programming",
            color: "#223388",
            extensions: &[".i3", ".ig", ".m3", ".mg"],
            tm_scope: "source.modula-3",
            ace_mode: "text",
            language_id: 564743864,
        }
    }
}
pub struct ModuleManagementSystem;
impl ModuleManagementSystem {
    pub fn info() -> Language {
        Language {
            name: "Module Management System",
            type_: "programming",
            color: "",
            extensions: &[".mms", ".mmk"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 235,
        }
    }
}
pub struct Mojo;
impl Mojo {
    pub fn info() -> Language {
        Language {
            name: "Mojo",
            type_: "programming",
            color: "#ff4c1f",
            extensions: &[".mojo"],
            tm_scope: "source.mojo",
            ace_mode: "python",
            language_id: 1045019587,
        }
    }
}
pub struct Monkey;
impl Monkey {
    pub fn info() -> Language {
        Language {
            name: "Monkey",
            type_: "programming",
            color: "",
            extensions: &[".monkey", ".monkey2"],
            tm_scope: "source.monkey",
            ace_mode: "text",
            language_id: 236,
        }
    }
}
pub struct MonkeyC;
impl MonkeyC {
    pub fn info() -> Language {
        Language {
            name: "Monkey C",
            type_: "programming",
            color: "#8D6747",
            extensions: &[".mc"],
            tm_scope: "source.mc",
            ace_mode: "c_cpp",
            language_id: 231751931,
        }
    }
}
pub struct Moocode;
impl Moocode {
    pub fn info() -> Language {
        Language {
            name: "Moocode",
            type_: "programming",
            color: "",
            extensions: &[".moo"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 237,
        }
    }
}
pub struct MoonScript;
impl MoonScript {
    pub fn info() -> Language {
        Language {
            name: "MoonScript",
            type_: "programming",
            color: "#ff4585",
            extensions: &[".moon"],
            tm_scope: "source.moonscript",
            ace_mode: "text",
            language_id: 238,
        }
    }
}
pub struct Motoko;
impl Motoko {
    pub fn info() -> Language {
        Language {
            name: "Motoko",
            type_: "programming",
            color: "#fbb03b",
            extensions: &[".mo"],
            tm_scope: "source.mo",
            ace_mode: "text",
            language_id: 202937027,
        }
    }
}
pub struct Motorola68KAssembly;
impl Motorola68KAssembly {
    pub fn info() -> Language {
        Language {
            name: "Motorola 68K Assembly",
            type_: "programming",
            color: "#005daa",
            extensions: &[".asm", ".i", ".inc", ".s", ".x68"],
            tm_scope: "source.m68k",
            ace_mode: "assembly_x86",
            language_id: 477582706,
        }
    }
}
pub struct Move;
impl Move {
    pub fn info() -> Language {
        Language {
            name: "Move",
            type_: "programming",
            color: "#4a137a",
            extensions: &[".move"],
            tm_scope: "source.move",
            ace_mode: "text",
            language_id: 638334599,
        }
    }
}
pub struct Muse;
impl Muse {
    pub fn info() -> Language {
        Language {
            name: "Muse",
            type_: "prose",
            color: "",
            extensions: &[".muse"],
            tm_scope: "text.muse",
            ace_mode: "text",
            language_id: 474864066,
        }
    }
}
pub struct Mustache;
impl Mustache {
    pub fn info() -> Language {
        Language {
            name: "Mustache",
            type_: "markup",
            color: "#724b3b",
            extensions: &[".mustache"],
            tm_scope: "text.html.smarty",
            ace_mode: "smarty",
            language_id: 638334590,
        }
    }
}
pub struct Myghty;
impl Myghty {
    pub fn info() -> Language {
        Language {
            name: "Myghty",
            type_: "programming",
            color: "",
            extensions: &[".myt"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 239,
        }
    }
}
pub struct NASL;
impl NASL {
    pub fn info() -> Language {
        Language {
            name: "NASL",
            type_: "programming",
            color: "",
            extensions: &[".nasl", ".inc"],
            tm_scope: "source.nasl",
            ace_mode: "text",
            language_id: 171666519,
        }
    }
}
pub struct NCL;
impl NCL {
    pub fn info() -> Language {
        Language {
            name: "NCL",
            type_: "programming",
            color: "#28431f",
            extensions: &[".ncl"],
            tm_scope: "source.ncl",
            ace_mode: "text",
            language_id: 240,
        }
    }
}
pub struct NEON;
impl NEON {
    pub fn info() -> Language {
        Language {
            name: "NEON",
            type_: "data",
            color: "",
            extensions: &[".neon"],
            tm_scope: "source.neon",
            ace_mode: "text",
            language_id: 481192983,
        }
    }
}
pub struct NL;
impl NL {
    pub fn info() -> Language {
        Language {
            name: "NL",
            type_: "data",
            color: "",
            extensions: &[".nl"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 241,
        }
    }
}
pub struct NMODL;
impl NMODL {
    pub fn info() -> Language {
        Language {
            name: "NMODL",
            type_: "programming",
            color: "#00356B",
            extensions: &[".mod"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 136456478,
        }
    }
}
pub struct NPMConfig;
impl NPMConfig {
    pub fn info() -> Language {
        Language {
            name: "NPM Config",
            type_: "data",
            color: "#cb3837",
            extensions: &[],
            tm_scope: "source.ini.npmrc",
            ace_mode: "text",
            language_id: 685022663,
        }
    }
}
pub struct NSIS;
impl NSIS {
    pub fn info() -> Language {
        Language {
            name: "NSIS",
            type_: "programming",
            color: "",
            extensions: &[".nsi", ".nsh"],
            tm_scope: "source.nsis",
            ace_mode: "text",
            language_id: 242,
        }
    }
}
pub struct NWScript;
impl NWScript {
    pub fn info() -> Language {
        Language {
            name: "NWScript",
            type_: "programming",
            color: "#111522",
            extensions: &[".nss"],
            tm_scope: "source.c.nwscript",
            ace_mode: "c_cpp",
            language_id: 731233819,
        }
    }
}
pub struct Nasal;
impl Nasal {
    pub fn info() -> Language {
        Language {
            name: "Nasal",
            type_: "programming",
            color: "#1d2c4e",
            extensions: &[".nas"],
            tm_scope: "source.nasal",
            ace_mode: "nasal",
            language_id: 178322513,
        }
    }
}
pub struct Nearley;
impl Nearley {
    pub fn info() -> Language {
        Language {
            name: "Nearley",
            type_: "programming",
            color: "#990000",
            extensions: &[".ne", ".nearley"],
            tm_scope: "source.ne",
            ace_mode: "text",
            language_id: 521429430,
        }
    }
}
pub struct Nemerle;
impl Nemerle {
    pub fn info() -> Language {
        Language {
            name: "Nemerle",
            type_: "programming",
            color: "#3d3c6e",
            extensions: &[".n"],
            tm_scope: "source.nemerle",
            ace_mode: "text",
            language_id: 243,
        }
    }
}
pub struct NetLinx;
impl NetLinx {
    pub fn info() -> Language {
        Language {
            name: "NetLinx",
            type_: "programming",
            color: "#0aa0ff",
            extensions: &[".axs", ".axi"],
            tm_scope: "source.netlinx",
            ace_mode: "text",
            language_id: 244,
        }
    }
}
pub struct NetLinxpERB;
impl NetLinxpERB {
    pub fn info() -> Language {
        Language {
            name: "NetLinx+ERB",
            type_: "programming",
            color: "#747faa",
            extensions: &[".axs.erb", ".axi.erb"],
            tm_scope: "source.netlinx.erb",
            ace_mode: "text",
            language_id: 245,
        }
    }
}
pub struct NetLogo;
impl NetLogo {
    pub fn info() -> Language {
        Language {
            name: "NetLogo",
            type_: "programming",
            color: "#ff6375",
            extensions: &[".nlogo"],
            tm_scope: "source.lisp",
            ace_mode: "lisp",
            language_id: 246,
        }
    }
}
pub struct NewLisp;
impl NewLisp {
    pub fn info() -> Language {
        Language {
            name: "NewLisp",
            type_: "programming",
            color: "#87AED7",
            extensions: &[".nl", ".lisp", ".lsp"],
            tm_scope: "source.lisp",
            ace_mode: "lisp",
            language_id: 247,
        }
    }
}
pub struct Nextflow;
impl Nextflow {
    pub fn info() -> Language {
        Language {
            name: "Nextflow",
            type_: "programming",
            color: "#3ac486",
            extensions: &[".nf"],
            tm_scope: "source.nextflow",
            ace_mode: "groovy",
            language_id: 506780613,
        }
    }
}
pub struct Nginx;
impl Nginx {
    pub fn info() -> Language {
        Language {
            name: "Nginx",
            type_: "data",
            color: "#009639",
            extensions: &[".nginx", ".nginxconf", ".vhost"],
            tm_scope: "source.nginx",
            ace_mode: "text",
            language_id: 248,
        }
    }
}
pub struct Nim;
impl Nim {
    pub fn info() -> Language {
        Language {
            name: "Nim",
            type_: "programming",
            color: "#ffc200",
            extensions: &[".nim", ".nim.cfg", ".nimble", ".nimrod", ".nims"],
            tm_scope: "source.nim",
            ace_mode: "text",
            language_id: 249,
        }
    }
}
pub struct Ninja;
impl Ninja {
    pub fn info() -> Language {
        Language {
            name: "Ninja",
            type_: "data",
            color: "",
            extensions: &[".ninja"],
            tm_scope: "source.ninja",
            ace_mode: "text",
            language_id: 250,
        }
    }
}
pub struct Nit;
impl Nit {
    pub fn info() -> Language {
        Language {
            name: "Nit",
            type_: "programming",
            color: "#009917",
            extensions: &[".nit"],
            tm_scope: "source.nit",
            ace_mode: "text",
            language_id: 251,
        }
    }
}
pub struct Nix;
impl Nix {
    pub fn info() -> Language {
        Language {
            name: "Nix",
            type_: "programming",
            color: "#7e7eff",
            extensions: &[".nix"],
            tm_scope: "source.nix",
            ace_mode: "nix",
            language_id: 252,
        }
    }
}
pub struct Noir;
impl Noir {
    pub fn info() -> Language {
        Language {
            name: "Noir",
            type_: "programming",
            color: "#2f1f49",
            extensions: &[".nr"],
            tm_scope: "source.nr",
            ace_mode: "rust",
            language_id: 813068465,
        }
    }
}
pub struct Nu;
impl Nu {
    pub fn info() -> Language {
        Language {
            name: "Nu",
            type_: "programming",
            color: "#c9df40",
            extensions: &[".nu"],
            tm_scope: "source.nu",
            ace_mode: "scheme",
            language_id: 253,
        }
    }
}
pub struct NumPy;
impl NumPy {
    pub fn info() -> Language {
        Language {
            name: "NumPy",
            type_: "programming",
            color: "#9C8AF9",
            extensions: &[".numpy", ".numpyw", ".numsc"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 254,
        }
    }
}
pub struct Nunjucks;
impl Nunjucks {
    pub fn info() -> Language {
        Language {
            name: "Nunjucks",
            type_: "markup",
            color: "#3d8137",
            extensions: &[".njk"],
            tm_scope: "text.html.nunjucks",
            ace_mode: "nunjucks",
            language_id: 461856962,
        }
    }
}
pub struct Nushell;
impl Nushell {
    pub fn info() -> Language {
        Language {
            name: "Nushell",
            type_: "programming",
            color: "#4E9906",
            extensions: &[".nu"],
            tm_scope: "source.nushell",
            ace_mode: "sh",
            language_id: 446573572,
        }
    }
}
pub struct OASv2Json;
impl OASv2Json {
    pub fn info() -> Language {
        Language {
            name: "OASv2-json",
            type_: "data",
            color: "#85ea2d",
            extensions: &[".json"],
            tm_scope: "source.json",
            ace_mode: "json",
            language_id: 834374816,
        }
    }
}
pub struct OASv2Yaml;
impl OASv2Yaml {
    pub fn info() -> Language {
        Language {
            name: "OASv2-yaml",
            type_: "data",
            color: "#85ea2d",
            extensions: &[".yaml", ".yml"],
            tm_scope: "source.yaml",
            ace_mode: "yaml",
            language_id: 105187618,
        }
    }
}
pub struct OASv3Json;
impl OASv3Json {
    pub fn info() -> Language {
        Language {
            name: "OASv3-json",
            type_: "data",
            color: "#85ea2d",
            extensions: &[".json"],
            tm_scope: "source.json",
            ace_mode: "json",
            language_id: 980062566,
        }
    }
}
pub struct OASv3Yaml;
impl OASv3Yaml {
    pub fn info() -> Language {
        Language {
            name: "OASv3-yaml",
            type_: "data",
            color: "#85ea2d",
            extensions: &[".yaml", ".yml"],
            tm_scope: "source.yaml",
            ace_mode: "yaml",
            language_id: 51239111,
        }
    }
}
pub struct OCaml;
impl OCaml {
    pub fn info() -> Language {
        Language {
            name: "OCaml",
            type_: "programming",
            color: "#ef7a08",
            extensions: &[".ml", ".eliom", ".eliomi", ".ml4", ".mli", ".mll", ".mly"],
            tm_scope: "source.ocaml",
            ace_mode: "ocaml",
            language_id: 255,
        }
    }
}
pub struct Oberon;
impl Oberon {
    pub fn info() -> Language {
        Language {
            name: "Oberon",
            type_: "programming",
            color: "",
            extensions: &[".ob2"],
            tm_scope: "source.modula2",
            ace_mode: "text",
            language_id: 677210597,
        }
    }
}
pub struct ObjDump;
impl ObjDump {
    pub fn info() -> Language {
        Language {
            name: "ObjDump",
            type_: "data",
            color: "",
            extensions: &[".objdump"],
            tm_scope: "objdump.x86asm",
            ace_mode: "assembly_x86",
            language_id: 256,
        }
    }
}
pub struct ObjectDataInstanceNotation;
impl ObjectDataInstanceNotation {
    pub fn info() -> Language {
        Language {
            name: "Object Data Instance Notation",
            type_: "data",
            color: "",
            extensions: &[".odin"],
            tm_scope: "source.odin-ehr",
            ace_mode: "text",
            language_id: 985227236,
        }
    }
}
pub struct ObjectScript;
impl ObjectScript {
    pub fn info() -> Language {
        Language {
            name: "ObjectScript",
            type_: "programming",
            color: "#424893",
            extensions: &[".cls"],
            tm_scope: "source.objectscript",
            ace_mode: "text",
            language_id: 202735509,
        }
    }
}
pub struct ObjectiveC;
impl ObjectiveC {
    pub fn info() -> Language {
        Language {
            name: "Objective-C",
            type_: "programming",
            color: "#438eff",
            extensions: &[".m", ".h"],
            tm_scope: "source.objc",
            ace_mode: "objectivec",
            language_id: 257,
        }
    }
}
pub struct ObjectiveCpp;
impl ObjectiveCpp {
    pub fn info() -> Language {
        Language {
            name: "Objective-C++",
            type_: "programming",
            color: "#6866fb",
            extensions: &[".mm"],
            tm_scope: "source.objc++",
            ace_mode: "objectivec",
            language_id: 258,
        }
    }
}
pub struct ObjectiveJ;
impl ObjectiveJ {
    pub fn info() -> Language {
        Language {
            name: "Objective-J",
            type_: "programming",
            color: "#ff0c5a",
            extensions: &[".j", ".sj"],
            tm_scope: "source.js.objj",
            ace_mode: "text",
            language_id: 259,
        }
    }
}
pub struct Odin;
impl Odin {
    pub fn info() -> Language {
        Language {
            name: "Odin",
            type_: "programming",
            color: "#60AFFE",
            extensions: &[".odin"],
            tm_scope: "source.odin",
            ace_mode: "text",
            language_id: 889244082,
        }
    }
}
pub struct Omgrofl;
impl Omgrofl {
    pub fn info() -> Language {
        Language {
            name: "Omgrofl",
            type_: "programming",
            color: "#cabbff",
            extensions: &[".omgrofl"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 260,
        }
    }
}
pub struct Opa;
impl Opa {
    pub fn info() -> Language {
        Language {
            name: "Opa",
            type_: "programming",
            color: "",
            extensions: &[".opa"],
            tm_scope: "source.opa",
            ace_mode: "text",
            language_id: 261,
        }
    }
}
pub struct Opal;
impl Opal {
    pub fn info() -> Language {
        Language {
            name: "Opal",
            type_: "programming",
            color: "#f7ede0",
            extensions: &[".opal"],
            tm_scope: "source.opal",
            ace_mode: "text",
            language_id: 262,
        }
    }
}
pub struct OpenPolicyAgent;
impl OpenPolicyAgent {
    pub fn info() -> Language {
        Language {
            name: "Open Policy Agent",
            type_: "programming",
            color: "#7d9199",
            extensions: &[".rego"],
            tm_scope: "source.rego",
            ace_mode: "text",
            language_id: 840483232,
        }
    }
}
pub struct OpenAPISpecificationV2;
impl OpenAPISpecificationV2 {
    pub fn info() -> Language {
        Language {
            name: "OpenAPI Specification v2",
            type_: "data",
            color: "#85ea2d",
            extensions: &[],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 848295328,
        }
    }
}
pub struct OpenAPISpecificationV3;
impl OpenAPISpecificationV3 {
    pub fn info() -> Language {
        Language {
            name: "OpenAPI Specification v3",
            type_: "data",
            color: "#85ea2d",
            extensions: &[],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 557959099,
        }
    }
}
pub struct OpenCL;
impl OpenCL {
    pub fn info() -> Language {
        Language {
            name: "OpenCL",
            type_: "programming",
            color: "#ed2e2d",
            extensions: &[".cl", ".opencl"],
            tm_scope: "source.c",
            ace_mode: "c_cpp",
            language_id: 263,
        }
    }
}
pub struct OpenEdgeABL;
impl OpenEdgeABL {
    pub fn info() -> Language {
        Language {
            name: "OpenEdge ABL",
            type_: "programming",
            color: "#5ce600",
            extensions: &[".p", ".cls", ".w"],
            tm_scope: "source.abl",
            ace_mode: "text",
            language_id: 264,
        }
    }
}
pub struct OpenQASM;
impl OpenQASM {
    pub fn info() -> Language {
        Language {
            name: "OpenQASM",
            type_: "programming",
            color: "#AA70FF",
            extensions: &[".qasm"],
            tm_scope: "source.qasm",
            ace_mode: "text",
            language_id: 153739399,
        }
    }
}
pub struct OpenRCRunscript;
impl OpenRCRunscript {
    pub fn info() -> Language {
        Language {
            name: "OpenRC runscript",
            type_: "programming",
            color: "",
            extensions: &[],
            tm_scope: "source.shell",
            ace_mode: "sh",
            language_id: 265,
        }
    }
}
pub struct OpenSCAD;
impl OpenSCAD {
    pub fn info() -> Language {
        Language {
            name: "OpenSCAD",
            type_: "programming",
            color: "#e5cd45",
            extensions: &[".scad"],
            tm_scope: "source.scad",
            ace_mode: "scad",
            language_id: 266,
        }
    }
}
pub struct OpenStepPropertyList;
impl OpenStepPropertyList {
    pub fn info() -> Language {
        Language {
            name: "OpenStep Property List",
            type_: "data",
            color: "",
            extensions: &[".plist", ".glyphs"],
            tm_scope: "source.plist",
            ace_mode: "text",
            language_id: 598917541,
        }
    }
}
pub struct OpenTypeFeatureFile;
impl OpenTypeFeatureFile {
    pub fn info() -> Language {
        Language {
            name: "OpenType Feature File",
            type_: "data",
            color: "",
            extensions: &[".fea"],
            tm_scope: "source.opentype",
            ace_mode: "text",
            language_id: 374317347,
        }
    }
}
pub struct OptionList;
impl OptionList {
    pub fn info() -> Language {
        Language {
            name: "Option List",
            type_: "data",
            color: "#476732",
            extensions: &[],
            tm_scope: "source.opts",
            ace_mode: "sh",
            language_id: 723589315,
        }
    }
}
pub struct Org;
impl Org {
    pub fn info() -> Language {
        Language {
            name: "Org",
            type_: "prose",
            color: "#77aa99",
            extensions: &[".org"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 267,
        }
    }
}
pub struct Ox;
impl Ox {
    pub fn info() -> Language {
        Language {
            name: "Ox",
            type_: "programming",
            color: "",
            extensions: &[".ox", ".oxh", ".oxo"],
            tm_scope: "source.ox",
            ace_mode: "text",
            language_id: 268,
        }
    }
}
pub struct Oxygene;
impl Oxygene {
    pub fn info() -> Language {
        Language {
            name: "Oxygene",
            type_: "programming",
            color: "#cdd0e3",
            extensions: &[".oxygene"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 269,
        }
    }
}
pub struct Oz;
impl Oz {
    pub fn info() -> Language {
        Language {
            name: "Oz",
            type_: "programming",
            color: "#fab738",
            extensions: &[".oz"],
            tm_scope: "source.oz",
            ace_mode: "text",
            language_id: 270,
        }
    }
}
pub struct P4;
impl P4 {
    pub fn info() -> Language {
        Language {
            name: "P4",
            type_: "programming",
            color: "#7055b5",
            extensions: &[".p4"],
            tm_scope: "source.p4",
            ace_mode: "text",
            language_id: 348895984,
        }
    }
}
pub struct PDDL;
impl PDDL {
    pub fn info() -> Language {
        Language {
            name: "PDDL",
            type_: "programming",
            color: "#0d00ff",
            extensions: &[".pddl"],
            tm_scope: "source.pddl",
            ace_mode: "text",
            language_id: 736235603,
        }
    }
}
pub struct PEGjs;
impl PEGjs {
    pub fn info() -> Language {
        Language {
            name: "PEG.js",
            type_: "programming",
            color: "#234d6b",
            extensions: &[".pegjs", ".peggy"],
            tm_scope: "source.peggy",
            ace_mode: "javascript",
            language_id: 81442128,
        }
    }
}
pub struct PHP;
impl PHP {
    pub fn info() -> Language {
        Language {
            name: "PHP",
            type_: "programming",
            color: "#4F5D95",
            extensions: &[
                ".php",
                ".aw",
                ".ctp",
                ".fcgi",
                ".inc",
                ".php3",
                ".php4",
                ".php5",
                ".phps",
                ".phpt",
            ],
            tm_scope: "text.html.php",
            ace_mode: "php",
            language_id: 272,
        }
    }
}
pub struct PLSQL;
impl PLSQL {
    pub fn info() -> Language {
        Language {
            name: "PLSQL",
            type_: "programming",
            color: "#dad8d8",
            extensions: &[
                ".pls",
                ".bdy",
                ".ddl",
                ".fnc",
                ".pck",
                ".pkb",
                ".pks",
                ".plb",
                ".plsql",
                ".prc",
                ".spc",
                ".sql",
                ".tpb",
                ".tps",
                ".trg",
                ".vw",
            ],
            tm_scope: "none",
            ace_mode: "sql",
            language_id: 273,
        }
    }
}
pub struct PLpgSQL;
impl PLpgSQL {
    pub fn info() -> Language {
        Language {
            name: "PLpgSQL",
            type_: "programming",
            color: "#336790",
            extensions: &[".pgsql", ".sql"],
            tm_scope: "source.sql",
            ace_mode: "pgsql",
            language_id: 274,
        }
    }
}
pub struct POVRaySDL;
impl POVRaySDL {
    pub fn info() -> Language {
        Language {
            name: "POV-Ray SDL",
            type_: "programming",
            color: "#6bac65",
            extensions: &[".pov", ".inc"],
            tm_scope: "source.pov-ray sdl",
            ace_mode: "text",
            language_id: 275,
        }
    }
}
pub struct Pact;
impl Pact {
    pub fn info() -> Language {
        Language {
            name: "Pact",
            type_: "programming",
            color: "#F7A8B8",
            extensions: &[".pact"],
            tm_scope: "source.pact",
            ace_mode: "text",
            language_id: 756774415,
        }
    }
}
pub struct Pan;
impl Pan {
    pub fn info() -> Language {
        Language {
            name: "Pan",
            type_: "programming",
            color: "#cc0000",
            extensions: &[".pan"],
            tm_scope: "source.pan",
            ace_mode: "text",
            language_id: 276,
        }
    }
}
pub struct Papyrus;
impl Papyrus {
    pub fn info() -> Language {
        Language {
            name: "Papyrus",
            type_: "programming",
            color: "#6600cc",
            extensions: &[".psc"],
            tm_scope: "source.papyrus.skyrim",
            ace_mode: "text",
            language_id: 277,
        }
    }
}
pub struct Parrot;
impl Parrot {
    pub fn info() -> Language {
        Language {
            name: "Parrot",
            type_: "programming",
            color: "#f3ca0a",
            extensions: &[".parrot"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 278,
        }
    }
}
pub struct ParrotAssembly;
impl ParrotAssembly {
    pub fn info() -> Language {
        Language {
            name: "Parrot Assembly",
            type_: "programming",
            color: "",
            extensions: &[".pasm"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 279,
        }
    }
}
pub struct ParrotInternalRepresentation;
impl ParrotInternalRepresentation {
    pub fn info() -> Language {
        Language {
            name: "Parrot Internal Representation",
            type_: "programming",
            color: "",
            extensions: &[".pir"],
            tm_scope: "source.parrot.pir",
            ace_mode: "text",
            language_id: 280,
        }
    }
}
pub struct Pascal;
impl Pascal {
    pub fn info() -> Language {
        Language {
            name: "Pascal",
            type_: "programming",
            color: "#E3F171",
            extensions: &[".pas", ".dfm", ".dpr", ".inc", ".lpr", ".pascal", ".pp"],
            tm_scope: "source.pascal",
            ace_mode: "pascal",
            language_id: 281,
        }
    }
}
pub struct Pawn;
impl Pawn {
    pub fn info() -> Language {
        Language {
            name: "Pawn",
            type_: "programming",
            color: "#dbb284",
            extensions: &[".pwn", ".inc", ".sma"],
            tm_scope: "source.pawn",
            ace_mode: "text",
            language_id: 271,
        }
    }
}
pub struct Pep8;
impl Pep8 {
    pub fn info() -> Language {
        Language {
            name: "Pep8",
            type_: "programming",
            color: "#C76F5B",
            extensions: &[".pep"],
            tm_scope: "source.pep8",
            ace_mode: "text",
            language_id: 840372442,
        }
    }
}
pub struct Perl;
impl Perl {
    pub fn info() -> Language {
        Language {
            name: "Perl",
            type_: "programming",
            color: "#0298c3",
            extensions: &[
                ".pl",
                ".al",
                ".cgi",
                ".fcgi",
                ".perl",
                ".ph",
                ".plx",
                ".pm",
                ".psgi",
                ".t",
            ],
            tm_scope: "source.perl",
            ace_mode: "perl",
            language_id: 282,
        }
    }
}
pub struct Pic;
impl Pic {
    pub fn info() -> Language {
        Language {
            name: "Pic",
            type_: "markup",
            color: "",
            extensions: &[".pic", ".chem"],
            tm_scope: "source.pic",
            ace_mode: "text",
            language_id: 425,
        }
    }
}
pub struct Pickle;
impl Pickle {
    pub fn info() -> Language {
        Language {
            name: "Pickle",
            type_: "data",
            color: "",
            extensions: &[".pkl"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 284,
        }
    }
}
pub struct PicoLisp;
impl PicoLisp {
    pub fn info() -> Language {
        Language {
            name: "PicoLisp",
            type_: "programming",
            color: "#6067af",
            extensions: &[".l"],
            tm_scope: "source.lisp",
            ace_mode: "lisp",
            language_id: 285,
        }
    }
}
pub struct PigLatin;
impl PigLatin {
    pub fn info() -> Language {
        Language {
            name: "PigLatin",
            type_: "programming",
            color: "#fcd7de",
            extensions: &[".pig"],
            tm_scope: "source.pig_latin",
            ace_mode: "text",
            language_id: 286,
        }
    }
}
pub struct Pike;
impl Pike {
    pub fn info() -> Language {
        Language {
            name: "Pike",
            type_: "programming",
            color: "#005390",
            extensions: &[".pike", ".pmod"],
            tm_scope: "source.pike",
            ace_mode: "text",
            language_id: 287,
        }
    }
}
pub struct PipRequirements;
impl PipRequirements {
    pub fn info() -> Language {
        Language {
            name: "Pip Requirements",
            type_: "data",
            color: "#FFD343",
            extensions: &[],
            tm_scope: "source.pip-requirements",
            ace_mode: "text",
            language_id: 684385621,
        }
    }
}
pub struct Pkl;
impl Pkl {
    pub fn info() -> Language {
        Language {
            name: "Pkl",
            type_: "programming",
            color: "#6b9543",
            extensions: &[".pkl"],
            tm_scope: "source.pkl",
            ace_mode: "text",
            language_id: 288822799,
        }
    }
}
pub struct PlantUML;
impl PlantUML {
    pub fn info() -> Language {
        Language {
            name: "PlantUML",
            type_: "data",
            color: "#fbbd16",
            extensions: &[".puml", ".iuml", ".plantuml"],
            tm_scope: "source.wsd",
            ace_mode: "text",
            language_id: 833504686,
        }
    }
}
pub struct Pod;
impl Pod {
    pub fn info() -> Language {
        Language {
            name: "Pod",
            type_: "prose",
            color: "",
            extensions: &[".pod"],
            tm_scope: "none",
            ace_mode: "perl",
            language_id: 288,
        }
    }
}
pub struct Pod6;
impl Pod6 {
    pub fn info() -> Language {
        Language {
            name: "Pod 6",
            type_: "prose",
            color: "",
            extensions: &[".pod", ".pod6"],
            tm_scope: "source.raku",
            ace_mode: "perl",
            language_id: 155357471,
        }
    }
}
pub struct PogoScript;
impl PogoScript {
    pub fn info() -> Language {
        Language {
            name: "PogoScript",
            type_: "programming",
            color: "#d80074",
            extensions: &[".pogo"],
            tm_scope: "source.pogoscript",
            ace_mode: "text",
            language_id: 289,
        }
    }
}
pub struct Polar;
impl Polar {
    pub fn info() -> Language {
        Language {
            name: "Polar",
            type_: "programming",
            color: "#ae81ff",
            extensions: &[".polar"],
            tm_scope: "source.polar",
            ace_mode: "text",
            language_id: 839112914,
        }
    }
}
pub struct Pony;
impl Pony {
    pub fn info() -> Language {
        Language {
            name: "Pony",
            type_: "programming",
            color: "",
            extensions: &[".pony"],
            tm_scope: "source.pony",
            ace_mode: "text",
            language_id: 290,
        }
    }
}
pub struct Portugol;
impl Portugol {
    pub fn info() -> Language {
        Language {
            name: "Portugol",
            type_: "programming",
            color: "#f8bd00",
            extensions: &[".por"],
            tm_scope: "source.portugol",
            ace_mode: "text",
            language_id: 832391833,
        }
    }
}
pub struct PostCSS;
impl PostCSS {
    pub fn info() -> Language {
        Language {
            name: "PostCSS",
            type_: "markup",
            color: "#dc3a0c",
            extensions: &[".pcss", ".postcss"],
            tm_scope: "source.postcss",
            ace_mode: "text",
            language_id: 262764437,
        }
    }
}
pub struct PostScript;
impl PostScript {
    pub fn info() -> Language {
        Language {
            name: "PostScript",
            type_: "markup",
            color: "#da291c",
            extensions: &[".ps", ".eps", ".epsi", ".pfa"],
            tm_scope: "source.postscript",
            ace_mode: "text",
            language_id: 291,
        }
    }
}
pub struct PowerBuilder;
impl PowerBuilder {
    pub fn info() -> Language {
        Language {
            name: "PowerBuilder",
            type_: "programming",
            color: "#8f0f8d",
            extensions: &[".pbt", ".sra", ".sru", ".srw"],
            tm_scope: "source.powerbuilder",
            ace_mode: "text",
            language_id: 292,
        }
    }
}
pub struct PowerShell;
impl PowerShell {
    pub fn info() -> Language {
        Language {
            name: "PowerShell",
            type_: "programming",
            color: "#012456",
            extensions: &[".ps1", ".psd1", ".psm1"],
            tm_scope: "source.powershell",
            ace_mode: "powershell",
            language_id: 293,
        }
    }
}
pub struct Praat;
impl Praat {
    pub fn info() -> Language {
        Language {
            name: "Praat",
            type_: "programming",
            color: "#c8506d",
            extensions: &[".praat"],
            tm_scope: "source.praat",
            ace_mode: "praat",
            language_id: 106029007,
        }
    }
}
pub struct Prisma;
impl Prisma {
    pub fn info() -> Language {
        Language {
            name: "Prisma",
            type_: "data",
            color: "#0c344b",
            extensions: &[".prisma"],
            tm_scope: "source.prisma",
            ace_mode: "text",
            language_id: 499933428,
        }
    }
}
pub struct Processing;
impl Processing {
    pub fn info() -> Language {
        Language {
            name: "Processing",
            type_: "programming",
            color: "#0096D8",
            extensions: &[".pde"],
            tm_scope: "source.processing",
            ace_mode: "text",
            language_id: 294,
        }
    }
}
pub struct Procfile;
impl Procfile {
    pub fn info() -> Language {
        Language {
            name: "Procfile",
            type_: "programming",
            color: "#3B2F63",
            extensions: &[],
            tm_scope: "source.procfile",
            ace_mode: "batchfile",
            language_id: 305313959,
        }
    }
}
pub struct Proguard;
impl Proguard {
    pub fn info() -> Language {
        Language {
            name: "Proguard",
            type_: "data",
            color: "",
            extensions: &[".pro"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 716513858,
        }
    }
}
pub struct Prolog;
impl Prolog {
    pub fn info() -> Language {
        Language {
            name: "Prolog",
            type_: "programming",
            color: "#74283c",
            extensions: &[".pl", ".plt", ".pro", ".prolog", ".yap"],
            tm_scope: "source.prolog",
            ace_mode: "prolog",
            language_id: 295,
        }
    }
}
pub struct Promela;
impl Promela {
    pub fn info() -> Language {
        Language {
            name: "Promela",
            type_: "programming",
            color: "#de0000",
            extensions: &[".pml"],
            tm_scope: "source.promela",
            ace_mode: "text",
            language_id: 441858312,
        }
    }
}
pub struct PropellerSpin;
impl PropellerSpin {
    pub fn info() -> Language {
        Language {
            name: "Propeller Spin",
            type_: "programming",
            color: "#7fa2a7",
            extensions: &[".spin"],
            tm_scope: "source.spin",
            ace_mode: "text",
            language_id: 296,
        }
    }
}
pub struct ProtocolBuffer;
impl ProtocolBuffer {
    pub fn info() -> Language {
        Language {
            name: "Protocol Buffer",
            type_: "data",
            color: "",
            extensions: &[".proto"],
            tm_scope: "source.proto",
            ace_mode: "protobuf",
            language_id: 297,
        }
    }
}
pub struct ProtocolBufferTextFormat;
impl ProtocolBufferTextFormat {
    pub fn info() -> Language {
        Language {
            name: "Protocol Buffer Text Format",
            type_: "data",
            color: "",
            extensions: &[".textproto", ".pbt", ".pbtxt"],
            tm_scope: "source.textproto",
            ace_mode: "text",
            language_id: 436568854,
        }
    }
}
pub struct PublicKey;
impl PublicKey {
    pub fn info() -> Language {
        Language {
            name: "Public Key",
            type_: "data",
            color: "",
            extensions: &[".asc", ".pub"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 298,
        }
    }
}
pub struct Pug;
impl Pug {
    pub fn info() -> Language {
        Language {
            name: "Pug",
            type_: "markup",
            color: "#a86454",
            extensions: &[".jade", ".pug"],
            tm_scope: "text.jade",
            ace_mode: "jade",
            language_id: 179,
        }
    }
}
pub struct Puppet;
impl Puppet {
    pub fn info() -> Language {
        Language {
            name: "Puppet",
            type_: "programming",
            color: "#302B6D",
            extensions: &[".pp"],
            tm_scope: "source.puppet",
            ace_mode: "text",
            language_id: 299,
        }
    }
}
pub struct PureData;
impl PureData {
    pub fn info() -> Language {
        Language {
            name: "Pure Data",
            type_: "data",
            color: "",
            extensions: &[".pd"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 300,
        }
    }
}
pub struct PureBasic;
impl PureBasic {
    pub fn info() -> Language {
        Language {
            name: "PureBasic",
            type_: "programming",
            color: "#5a6986",
            extensions: &[".pb", ".pbi"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 301,
        }
    }
}
pub struct PureScript;
impl PureScript {
    pub fn info() -> Language {
        Language {
            name: "PureScript",
            type_: "programming",
            color: "#1D222D",
            extensions: &[".purs"],
            tm_scope: "source.purescript",
            ace_mode: "haskell",
            language_id: 302,
        }
    }
}
pub struct Pyret;
impl Pyret {
    pub fn info() -> Language {
        Language {
            name: "Pyret",
            type_: "programming",
            color: "#ee1e10",
            extensions: &[".arr"],
            tm_scope: "source.arr",
            ace_mode: "python",
            language_id: 252961827,
        }
    }
}
pub struct Python;
impl Python {
    pub fn info() -> Language {
        Language {
            name: "Python",
            type_: "programming",
            color: "#3572A5",
            extensions: &[
                ".py",
                ".cgi",
                ".fcgi",
                ".gyp",
                ".gypi",
                ".lmi",
                ".py3",
                ".pyde",
                ".pyi",
                ".pyp",
                ".pyt",
                ".pyw",
                ".rpy",
                ".spec",
                ".tac",
                ".wsgi",
                ".xpy",
            ],
            tm_scope: "source.python",
            ace_mode: "python",
            language_id: 303,
        }
    }
}
pub struct PythonConsole;
impl PythonConsole {
    pub fn info() -> Language {
        Language {
            name: "Python console",
            type_: "programming",
            color: "#3572A5",
            extensions: &[],
            tm_scope: "text.python.console",
            ace_mode: "text",
            language_id: 428,
        }
    }
}
pub struct PythonTraceback;
impl PythonTraceback {
    pub fn info() -> Language {
        Language {
            name: "Python traceback",
            type_: "data",
            color: "#3572A5",
            extensions: &[".pytb"],
            tm_scope: "text.python.traceback",
            ace_mode: "text",
            language_id: 304,
        }
    }
}
pub struct Qsharp;
impl Qsharp {
    pub fn info() -> Language {
        Language {
            name: "Q#",
            type_: "programming",
            color: "#fed659",
            extensions: &[".qs"],
            tm_scope: "source.qsharp",
            ace_mode: "text",
            language_id: 697448245,
        }
    }
}
pub struct QML;
impl QML {
    pub fn info() -> Language {
        Language {
            name: "QML",
            type_: "programming",
            color: "#44a51c",
            extensions: &[".qml", ".qbs"],
            tm_scope: "source.qml",
            ace_mode: "text",
            language_id: 305,
        }
    }
}
pub struct QMake;
impl QMake {
    pub fn info() -> Language {
        Language {
            name: "QMake",
            type_: "programming",
            color: "",
            extensions: &[".pro", ".pri"],
            tm_scope: "source.qmake",
            ace_mode: "text",
            language_id: 306,
        }
    }
}
pub struct QtScript;
impl QtScript {
    pub fn info() -> Language {
        Language {
            name: "Qt Script",
            type_: "programming",
            color: "#00b841",
            extensions: &[".qs"],
            tm_scope: "source.js",
            ace_mode: "javascript",
            language_id: 558193693,
        }
    }
}
pub struct Quake;
impl Quake {
    pub fn info() -> Language {
        Language {
            name: "Quake",
            type_: "programming",
            color: "#882233",
            extensions: &[],
            tm_scope: "source.quake",
            ace_mode: "text",
            language_id: 375265331,
        }
    }
}
pub struct R;
impl R {
    pub fn info() -> Language {
        Language {
            name: "R",
            type_: "programming",
            color: "#198CE7",
            extensions: &[".r", ".rd", ".rsx"],
            tm_scope: "source.r",
            ace_mode: "r",
            language_id: 307,
        }
    }
}
pub struct RAML;
impl RAML {
    pub fn info() -> Language {
        Language {
            name: "RAML",
            type_: "markup",
            color: "#77d9fb",
            extensions: &[".raml"],
            tm_scope: "source.yaml",
            ace_mode: "yaml",
            language_id: 308,
        }
    }
}
pub struct RBS;
impl RBS {
    pub fn info() -> Language {
        Language {
            name: "RBS",
            type_: "data",
            color: "#701516",
            extensions: &[".rbs"],
            tm_scope: "source.rbs",
            ace_mode: "ruby",
            language_id: 899227493,
        }
    }
}
pub struct RDoc;
impl RDoc {
    pub fn info() -> Language {
        Language {
            name: "RDoc",
            type_: "prose",
            color: "#701516",
            extensions: &[".rdoc"],
            tm_scope: "text.rdoc",
            ace_mode: "rdoc",
            language_id: 309,
        }
    }
}
pub struct REALbasic;
impl REALbasic {
    pub fn info() -> Language {
        Language {
            name: "REALbasic",
            type_: "programming",
            color: "",
            extensions: &[
                ".rbbas",
                ".rbfrm",
                ".rbmnu",
                ".rbres",
                ".rbtbar",
                ".rbuistate",
            ],
            tm_scope: "source.vbnet",
            ace_mode: "text",
            language_id: 310,
        }
    }
}
pub struct REXX;
impl REXX {
    pub fn info() -> Language {
        Language {
            name: "REXX",
            type_: "programming",
            color: "#d90e09",
            extensions: &[".rexx", ".pprx", ".rex"],
            tm_scope: "source.rexx",
            ace_mode: "text",
            language_id: 311,
        }
    }
}
pub struct RMarkdown;
impl RMarkdown {
    pub fn info() -> Language {
        Language {
            name: "RMarkdown",
            type_: "prose",
            color: "#198ce7",
            extensions: &[".qmd", ".rmd"],
            tm_scope: "text.md",
            ace_mode: "markdown",
            language_id: 313,
        }
    }
}
pub struct RON;
impl RON {
    pub fn info() -> Language {
        Language {
            name: "RON",
            type_: "data",
            color: "#a62c00",
            extensions: &[".ron"],
            tm_scope: "source.ron",
            ace_mode: "rust",
            language_id: 587855233,
        }
    }
}
pub struct RPC;
impl RPC {
    pub fn info() -> Language {
        Language {
            name: "RPC",
            type_: "programming",
            color: "",
            extensions: &[".x"],
            tm_scope: "source.c",
            ace_mode: "c_cpp",
            language_id: 1031374237,
        }
    }
}
pub struct RPGLE;
impl RPGLE {
    pub fn info() -> Language {
        Language {
            name: "RPGLE",
            type_: "programming",
            color: "#2BDE21",
            extensions: &[".rpgle", ".sqlrpgle"],
            tm_scope: "source.rpgle",
            ace_mode: "text",
            language_id: 609977990,
        }
    }
}
pub struct RPMSpec;
impl RPMSpec {
    pub fn info() -> Language {
        Language {
            name: "RPM Spec",
            type_: "data",
            color: "",
            extensions: &[".spec"],
            tm_scope: "source.rpm-spec",
            ace_mode: "text",
            language_id: 314,
        }
    }
}
pub struct RUNOFF;
impl RUNOFF {
    pub fn info() -> Language {
        Language {
            name: "RUNOFF",
            type_: "markup",
            color: "#665a4e",
            extensions: &[".rnh", ".rno"],
            tm_scope: "text.runoff",
            ace_mode: "text",
            language_id: 315,
        }
    }
}
pub struct Racket;
impl Racket {
    pub fn info() -> Language {
        Language {
            name: "Racket",
            type_: "programming",
            color: "#3c5caa",
            extensions: &[".rkt", ".rktd", ".rktl", ".scrbl"],
            tm_scope: "source.racket",
            ace_mode: "lisp",
            language_id: 316,
        }
    }
}
pub struct Ragel;
impl Ragel {
    pub fn info() -> Language {
        Language {
            name: "Ragel",
            type_: "programming",
            color: "#9d5200",
            extensions: &[".rl"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 317,
        }
    }
}
pub struct Raku;
impl Raku {
    pub fn info() -> Language {
        Language {
            name: "Raku",
            type_: "programming",
            color: "#0000fb",
            extensions: &[
                ".6pl",
                ".6pm",
                ".nqp",
                ".p6",
                ".p6l",
                ".p6m",
                ".pl",
                ".pl6",
                ".pm",
                ".pm6",
                ".raku",
                ".rakumod",
                ".t",
            ],
            tm_scope: "source.raku",
            ace_mode: "perl",
            language_id: 283,
        }
    }
}
pub struct Rascal;
impl Rascal {
    pub fn info() -> Language {
        Language {
            name: "Rascal",
            type_: "programming",
            color: "#fffaa0",
            extensions: &[".rsc"],
            tm_scope: "source.rascal",
            ace_mode: "text",
            language_id: 173616037,
        }
    }
}
pub struct RawTokenData;
impl RawTokenData {
    pub fn info() -> Language {
        Language {
            name: "Raw token data",
            type_: "data",
            color: "",
            extensions: &[".raw"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 318,
        }
    }
}
pub struct ReScript;
impl ReScript {
    pub fn info() -> Language {
        Language {
            name: "ReScript",
            type_: "programming",
            color: "#ed5051",
            extensions: &[".res"],
            tm_scope: "source.rescript",
            ace_mode: "rust",
            language_id: 501875647,
        }
    }
}
pub struct ReadlineConfig;
impl ReadlineConfig {
    pub fn info() -> Language {
        Language {
            name: "Readline Config",
            type_: "data",
            color: "",
            extensions: &[],
            tm_scope: "source.inputrc",
            ace_mode: "text",
            language_id: 538732839,
        }
    }
}
pub struct Reason;
impl Reason {
    pub fn info() -> Language {
        Language {
            name: "Reason",
            type_: "programming",
            color: "#ff5847",
            extensions: &[".re", ".rei"],
            tm_scope: "source.reason",
            ace_mode: "rust",
            language_id: 869538413,
        }
    }
}
pub struct ReasonLIGO;
impl ReasonLIGO {
    pub fn info() -> Language {
        Language {
            name: "ReasonLIGO",
            type_: "programming",
            color: "#ff5847",
            extensions: &[".religo"],
            tm_scope: "source.religo",
            ace_mode: "rust",
            language_id: 319002153,
        }
    }
}
pub struct Rebol;
impl Rebol {
    pub fn info() -> Language {
        Language {
            name: "Rebol",
            type_: "programming",
            color: "#358a5b",
            extensions: &[".reb", ".r", ".r2", ".r3", ".rebol"],
            tm_scope: "source.rebol",
            ace_mode: "text",
            language_id: 319,
        }
    }
}
pub struct RecordJar;
impl RecordJar {
    pub fn info() -> Language {
        Language {
            name: "Record Jar",
            type_: "data",
            color: "#0673ba",
            extensions: &[],
            tm_scope: "source.record-jar",
            ace_mode: "text",
            language_id: 865765202,
        }
    }
}
pub struct Red;
impl Red {
    pub fn info() -> Language {
        Language {
            name: "Red",
            type_: "programming",
            color: "#f50000",
            extensions: &[".red", ".reds"],
            tm_scope: "source.red",
            ace_mode: "text",
            language_id: 320,
        }
    }
}
pub struct Redcode;
impl Redcode {
    pub fn info() -> Language {
        Language {
            name: "Redcode",
            type_: "programming",
            color: "",
            extensions: &[".cw"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 321,
        }
    }
}
pub struct RedirectRules;
impl RedirectRules {
    pub fn info() -> Language {
        Language {
            name: "Redirect Rules",
            type_: "data",
            color: "",
            extensions: &[],
            tm_scope: "source.redirects",
            ace_mode: "text",
            language_id: 1020148948,
        }
    }
}
pub struct RegularExpression;
impl RegularExpression {
    pub fn info() -> Language {
        Language {
            name: "Regular Expression",
            type_: "data",
            color: "#009a00",
            extensions: &[".regexp", ".regex"],
            tm_scope: "source.regexp",
            ace_mode: "text",
            language_id: 363378884,
        }
    }
}
pub struct RenPy;
impl RenPy {
    pub fn info() -> Language {
        Language {
            name: "Ren'Py",
            type_: "programming",
            color: "#ff7f7f",
            extensions: &[".rpy"],
            tm_scope: "source.renpy",
            ace_mode: "python",
            language_id: 322,
        }
    }
}
pub struct RenderScript;
impl RenderScript {
    pub fn info() -> Language {
        Language {
            name: "RenderScript",
            type_: "programming",
            color: "",
            extensions: &[".rs", ".rsh"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 323,
        }
    }
}
pub struct Rez;
impl Rez {
    pub fn info() -> Language {
        Language {
            name: "Rez",
            type_: "programming",
            color: "#FFDAB3",
            extensions: &[".r"],
            tm_scope: "source.rez",
            ace_mode: "text",
            language_id: 498022874,
        }
    }
}
pub struct RichTextFormat;
impl RichTextFormat {
    pub fn info() -> Language {
        Language {
            name: "Rich Text Format",
            type_: "markup",
            color: "",
            extensions: &[".rtf"],
            tm_scope: "text.rtf",
            ace_mode: "text",
            language_id: 51601661,
        }
    }
}
pub struct Ring;
impl Ring {
    pub fn info() -> Language {
        Language {
            name: "Ring",
            type_: "programming",
            color: "#2D54CB",
            extensions: &[".ring"],
            tm_scope: "source.ring",
            ace_mode: "text",
            language_id: 431,
        }
    }
}
pub struct Riot;
impl Riot {
    pub fn info() -> Language {
        Language {
            name: "Riot",
            type_: "markup",
            color: "#A71E49",
            extensions: &[".riot"],
            tm_scope: "text.html.riot",
            ace_mode: "html",
            language_id: 878396783,
        }
    }
}
pub struct RobotFramework;
impl RobotFramework {
    pub fn info() -> Language {
        Language {
            name: "RobotFramework",
            type_: "programming",
            color: "#00c0b5",
            extensions: &[".robot", ".resource"],
            tm_scope: "text.robot",
            ace_mode: "text",
            language_id: 324,
        }
    }
}
pub struct Roc;
impl Roc {
    pub fn info() -> Language {
        Language {
            name: "Roc",
            type_: "programming",
            color: "#7c38f5",
            extensions: &[".roc"],
            tm_scope: "source.roc",
            ace_mode: "text",
            language_id: 440182480,
        }
    }
}
pub struct Roff;
impl Roff {
    pub fn info() -> Language {
        Language {
            name: "Roff",
            type_: "markup",
            color: "#ecdebe",
            extensions: &[
                ".roff",
                ".1",
                ".1in",
                ".1m",
                ".1x",
                ".2",
                ".3",
                ".3in",
                ".3m",
                ".3p",
                ".3pm",
                ".3qt",
                ".3x",
                ".4",
                ".5",
                ".6",
                ".7",
                ".8",
                ".9",
                ".l",
                ".man",
                ".mdoc",
                ".me",
                ".ms",
                ".n",
                ".nr",
                ".rno",
                ".tmac",
            ],
            tm_scope: "text.roff",
            ace_mode: "text",
            language_id: 141,
        }
    }
}
pub struct RoffManpage;
impl RoffManpage {
    pub fn info() -> Language {
        Language {
            name: "Roff Manpage",
            type_: "markup",
            color: "#ecdebe",
            extensions: &[
                ".1",
                ".1in",
                ".1m",
                ".1x",
                ".2",
                ".3",
                ".3in",
                ".3m",
                ".3p",
                ".3pm",
                ".3qt",
                ".3x",
                ".4",
                ".5",
                ".6",
                ".7",
                ".8",
                ".9",
                ".man",
                ".mdoc",
            ],
            tm_scope: "text.roff",
            ace_mode: "text",
            language_id: 612669833,
        }
    }
}
pub struct Rouge;
impl Rouge {
    pub fn info() -> Language {
        Language {
            name: "Rouge",
            type_: "programming",
            color: "#cc0088",
            extensions: &[".rg"],
            tm_scope: "source.clojure",
            ace_mode: "clojure",
            language_id: 325,
        }
    }
}
pub struct RouterOSScript;
impl RouterOSScript {
    pub fn info() -> Language {
        Language {
            name: "RouterOS Script",
            type_: "programming",
            color: "#DE3941",
            extensions: &[".rsc"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 592853203,
        }
    }
}
pub struct Ruby;
impl Ruby {
    pub fn info() -> Language {
        Language {
            name: "Ruby",
            type_: "programming",
            color: "#701516",
            extensions: &[
                ".rb",
                ".builder",
                ".eye",
                ".fcgi",
                ".gemspec",
                ".god",
                ".jbuilder",
                ".mspec",
                ".pluginspec",
                ".podspec",
                ".prawn",
                ".rabl",
                ".rake",
                ".rbi",
                ".rbuild",
                ".rbw",
                ".rbx",
                ".ru",
                ".ruby",
                ".spec",
                ".thor",
                ".watchr",
            ],
            tm_scope: "source.ruby",
            ace_mode: "ruby",
            language_id: 326,
        }
    }
}
pub struct Rust;
impl Rust {
    pub fn info() -> Language {
        Language {
            name: "Rust",
            type_: "programming",
            color: "#dea584",
            extensions: &[".rs", ".rs.in"],
            tm_scope: "source.rust",
            ace_mode: "rust",
            language_id: 327,
        }
    }
}
pub struct SAS;
impl SAS {
    pub fn info() -> Language {
        Language {
            name: "SAS",
            type_: "programming",
            color: "#B34936",
            extensions: &[".sas"],
            tm_scope: "source.sas",
            ace_mode: "text",
            language_id: 328,
        }
    }
}
pub struct SCSS;
impl SCSS {
    pub fn info() -> Language {
        Language {
            name: "SCSS",
            type_: "markup",
            color: "#c6538c",
            extensions: &[".scss"],
            tm_scope: "source.css.scss",
            ace_mode: "scss",
            language_id: 329,
        }
    }
}
pub struct SELinuxPolicy;
impl SELinuxPolicy {
    pub fn info() -> Language {
        Language {
            name: "SELinux Policy",
            type_: "data",
            color: "",
            extensions: &[".te"],
            tm_scope: "source.sepolicy",
            ace_mode: "text",
            language_id: 880010326,
        }
    }
}
pub struct SMT;
impl SMT {
    pub fn info() -> Language {
        Language {
            name: "SMT",
            type_: "programming",
            color: "",
            extensions: &[".smt2", ".smt"],
            tm_scope: "source.smt",
            ace_mode: "text",
            language_id: 330,
        }
    }
}
pub struct SPARQL;
impl SPARQL {
    pub fn info() -> Language {
        Language {
            name: "SPARQL",
            type_: "data",
            color: "#0C4597",
            extensions: &[".sparql", ".rq"],
            tm_scope: "source.sparql",
            ace_mode: "text",
            language_id: 331,
        }
    }
}
pub struct SQF;
impl SQF {
    pub fn info() -> Language {
        Language {
            name: "SQF",
            type_: "programming",
            color: "#3F3F3F",
            extensions: &[".sqf", ".hqf"],
            tm_scope: "source.sqf",
            ace_mode: "text",
            language_id: 332,
        }
    }
}
pub struct SQL;
impl SQL {
    pub fn info() -> Language {
        Language {
            name: "SQL",
            type_: "data",
            color: "#e38c00",
            extensions: &[
                ".sql",
                ".cql",
                ".ddl",
                ".inc",
                ".mysql",
                ".prc",
                ".tab",
                ".udf",
                ".viw",
            ],
            tm_scope: "source.sql",
            ace_mode: "sql",
            language_id: 333,
        }
    }
}
pub struct SQLPL;
impl SQLPL {
    pub fn info() -> Language {
        Language {
            name: "SQLPL",
            type_: "programming",
            color: "#e38c00",
            extensions: &[".sql", ".db2"],
            tm_scope: "source.sql",
            ace_mode: "sql",
            language_id: 334,
        }
    }
}
pub struct SRecodeTemplate;
impl SRecodeTemplate {
    pub fn info() -> Language {
        Language {
            name: "SRecode Template",
            type_: "markup",
            color: "#348a34",
            extensions: &[".srt"],
            tm_scope: "source.lisp",
            ace_mode: "lisp",
            language_id: 335,
        }
    }
}
pub struct SSHConfig;
impl SSHConfig {
    pub fn info() -> Language {
        Language {
            name: "SSH Config",
            type_: "data",
            color: "",
            extensions: &[],
            tm_scope: "source.ssh-config",
            ace_mode: "text",
            language_id: 554920715,
        }
    }
}
pub struct STAR;
impl STAR {
    pub fn info() -> Language {
        Language {
            name: "STAR",
            type_: "data",
            color: "",
            extensions: &[".star"],
            tm_scope: "source.star",
            ace_mode: "text",
            language_id: 424510560,
        }
    }
}
pub struct STL;
impl STL {
    pub fn info() -> Language {
        Language {
            name: "STL",
            type_: "data",
            color: "#373b5e",
            extensions: &[".stl"],
            tm_scope: "source.stl",
            ace_mode: "text",
            language_id: 455361735,
        }
    }
}
pub struct STON;
impl STON {
    pub fn info() -> Language {
        Language {
            name: "STON",
            type_: "data",
            color: "",
            extensions: &[".ston"],
            tm_scope: "source.smalltalk",
            ace_mode: "text",
            language_id: 336,
        }
    }
}
pub struct SVG;
impl SVG {
    pub fn info() -> Language {
        Language {
            name: "SVG",
            type_: "data",
            color: "#ff9900",
            extensions: &[".svg"],
            tm_scope: "text.xml.svg",
            ace_mode: "xml",
            language_id: 337,
        }
    }
}
pub struct SWIG;
impl SWIG {
    pub fn info() -> Language {
        Language {
            name: "SWIG",
            type_: "programming",
            color: "",
            extensions: &[".i"],
            tm_scope: "source.c++",
            ace_mode: "c_cpp",
            language_id: 1066250075,
        }
    }
}
pub struct Sage;
impl Sage {
    pub fn info() -> Language {
        Language {
            name: "Sage",
            type_: "programming",
            color: "",
            extensions: &[".sage", ".sagews"],
            tm_scope: "source.python",
            ace_mode: "python",
            language_id: 338,
        }
    }
}
pub struct SaltStack;
impl SaltStack {
    pub fn info() -> Language {
        Language {
            name: "SaltStack",
            type_: "programming",
            color: "#646464",
            extensions: &[".sls"],
            tm_scope: "source.yaml.salt",
            ace_mode: "yaml",
            language_id: 339,
        }
    }
}
pub struct Sass;
impl Sass {
    pub fn info() -> Language {
        Language {
            name: "Sass",
            type_: "markup",
            color: "#a53b70",
            extensions: &[".sass"],
            tm_scope: "source.sass",
            ace_mode: "sass",
            language_id: 340,
        }
    }
}
pub struct Scala;
impl Scala {
    pub fn info() -> Language {
        Language {
            name: "Scala",
            type_: "programming",
            color: "#c22d40",
            extensions: &[".scala", ".kojo", ".sbt", ".sc"],
            tm_scope: "source.scala",
            ace_mode: "scala",
            language_id: 341,
        }
    }
}
pub struct Scaml;
impl Scaml {
    pub fn info() -> Language {
        Language {
            name: "Scaml",
            type_: "markup",
            color: "#bd181a",
            extensions: &[".scaml"],
            tm_scope: "source.scaml",
            ace_mode: "text",
            language_id: 342,
        }
    }
}
pub struct Scenic;
impl Scenic {
    pub fn info() -> Language {
        Language {
            name: "Scenic",
            type_: "programming",
            color: "#fdc700",
            extensions: &[".scenic"],
            tm_scope: "source.scenic",
            ace_mode: "text",
            language_id: 619814037,
        }
    }
}
pub struct Scheme;
impl Scheme {
    pub fn info() -> Language {
        Language {
            name: "Scheme",
            type_: "programming",
            color: "#1e4aec",
            extensions: &[".scm", ".sch", ".sld", ".sls", ".sps", ".ss"],
            tm_scope: "source.scheme",
            ace_mode: "scheme",
            language_id: 343,
        }
    }
}
pub struct Scilab;
impl Scilab {
    pub fn info() -> Language {
        Language {
            name: "Scilab",
            type_: "programming",
            color: "#ca0f21",
            extensions: &[".sci", ".sce", ".tst"],
            tm_scope: "source.scilab",
            ace_mode: "text",
            language_id: 344,
        }
    }
}
pub struct _Self;
impl _Self {
    pub fn info() -> Language {
        Language {
            name: "Self",
            type_: "programming",
            color: "#0579aa",
            extensions: &[".self"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 345,
        }
    }
}
pub struct ShaderLab;
impl ShaderLab {
    pub fn info() -> Language {
        Language {
            name: "ShaderLab",
            type_: "programming",
            color: "#222c37",
            extensions: &[".shader"],
            tm_scope: "source.shaderlab",
            ace_mode: "text",
            language_id: 664257356,
        }
    }
}
pub struct Shell;
impl Shell {
    pub fn info() -> Language {
        Language {
            name: "Shell",
            type_: "programming",
            color: "#89e051",
            extensions: &[
                ".sh",
                ".bash",
                ".bats",
                ".cgi",
                ".command",
                ".fcgi",
                ".ksh",
                ".sh.in",
                ".tmux",
                ".tool",
                ".trigger",
                ".zsh",
                ".zsh-theme",
            ],
            tm_scope: "source.shell",
            ace_mode: "sh",
            language_id: 346,
        }
    }
}
pub struct ShellCheckConfig;
impl ShellCheckConfig {
    pub fn info() -> Language {
        Language {
            name: "ShellCheck Config",
            type_: "data",
            color: "#cecfcb",
            extensions: &[],
            tm_scope: "source.shellcheckrc",
            ace_mode: "ini",
            language_id: 687511714,
        }
    }
}
pub struct ShellSession;
impl ShellSession {
    pub fn info() -> Language {
        Language {
            name: "ShellSession",
            type_: "programming",
            color: "",
            extensions: &[".sh-session"],
            tm_scope: "text.shell-session",
            ace_mode: "sh",
            language_id: 347,
        }
    }
}
pub struct Shen;
impl Shen {
    pub fn info() -> Language {
        Language {
            name: "Shen",
            type_: "programming",
            color: "#120F14",
            extensions: &[".shen"],
            tm_scope: "source.shen",
            ace_mode: "text",
            language_id: 348,
        }
    }
}
pub struct Sieve;
impl Sieve {
    pub fn info() -> Language {
        Language {
            name: "Sieve",
            type_: "programming",
            color: "",
            extensions: &[".sieve"],
            tm_scope: "source.sieve",
            ace_mode: "text",
            language_id: 208976687,
        }
    }
}
pub struct SimpleFileVerification;
impl SimpleFileVerification {
    pub fn info() -> Language {
        Language {
            name: "Simple File Verification",
            type_: "data",
            color: "#C9BFED",
            extensions: &[".sfv"],
            tm_scope: "source.sfv",
            ace_mode: "ini",
            language_id: 735623761,
        }
    }
}
pub struct Singularity;
impl Singularity {
    pub fn info() -> Language {
        Language {
            name: "Singularity",
            type_: "programming",
            color: "#64E6AD",
            extensions: &[],
            tm_scope: "source.singularity",
            ace_mode: "text",
            language_id: 987024632,
        }
    }
}
pub struct Slash;
impl Slash {
    pub fn info() -> Language {
        Language {
            name: "Slash",
            type_: "programming",
            color: "#007eff",
            extensions: &[".sl"],
            tm_scope: "text.html.slash",
            ace_mode: "text",
            language_id: 349,
        }
    }
}
pub struct Slice;
impl Slice {
    pub fn info() -> Language {
        Language {
            name: "Slice",
            type_: "programming",
            color: "#003fa2",
            extensions: &[".ice"],
            tm_scope: "source.ice",
            ace_mode: "text",
            language_id: 894641667,
        }
    }
}
pub struct Slim;
impl Slim {
    pub fn info() -> Language {
        Language {
            name: "Slim",
            type_: "markup",
            color: "#2b2b2b",
            extensions: &[".slim"],
            tm_scope: "text.slim",
            ace_mode: "text",
            language_id: 350,
        }
    }
}
pub struct Slint;
impl Slint {
    pub fn info() -> Language {
        Language {
            name: "Slint",
            type_: "markup",
            color: "#2379F4",
            extensions: &[".slint"],
            tm_scope: "source.slint",
            ace_mode: "text",
            language_id: 119900149,
        }
    }
}
pub struct SmPL;
impl SmPL {
    pub fn info() -> Language {
        Language {
            name: "SmPL",
            type_: "programming",
            color: "#c94949",
            extensions: &[".cocci"],
            tm_scope: "source.smpl",
            ace_mode: "text",
            language_id: 164123055,
        }
    }
}
pub struct Smali;
impl Smali {
    pub fn info() -> Language {
        Language {
            name: "Smali",
            type_: "programming",
            color: "",
            extensions: &[".smali"],
            tm_scope: "source.smali",
            ace_mode: "text",
            language_id: 351,
        }
    }
}
pub struct Smalltalk;
impl Smalltalk {
    pub fn info() -> Language {
        Language {
            name: "Smalltalk",
            type_: "programming",
            color: "#596706",
            extensions: &[".st", ".cs"],
            tm_scope: "source.smalltalk",
            ace_mode: "text",
            language_id: 352,
        }
    }
}
pub struct Smarty;
impl Smarty {
    pub fn info() -> Language {
        Language {
            name: "Smarty",
            type_: "programming",
            color: "#f0c040",
            extensions: &[".tpl"],
            tm_scope: "text.html.smarty",
            ace_mode: "smarty",
            language_id: 353,
        }
    }
}
pub struct Smithy;
impl Smithy {
    pub fn info() -> Language {
        Language {
            name: "Smithy",
            type_: "programming",
            color: "#c44536",
            extensions: &[".smithy"],
            tm_scope: "source.smithy",
            ace_mode: "text",
            language_id: 1027892786,
        }
    }
}
pub struct Snakemake;
impl Snakemake {
    pub fn info() -> Language {
        Language {
            name: "Snakemake",
            type_: "programming",
            color: "#419179",
            extensions: &[".smk", ".snakefile"],
            tm_scope: "source.python",
            ace_mode: "python",
            language_id: 151241392,
        }
    }
}
pub struct Solidity;
impl Solidity {
    pub fn info() -> Language {
        Language {
            name: "Solidity",
            type_: "programming",
            color: "#AA6746",
            extensions: &[".sol"],
            tm_scope: "source.solidity",
            ace_mode: "text",
            language_id: 237469032,
        }
    }
}
pub struct Soong;
impl Soong {
    pub fn info() -> Language {
        Language {
            name: "Soong",
            type_: "data",
            color: "",
            extensions: &[],
            tm_scope: "source.bp",
            ace_mode: "text",
            language_id: 222900098,
        }
    }
}
pub struct SourcePawn;
impl SourcePawn {
    pub fn info() -> Language {
        Language {
            name: "SourcePawn",
            type_: "programming",
            color: "#f69e1d",
            extensions: &[".sp", ".inc"],
            tm_scope: "source.sourcepawn",
            ace_mode: "text",
            language_id: 354,
        }
    }
}
pub struct SplineFontDatabase;
impl SplineFontDatabase {
    pub fn info() -> Language {
        Language {
            name: "Spline Font Database",
            type_: "data",
            color: "",
            extensions: &[".sfd"],
            tm_scope: "text.sfd",
            ace_mode: "yaml",
            language_id: 767169629,
        }
    }
}
pub struct Squirrel;
impl Squirrel {
    pub fn info() -> Language {
        Language {
            name: "Squirrel",
            type_: "programming",
            color: "#800000",
            extensions: &[".nut"],
            tm_scope: "source.nut",
            ace_mode: "c_cpp",
            language_id: 355,
        }
    }
}
pub struct Stan;
impl Stan {
    pub fn info() -> Language {
        Language {
            name: "Stan",
            type_: "programming",
            color: "#b2011d",
            extensions: &[".stan"],
            tm_scope: "source.stan",
            ace_mode: "text",
            language_id: 356,
        }
    }
}
pub struct StandardML;
impl StandardML {
    pub fn info() -> Language {
        Language {
            name: "Standard ML",
            type_: "programming",
            color: "#dc566d",
            extensions: &[".ml", ".fun", ".sig", ".sml"],
            tm_scope: "source.ml",
            ace_mode: "text",
            language_id: 357,
        }
    }
}
pub struct Starlark;
impl Starlark {
    pub fn info() -> Language {
        Language {
            name: "Starlark",
            type_: "programming",
            color: "#76d275",
            extensions: &[".bzl", ".star"],
            tm_scope: "source.python",
            ace_mode: "python",
            language_id: 960266174,
        }
    }
}
pub struct Stata;
impl Stata {
    pub fn info() -> Language {
        Language {
            name: "Stata",
            type_: "programming",
            color: "#1a5f91",
            extensions: &[".do", ".ado", ".doh", ".ihlp", ".mata", ".matah", ".sthlp"],
            tm_scope: "source.stata",
            ace_mode: "text",
            language_id: 358,
        }
    }
}
pub struct StringTemplate;
impl StringTemplate {
    pub fn info() -> Language {
        Language {
            name: "StringTemplate",
            type_: "markup",
            color: "#3fb34f",
            extensions: &[".st"],
            tm_scope: "source.string-template",
            ace_mode: "html",
            language_id: 89855901,
        }
    }
}
pub struct Stylus;
impl Stylus {
    pub fn info() -> Language {
        Language {
            name: "Stylus",
            type_: "markup",
            color: "#ff6347",
            extensions: &[".styl"],
            tm_scope: "source.stylus",
            ace_mode: "stylus",
            language_id: 359,
        }
    }
}
pub struct SubRipText;
impl SubRipText {
    pub fn info() -> Language {
        Language {
            name: "SubRip Text",
            type_: "data",
            color: "#9e0101",
            extensions: &[".srt"],
            tm_scope: "text.srt",
            ace_mode: "text",
            language_id: 360,
        }
    }
}
pub struct SugarSS;
impl SugarSS {
    pub fn info() -> Language {
        Language {
            name: "SugarSS",
            type_: "markup",
            color: "#2fcc9f",
            extensions: &[".sss"],
            tm_scope: "source.css.postcss.sugarss",
            ace_mode: "text",
            language_id: 826404698,
        }
    }
}
pub struct SuperCollider;
impl SuperCollider {
    pub fn info() -> Language {
        Language {
            name: "SuperCollider",
            type_: "programming",
            color: "#46390b",
            extensions: &[".sc", ".scd"],
            tm_scope: "source.supercollider",
            ace_mode: "text",
            language_id: 361,
        }
    }
}
pub struct Svelte;
impl Svelte {
    pub fn info() -> Language {
        Language {
            name: "Svelte",
            type_: "markup",
            color: "#ff3e00",
            extensions: &[".svelte"],
            tm_scope: "source.svelte",
            ace_mode: "html",
            language_id: 928734530,
        }
    }
}
pub struct Sway;
impl Sway {
    pub fn info() -> Language {
        Language {
            name: "Sway",
            type_: "programming",
            color: "#00F58C",
            extensions: &[".sw"],
            tm_scope: "source.sway",
            ace_mode: "rust",
            language_id: 271471144,
        }
    }
}
pub struct Sweave;
impl Sweave {
    pub fn info() -> Language {
        Language {
            name: "Sweave",
            type_: "prose",
            color: "#198ce7",
            extensions: &[".rnw"],
            tm_scope: "text.tex.latex.sweave",
            ace_mode: "tex",
            language_id: 558779190,
        }
    }
}
pub struct Swift;
impl Swift {
    pub fn info() -> Language {
        Language {
            name: "Swift",
            type_: "programming",
            color: "#F05138",
            extensions: &[".swift"],
            tm_scope: "source.swift",
            ace_mode: "text",
            language_id: 362,
        }
    }
}
pub struct SystemVerilog;
impl SystemVerilog {
    pub fn info() -> Language {
        Language {
            name: "SystemVerilog",
            type_: "programming",
            color: "#DAE1C2",
            extensions: &[".sv", ".svh", ".vh"],
            tm_scope: "source.systemverilog",
            ace_mode: "verilog",
            language_id: 363,
        }
    }
}
pub struct TIProgram;
impl TIProgram {
    pub fn info() -> Language {
        Language {
            name: "TI Program",
            type_: "programming",
            color: "#A0AA87",
            extensions: &[".8xp", ".8xp.txt"],
            tm_scope: "source.8xp",
            ace_mode: "text",
            language_id: 422,
        }
    }
}
pub struct TLVerilog;
impl TLVerilog {
    pub fn info() -> Language {
        Language {
            name: "TL-Verilog",
            type_: "programming",
            color: "#C40023",
            extensions: &[".tlv"],
            tm_scope: "source.tlverilog",
            ace_mode: "verilog",
            language_id: 118656070,
        }
    }
}
pub struct TLA;
impl TLA {
    pub fn info() -> Language {
        Language {
            name: "TLA",
            type_: "programming",
            color: "#4b0079",
            extensions: &[".tla"],
            tm_scope: "source.tla",
            ace_mode: "text",
            language_id: 364,
        }
    }
}
pub struct TOML;
impl TOML {
    pub fn info() -> Language {
        Language {
            name: "TOML",
            type_: "data",
            color: "#9c4221",
            extensions: &[".toml"],
            tm_scope: "source.toml",
            ace_mode: "toml",
            language_id: 365,
        }
    }
}
pub struct TSQL;
impl TSQL {
    pub fn info() -> Language {
        Language {
            name: "TSQL",
            type_: "programming",
            color: "#e38c00",
            extensions: &[".sql"],
            tm_scope: "source.tsql",
            ace_mode: "sql",
            language_id: 918334941,
        }
    }
}
pub struct TSV;
impl TSV {
    pub fn info() -> Language {
        Language {
            name: "TSV",
            type_: "data",
            color: "#237346",
            extensions: &[".tsv", ".vcf"],
            tm_scope: "source.generic-db",
            ace_mode: "text",
            language_id: 1035892117,
        }
    }
}
pub struct TSX;
impl TSX {
    pub fn info() -> Language {
        Language {
            name: "TSX",
            type_: "programming",
            color: "#3178c6",
            extensions: &[".tsx"],
            tm_scope: "source.tsx",
            ace_mode: "javascript",
            language_id: 94901924,
        }
    }
}
pub struct TXL;
impl TXL {
    pub fn info() -> Language {
        Language {
            name: "TXL",
            type_: "programming",
            color: "#0178b8",
            extensions: &[".txl"],
            tm_scope: "source.txl",
            ace_mode: "text",
            language_id: 366,
        }
    }
}
pub struct Talon;
impl Talon {
    pub fn info() -> Language {
        Language {
            name: "Talon",
            type_: "programming",
            color: "#333333",
            extensions: &[".talon"],
            tm_scope: "source.talon",
            ace_mode: "text",
            language_id: 959889508,
        }
    }
}
pub struct Tcl;
impl Tcl {
    pub fn info() -> Language {
        Language {
            name: "Tcl",
            type_: "programming",
            color: "#e4cc98",
            extensions: &[".tcl", ".adp", ".sdc", ".tcl.in", ".tm", ".xdc"],
            tm_scope: "source.tcl",
            ace_mode: "tcl",
            language_id: 367,
        }
    }
}
pub struct Tcsh;
impl Tcsh {
    pub fn info() -> Language {
        Language {
            name: "Tcsh",
            type_: "programming",
            color: "",
            extensions: &[".tcsh", ".csh"],
            tm_scope: "source.shell",
            ace_mode: "sh",
            language_id: 368,
        }
    }
}
pub struct TeX;
impl TeX {
    pub fn info() -> Language {
        Language {
            name: "TeX",
            type_: "markup",
            color: "#3D6117",
            extensions: &[
                ".tex",
                ".aux",
                ".bbx",
                ".cbx",
                ".cls",
                ".dtx",
                ".ins",
                ".lbx",
                ".ltx",
                ".mkii",
                ".mkiv",
                ".mkvi",
                ".sty",
                ".toc",
            ],
            tm_scope: "text.tex.latex",
            ace_mode: "tex",
            language_id: 369,
        }
    }
}
pub struct Tea;
impl Tea {
    pub fn info() -> Language {
        Language {
            name: "Tea",
            type_: "markup",
            color: "",
            extensions: &[".tea"],
            tm_scope: "source.tea",
            ace_mode: "text",
            language_id: 370,
        }
    }
}
pub struct Terra;
impl Terra {
    pub fn info() -> Language {
        Language {
            name: "Terra",
            type_: "programming",
            color: "#00004c",
            extensions: &[".t"],
            tm_scope: "source.terra",
            ace_mode: "lua",
            language_id: 371,
        }
    }
}
pub struct TerraformTemplate;
impl TerraformTemplate {
    pub fn info() -> Language {
        Language {
            name: "Terraform Template",
            type_: "markup",
            color: "#7b42bb",
            extensions: &[".tftpl"],
            tm_scope: "source.hcl.terraform",
            ace_mode: "ruby",
            language_id: 856832701,
        }
    }
}
pub struct Texinfo;
impl Texinfo {
    pub fn info() -> Language {
        Language {
            name: "Texinfo",
            type_: "prose",
            color: "",
            extensions: &[".texinfo", ".texi", ".txi"],
            tm_scope: "text.texinfo",
            ace_mode: "text",
            language_id: 988020015,
        }
    }
}
pub struct Text;
impl Text {
    pub fn info() -> Language {
        Language {
            name: "Text",
            type_: "prose",
            color: "",
            extensions: &[".txt", ".fr", ".nb", ".ncl", ".no"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 372,
        }
    }
}
pub struct TextGrid;
impl TextGrid {
    pub fn info() -> Language {
        Language {
            name: "TextGrid",
            type_: "data",
            color: "#c8506d",
            extensions: &[".TextGrid"],
            tm_scope: "source.textgrid",
            ace_mode: "text",
            language_id: 965696054,
        }
    }
}
pub struct TextMateProperties;
impl TextMateProperties {
    pub fn info() -> Language {
        Language {
            name: "TextMate Properties",
            type_: "data",
            color: "#df66e4",
            extensions: &[],
            tm_scope: "source.tm-properties",
            ace_mode: "properties",
            language_id: 981795023,
        }
    }
}
pub struct Textile;
impl Textile {
    pub fn info() -> Language {
        Language {
            name: "Textile",
            type_: "prose",
            color: "#ffe7ac",
            extensions: &[".textile"],
            tm_scope: "none",
            ace_mode: "textile",
            language_id: 373,
        }
    }
}
pub struct Thrift;
impl Thrift {
    pub fn info() -> Language {
        Language {
            name: "Thrift",
            type_: "programming",
            color: "#D12127",
            extensions: &[".thrift"],
            tm_scope: "source.thrift",
            ace_mode: "text",
            language_id: 374,
        }
    }
}
pub struct Toit;
impl Toit {
    pub fn info() -> Language {
        Language {
            name: "Toit",
            type_: "programming",
            color: "#c2c9fb",
            extensions: &[".toit"],
            tm_scope: "source.toit",
            ace_mode: "text",
            language_id: 356554395,
        }
    }
}
pub struct Turing;
impl Turing {
    pub fn info() -> Language {
        Language {
            name: "Turing",
            type_: "programming",
            color: "#cf142b",
            extensions: &[".t", ".tu"],
            tm_scope: "source.turing",
            ace_mode: "text",
            language_id: 375,
        }
    }
}
pub struct Turtle;
impl Turtle {
    pub fn info() -> Language {
        Language {
            name: "Turtle",
            type_: "data",
            color: "",
            extensions: &[".ttl"],
            tm_scope: "source.turtle",
            ace_mode: "text",
            language_id: 376,
        }
    }
}
pub struct Twig;
impl Twig {
    pub fn info() -> Language {
        Language {
            name: "Twig",
            type_: "markup",
            color: "#c1d026",
            extensions: &[".twig"],
            tm_scope: "text.html.twig",
            ace_mode: "twig",
            language_id: 377,
        }
    }
}
pub struct TypeLanguage;
impl TypeLanguage {
    pub fn info() -> Language {
        Language {
            name: "Type Language",
            type_: "data",
            color: "",
            extensions: &[".tl"],
            tm_scope: "source.tl",
            ace_mode: "text",
            language_id: 632765617,
        }
    }
}
pub struct TypeScript;
impl TypeScript {
    pub fn info() -> Language {
        Language {
            name: "TypeScript",
            type_: "programming",
            color: "#3178c6",
            extensions: &[".ts", ".cts", ".mts"],
            tm_scope: "source.ts",
            ace_mode: "typescript",
            language_id: 378,
        }
    }
}
pub struct Typst;
impl Typst {
    pub fn info() -> Language {
        Language {
            name: "Typst",
            type_: "programming",
            color: "#239dad",
            extensions: &[".typ"],
            tm_scope: "source.typst",
            ace_mode: "text",
            language_id: 704730682,
        }
    }
}
pub struct UnifiedParallelC;
impl UnifiedParallelC {
    pub fn info() -> Language {
        Language {
            name: "Unified Parallel C",
            type_: "programming",
            color: "#4e3617",
            extensions: &[".upc"],
            tm_scope: "source.c",
            ace_mode: "c_cpp",
            language_id: 379,
        }
    }
}
pub struct Unity3DAsset;
impl Unity3DAsset {
    pub fn info() -> Language {
        Language {
            name: "Unity3D Asset",
            type_: "data",
            color: "#222c37",
            extensions: &[
                ".anim",
                ".asset",
                ".mask",
                ".mat",
                ".meta",
                ".prefab",
                ".unity",
            ],
            tm_scope: "source.yaml",
            ace_mode: "yaml",
            language_id: 380,
        }
    }
}
pub struct UnixAssembly;
impl UnixAssembly {
    pub fn info() -> Language {
        Language {
            name: "Unix Assembly",
            type_: "programming",
            color: "",
            extensions: &[".s", ".ms"],
            tm_scope: "source.x86",
            ace_mode: "assembly_x86",
            language_id: 120,
        }
    }
}
pub struct Uno;
impl Uno {
    pub fn info() -> Language {
        Language {
            name: "Uno",
            type_: "programming",
            color: "#9933cc",
            extensions: &[".uno"],
            tm_scope: "source.cs",
            ace_mode: "csharp",
            language_id: 381,
        }
    }
}
pub struct UnrealScript;
impl UnrealScript {
    pub fn info() -> Language {
        Language {
            name: "UnrealScript",
            type_: "programming",
            color: "#a54c4d",
            extensions: &[".uc"],
            tm_scope: "source.java",
            ace_mode: "java",
            language_id: 382,
        }
    }
}
pub struct UrWeb;
impl UrWeb {
    pub fn info() -> Language {
        Language {
            name: "UrWeb",
            type_: "programming",
            color: "#ccccee",
            extensions: &[".ur", ".urs"],
            tm_scope: "source.ur",
            ace_mode: "text",
            language_id: 383,
        }
    }
}
pub struct V;
impl V {
    pub fn info() -> Language {
        Language {
            name: "V",
            type_: "programming",
            color: "#4f87c4",
            extensions: &[".v"],
            tm_scope: "source.v",
            ace_mode: "golang",
            language_id: 603371597,
        }
    }
}
pub struct VBA;
impl VBA {
    pub fn info() -> Language {
        Language {
            name: "VBA",
            type_: "programming",
            color: "#867db1",
            extensions: &[".bas", ".cls", ".frm", ".vba"],
            tm_scope: "source.vba",
            ace_mode: "text",
            language_id: 399230729,
        }
    }
}
pub struct VBScript;
impl VBScript {
    pub fn info() -> Language {
        Language {
            name: "VBScript",
            type_: "programming",
            color: "#15dcdc",
            extensions: &[".vbs"],
            tm_scope: "source.vbnet",
            ace_mode: "text",
            language_id: 408016005,
        }
    }
}
pub struct VCL;
impl VCL {
    pub fn info() -> Language {
        Language {
            name: "VCL",
            type_: "programming",
            color: "#148AA8",
            extensions: &[".vcl"],
            tm_scope: "source.varnish.vcl",
            ace_mode: "text",
            language_id: 384,
        }
    }
}
pub struct VHDL;
impl VHDL {
    pub fn info() -> Language {
        Language {
            name: "VHDL",
            type_: "programming",
            color: "#adb2cb",
            extensions: &[
                ".vhdl",
                ".vhd",
                ".vhf",
                ".vhi",
                ".vho",
                ".vhs",
                ".vht",
                ".vhw",
            ],
            tm_scope: "source.vhdl",
            ace_mode: "vhdl",
            language_id: 385,
        }
    }
}
pub struct Vala;
impl Vala {
    pub fn info() -> Language {
        Language {
            name: "Vala",
            type_: "programming",
            color: "#a56de2",
            extensions: &[".vala", ".vapi"],
            tm_scope: "source.vala",
            ace_mode: "vala",
            language_id: 386,
        }
    }
}
pub struct ValveDataFormat;
impl ValveDataFormat {
    pub fn info() -> Language {
        Language {
            name: "Valve Data Format",
            type_: "data",
            color: "#f26025",
            extensions: &[".vdf"],
            tm_scope: "source.keyvalues",
            ace_mode: "text",
            language_id: 544060961,
        }
    }
}
pub struct VelocityTemplateLanguage;
impl VelocityTemplateLanguage {
    pub fn info() -> Language {
        Language {
            name: "Velocity Template Language",
            type_: "markup",
            color: "#507cff",
            extensions: &[".vtl"],
            tm_scope: "source.velocity",
            ace_mode: "velocity",
            language_id: 292377326,
        }
    }
}
pub struct Verilog;
impl Verilog {
    pub fn info() -> Language {
        Language {
            name: "Verilog",
            type_: "programming",
            color: "#b2b7f8",
            extensions: &[".v", ".veo"],
            tm_scope: "source.verilog",
            ace_mode: "verilog",
            language_id: 387,
        }
    }
}
pub struct VimHelpFile;
impl VimHelpFile {
    pub fn info() -> Language {
        Language {
            name: "Vim Help File",
            type_: "prose",
            color: "#199f4b",
            extensions: &[".txt"],
            tm_scope: "text.vim-help",
            ace_mode: "text",
            language_id: 508563686,
        }
    }
}
pub struct VimScript;
impl VimScript {
    pub fn info() -> Language {
        Language {
            name: "Vim Script",
            type_: "programming",
            color: "#199f4b",
            extensions: &[".vim", ".vba", ".vimrc", ".vmb"],
            tm_scope: "source.viml",
            ace_mode: "text",
            language_id: 388,
        }
    }
}
pub struct VimSnippet;
impl VimSnippet {
    pub fn info() -> Language {
        Language {
            name: "Vim Snippet",
            type_: "markup",
            color: "#199f4b",
            extensions: &[".snip", ".snippet", ".snippets"],
            tm_scope: "source.vim-snippet",
            ace_mode: "text",
            language_id: 81265970,
        }
    }
}
pub struct VisualBasicNET;
impl VisualBasicNET {
    pub fn info() -> Language {
        Language {
            name: "Visual Basic .NET",
            type_: "programming",
            color: "#945db7",
            extensions: &[".vb", ".vbhtml"],
            tm_scope: "source.vbnet",
            ace_mode: "text",
            language_id: 389,
        }
    }
}
pub struct VisualBasic60;
impl VisualBasic60 {
    pub fn info() -> Language {
        Language {
            name: "Visual Basic 6.0",
            type_: "programming",
            color: "#2c6353",
            extensions: &[".bas", ".cls", ".ctl", ".Dsr", ".frm"],
            tm_scope: "source.vba",
            ace_mode: "text",
            language_id: 679594952,
        }
    }
}
pub struct Volt;
impl Volt {
    pub fn info() -> Language {
        Language {
            name: "Volt",
            type_: "programming",
            color: "#1F1F1F",
            extensions: &[".volt"],
            tm_scope: "source.d",
            ace_mode: "d",
            language_id: 390,
        }
    }
}
pub struct Vue;
impl Vue {
    pub fn info() -> Language {
        Language {
            name: "Vue",
            type_: "markup",
            color: "#41b883",
            extensions: &[".vue"],
            tm_scope: "text.html.vue",
            ace_mode: "html",
            language_id: 391,
        }
    }
}
pub struct Vyper;
impl Vyper {
    pub fn info() -> Language {
        Language {
            name: "Vyper",
            type_: "programming",
            color: "#2980b9",
            extensions: &[".vy"],
            tm_scope: "source.vyper",
            ace_mode: "text",
            language_id: 1055641948,
        }
    }
}
pub struct WDL;
impl WDL {
    pub fn info() -> Language {
        Language {
            name: "WDL",
            type_: "programming",
            color: "#42f1f4",
            extensions: &[".wdl"],
            tm_scope: "source.wdl",
            ace_mode: "text",
            language_id: 374521672,
        }
    }
}
pub struct WGSL;
impl WGSL {
    pub fn info() -> Language {
        Language {
            name: "WGSL",
            type_: "programming",
            color: "#1a5e9a",
            extensions: &[".wgsl"],
            tm_scope: "source.wgsl",
            ace_mode: "text",
            language_id: 836605993,
        }
    }
}
pub struct WavefrontMaterial;
impl WavefrontMaterial {
    pub fn info() -> Language {
        Language {
            name: "Wavefront Material",
            type_: "data",
            color: "",
            extensions: &[".mtl"],
            tm_scope: "source.wavefront.mtl",
            ace_mode: "text",
            language_id: 392,
        }
    }
}
pub struct WavefrontObject;
impl WavefrontObject {
    pub fn info() -> Language {
        Language {
            name: "Wavefront Object",
            type_: "data",
            color: "",
            extensions: &[".obj"],
            tm_scope: "source.wavefront.obj",
            ace_mode: "text",
            language_id: 393,
        }
    }
}
pub struct WebOntologyLanguage;
impl WebOntologyLanguage {
    pub fn info() -> Language {
        Language {
            name: "Web Ontology Language",
            type_: "data",
            color: "#5b70bd",
            extensions: &[".owl"],
            tm_scope: "text.xml",
            ace_mode: "xml",
            language_id: 394,
        }
    }
}
pub struct WebAssembly;
impl WebAssembly {
    pub fn info() -> Language {
        Language {
            name: "WebAssembly",
            type_: "programming",
            color: "#04133b",
            extensions: &[".wast", ".wat"],
            tm_scope: "source.webassembly",
            ace_mode: "lisp",
            language_id: 956556503,
        }
    }
}
pub struct WebAssemblyInterfaceType;
impl WebAssemblyInterfaceType {
    pub fn info() -> Language {
        Language {
            name: "WebAssembly Interface Type",
            type_: "data",
            color: "#6250e7",
            extensions: &[".wit"],
            tm_scope: "source.wit",
            ace_mode: "text",
            language_id: 134534086,
        }
    }
}
pub struct WebIDL;
impl WebIDL {
    pub fn info() -> Language {
        Language {
            name: "WebIDL",
            type_: "programming",
            color: "",
            extensions: &[".webidl"],
            tm_scope: "source.webidl",
            ace_mode: "text",
            language_id: 395,
        }
    }
}
pub struct WebVTT;
impl WebVTT {
    pub fn info() -> Language {
        Language {
            name: "WebVTT",
            type_: "data",
            color: "",
            extensions: &[".vtt"],
            tm_scope: "text.vtt",
            ace_mode: "text",
            language_id: 658679714,
        }
    }
}
pub struct WgetConfig;
impl WgetConfig {
    pub fn info() -> Language {
        Language {
            name: "Wget Config",
            type_: "data",
            color: "",
            extensions: &[],
            tm_scope: "source.wgetrc",
            ace_mode: "text",
            language_id: 668457123,
        }
    }
}
pub struct Whiley;
impl Whiley {
    pub fn info() -> Language {
        Language {
            name: "Whiley",
            type_: "programming",
            color: "#d5c397",
            extensions: &[".whiley"],
            tm_scope: "source.whiley",
            ace_mode: "text",
            language_id: 888779559,
        }
    }
}
pub struct Wikitext;
impl Wikitext {
    pub fn info() -> Language {
        Language {
            name: "Wikitext",
            type_: "prose",
            color: "#fc5757",
            extensions: &[".mediawiki", ".wiki", ".wikitext"],
            tm_scope: "text.html.mediawiki",
            ace_mode: "text",
            language_id: 228,
        }
    }
}
pub struct Win32MessageFile;
impl Win32MessageFile {
    pub fn info() -> Language {
        Language {
            name: "Win32 Message File",
            type_: "data",
            color: "",
            extensions: &[".mc"],
            tm_scope: "source.win32-messages",
            ace_mode: "ini",
            language_id: 950967261,
        }
    }
}
pub struct WindowsRegistryEntries;
impl WindowsRegistryEntries {
    pub fn info() -> Language {
        Language {
            name: "Windows Registry Entries",
            type_: "data",
            color: "#52d5ff",
            extensions: &[".reg"],
            tm_scope: "source.reg",
            ace_mode: "ini",
            language_id: 969674868,
        }
    }
}
pub struct WitcherScript;
impl WitcherScript {
    pub fn info() -> Language {
        Language {
            name: "Witcher Script",
            type_: "programming",
            color: "#ff0000",
            extensions: &[".ws"],
            tm_scope: "source.witcherscript",
            ace_mode: "text",
            language_id: 686821385,
        }
    }
}
pub struct Wollok;
impl Wollok {
    pub fn info() -> Language {
        Language {
            name: "Wollok",
            type_: "programming",
            color: "#a23738",
            extensions: &[".wlk"],
            tm_scope: "source.wollok",
            ace_mode: "text",
            language_id: 632745969,
        }
    }
}
pub struct WorldOfWarcraftAddonData;
impl WorldOfWarcraftAddonData {
    pub fn info() -> Language {
        Language {
            name: "World of Warcraft Addon Data",
            type_: "data",
            color: "#f7e43f",
            extensions: &[".toc"],
            tm_scope: "source.toc",
            ace_mode: "text",
            language_id: 396,
        }
    }
}
pub struct Wren;
impl Wren {
    pub fn info() -> Language {
        Language {
            name: "Wren",
            type_: "programming",
            color: "#383838",
            extensions: &[".wren"],
            tm_scope: "source.wren",
            ace_mode: "text",
            language_id: 713580619,
        }
    }
}
pub struct XBitMap;
impl XBitMap {
    pub fn info() -> Language {
        Language {
            name: "X BitMap",
            type_: "data",
            color: "",
            extensions: &[".xbm"],
            tm_scope: "source.c",
            ace_mode: "c_cpp",
            language_id: 782911107,
        }
    }
}
pub struct XFontDirectoryIndex;
impl XFontDirectoryIndex {
    pub fn info() -> Language {
        Language {
            name: "X Font Directory Index",
            type_: "data",
            color: "",
            extensions: &[],
            tm_scope: "source.fontdir",
            ace_mode: "text",
            language_id: 208700028,
        }
    }
}
pub struct XPixMap;
impl XPixMap {
    pub fn info() -> Language {
        Language {
            name: "X PixMap",
            type_: "data",
            color: "",
            extensions: &[".xpm", ".pm"],
            tm_scope: "source.c",
            ace_mode: "c_cpp",
            language_id: 781846279,
        }
    }
}
pub struct X10;
impl X10 {
    pub fn info() -> Language {
        Language {
            name: "X10",
            type_: "programming",
            color: "#4B6BEF",
            extensions: &[".x10"],
            tm_scope: "source.x10",
            ace_mode: "text",
            language_id: 397,
        }
    }
}
pub struct XC;
impl XC {
    pub fn info() -> Language {
        Language {
            name: "XC",
            type_: "programming",
            color: "#99DA07",
            extensions: &[".xc"],
            tm_scope: "source.xc",
            ace_mode: "c_cpp",
            language_id: 398,
        }
    }
}
pub struct XCompose;
impl XCompose {
    pub fn info() -> Language {
        Language {
            name: "XCompose",
            type_: "data",
            color: "",
            extensions: &[],
            tm_scope: "config.xcompose",
            ace_mode: "text",
            language_id: 225167241,
        }
    }
}
pub struct XML;
impl XML {
    pub fn info() -> Language {
        Language {
            name: "XML",
            type_: "data",
            color: "#0060ac",
            extensions: &[
                ".xml",
                ".adml",
                ".admx",
                ".ant",
                ".axaml",
                ".axml",
                ".builds",
                ".ccproj",
                ".ccxml",
                ".clixml",
                ".cproject",
                ".cscfg",
                ".csdef",
                ".csl",
                ".csproj",
                ".ct",
                ".depproj",
                ".dita",
                ".ditamap",
                ".ditaval",
                ".dll.config",
                ".dotsettings",
                ".filters",
                ".fsproj",
                ".fxml",
                ".glade",
                ".gml",
                ".gmx",
                ".grxml",
                ".gst",
                ".hzp",
                ".iml",
                ".ivy",
                ".jelly",
                ".jsproj",
                ".kml",
                ".launch",
                ".mdpolicy",
                ".mjml",
                ".mm",
                ".mod",
                ".mojo",
                ".mxml",
                ".natvis",
                ".ncl",
                ".ndproj",
                ".nproj",
                ".nuspec",
                ".odd",
                ".osm",
                ".pkgproj",
                ".pluginspec",
                ".proj",
                ".props",
                ".ps1xml",
                ".psc1",
                ".pt",
                ".qhelp",
                ".rdf",
                ".res",
                ".resx",
                ".rs",
                ".rss",
                ".sch",
                ".scxml",
                ".sfproj",
                ".shproj",
                ".srdf",
                ".storyboard",
                ".sublime-snippet",
                ".sw",
                ".targets",
                ".tml",
                ".ts",
                ".tsx",
                ".typ",
                ".ui",
                ".urdf",
                ".ux",
                ".vbproj",
                ".vcxproj",
                ".vsixmanifest",
                ".vssettings",
                ".vstemplate",
                ".vxml",
                ".wixproj",
                ".workflow",
                ".wsdl",
                ".wsf",
                ".wxi",
                ".wxl",
                ".wxs",
                ".x3d",
                ".xacro",
                ".xaml",
                ".xib",
                ".xlf",
                ".xliff",
                ".xmi",
                ".xml.dist",
                ".xmp",
                ".xproj",
                ".xsd",
                ".xspec",
                ".xul",
                ".zcml",
            ],
            tm_scope: "text.xml",
            ace_mode: "xml",
            language_id: 399,
        }
    }
}
pub struct XMLPropertyList;
impl XMLPropertyList {
    pub fn info() -> Language {
        Language {
            name: "XML Property List",
            type_: "data",
            color: "#0060ac",
            extensions: &[
                ".plist",
                ".stTheme",
                ".tmCommand",
                ".tmLanguage",
                ".tmPreferences",
                ".tmSnippet",
                ".tmTheme",
            ],
            tm_scope: "text.xml.plist",
            ace_mode: "xml",
            language_id: 75622871,
        }
    }
}
pub struct XPages;
impl XPages {
    pub fn info() -> Language {
        Language {
            name: "XPages",
            type_: "data",
            color: "",
            extensions: &[".xsp-config", ".xsp.metadata"],
            tm_scope: "text.xml",
            ace_mode: "xml",
            language_id: 400,
        }
    }
}
pub struct XProc;
impl XProc {
    pub fn info() -> Language {
        Language {
            name: "XProc",
            type_: "programming",
            color: "",
            extensions: &[".xpl", ".xproc"],
            tm_scope: "text.xml",
            ace_mode: "xml",
            language_id: 401,
        }
    }
}
pub struct XQuery;
impl XQuery {
    pub fn info() -> Language {
        Language {
            name: "XQuery",
            type_: "programming",
            color: "#5232e7",
            extensions: &[".xquery", ".xq", ".xql", ".xqm", ".xqy"],
            tm_scope: "source.xq",
            ace_mode: "xquery",
            language_id: 402,
        }
    }
}
pub struct XS;
impl XS {
    pub fn info() -> Language {
        Language {
            name: "XS",
            type_: "programming",
            color: "",
            extensions: &[".xs"],
            tm_scope: "source.c",
            ace_mode: "c_cpp",
            language_id: 403,
        }
    }
}
pub struct XSLT;
impl XSLT {
    pub fn info() -> Language {
        Language {
            name: "XSLT",
            type_: "programming",
            color: "#EB8CEB",
            extensions: &[".xslt", ".xsl"],
            tm_scope: "text.xml.xsl",
            ace_mode: "xml",
            language_id: 404,
        }
    }
}
pub struct Xojo;
impl Xojo {
    pub fn info() -> Language {
        Language {
            name: "Xojo",
            type_: "programming",
            color: "#81bd41",
            extensions: &[
                ".xojo_code",
                ".xojo_menu",
                ".xojo_report",
                ".xojo_script",
                ".xojo_toolbar",
                ".xojo_window",
            ],
            tm_scope: "source.xojo",
            ace_mode: "text",
            language_id: 405,
        }
    }
}
pub struct Xonsh;
impl Xonsh {
    pub fn info() -> Language {
        Language {
            name: "Xonsh",
            type_: "programming",
            color: "#285EEF",
            extensions: &[".xsh"],
            tm_scope: "source.python",
            ace_mode: "text",
            language_id: 614078284,
        }
    }
}
pub struct Xtend;
impl Xtend {
    pub fn info() -> Language {
        Language {
            name: "Xtend",
            type_: "programming",
            color: "#24255d",
            extensions: &[".xtend"],
            tm_scope: "source.xtend",
            ace_mode: "text",
            language_id: 406,
        }
    }
}
pub struct YAML;
impl YAML {
    pub fn info() -> Language {
        Language {
            name: "YAML",
            type_: "data",
            color: "#cb171e",
            extensions: &[
                ".yml",
                ".mir",
                ".reek",
                ".rviz",
                ".sublime-syntax",
                ".syntax",
                ".yaml",
                ".yaml-tmlanguage",
                ".yaml.sed",
                ".yml.mysql",
            ],
            tm_scope: "source.yaml",
            ace_mode: "yaml",
            language_id: 407,
        }
    }
}
pub struct YANG;
impl YANG {
    pub fn info() -> Language {
        Language {
            name: "YANG",
            type_: "data",
            color: "",
            extensions: &[".yang"],
            tm_scope: "source.yang",
            ace_mode: "text",
            language_id: 408,
        }
    }
}
pub struct YARA;
impl YARA {
    pub fn info() -> Language {
        Language {
            name: "YARA",
            type_: "programming",
            color: "#220000",
            extensions: &[".yar", ".yara"],
            tm_scope: "source.yara",
            ace_mode: "text",
            language_id: 805122868,
        }
    }
}
pub struct YASnippet;
impl YASnippet {
    pub fn info() -> Language {
        Language {
            name: "YASnippet",
            type_: "markup",
            color: "#32AB90",
            extensions: &[".yasnippet"],
            tm_scope: "source.yasnippet",
            ace_mode: "text",
            language_id: 378760102,
        }
    }
}
pub struct Yacc;
impl Yacc {
    pub fn info() -> Language {
        Language {
            name: "Yacc",
            type_: "programming",
            color: "#4B6C4B",
            extensions: &[".y", ".yacc", ".yy"],
            tm_scope: "source.yacc",
            ace_mode: "text",
            language_id: 409,
        }
    }
}
pub struct Yul;
impl Yul {
    pub fn info() -> Language {
        Language {
            name: "Yul",
            type_: "programming",
            color: "#794932",
            extensions: &[".yul"],
            tm_scope: "source.yul",
            ace_mode: "text",
            language_id: 237469033,
        }
    }
}
pub struct ZAP;
impl ZAP {
    pub fn info() -> Language {
        Language {
            name: "ZAP",
            type_: "programming",
            color: "#0d665e",
            extensions: &[".zap", ".xzap"],
            tm_scope: "source.zap",
            ace_mode: "text",
            language_id: 952972794,
        }
    }
}
pub struct ZIL;
impl ZIL {
    pub fn info() -> Language {
        Language {
            name: "ZIL",
            type_: "programming",
            color: "#dc75e5",
            extensions: &[".zil", ".mud"],
            tm_scope: "source.zil",
            ace_mode: "text",
            language_id: 973483626,
        }
    }
}
pub struct Zeek;
impl Zeek {
    pub fn info() -> Language {
        Language {
            name: "Zeek",
            type_: "programming",
            color: "",
            extensions: &[".zeek", ".bro"],
            tm_scope: "source.zeek",
            ace_mode: "text",
            language_id: 40,
        }
    }
}
pub struct ZenScript;
impl ZenScript {
    pub fn info() -> Language {
        Language {
            name: "ZenScript",
            type_: "programming",
            color: "#00BCD1",
            extensions: &[".zs"],
            tm_scope: "source.zenscript",
            ace_mode: "text",
            language_id: 494938890,
        }
    }
}
pub struct Zephir;
impl Zephir {
    pub fn info() -> Language {
        Language {
            name: "Zephir",
            type_: "programming",
            color: "#118f9e",
            extensions: &[".zep"],
            tm_scope: "source.php.zephir",
            ace_mode: "php",
            language_id: 410,
        }
    }
}
pub struct Zig;
impl Zig {
    pub fn info() -> Language {
        Language {
            name: "Zig",
            type_: "programming",
            color: "#ec915c",
            extensions: &[".zig", ".zig.zon"],
            tm_scope: "source.zig",
            ace_mode: "text",
            language_id: 646424281,
        }
    }
}
pub struct Zimpl;
impl Zimpl {
    pub fn info() -> Language {
        Language {
            name: "Zimpl",
            type_: "programming",
            color: "#d67711",
            extensions: &[".zimpl", ".zmpl", ".zpl"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 411,
        }
    }
}
pub struct CURLConfig;
impl CURLConfig {
    pub fn info() -> Language {
        Language {
            name: "cURL Config",
            type_: "data",
            color: "",
            extensions: &[],
            tm_scope: "source.curlrc",
            ace_mode: "text",
            language_id: 992375436,
        }
    }
}
pub struct Crontab;
impl Crontab {
    pub fn info() -> Language {
        Language {
            name: "crontab",
            type_: "data",
            color: "#ead7ac",
            extensions: &[],
            tm_scope: "text.crontab",
            ace_mode: "tcl",
            language_id: 705203557,
        }
    }
}
pub struct Desktop;
impl Desktop {
    pub fn info() -> Language {
        Language {
            name: "desktop",
            type_: "data",
            color: "",
            extensions: &[".desktop", ".desktop.in", ".service"],
            tm_scope: "source.desktop",
            ace_mode: "text",
            language_id: 412,
        }
    }
}
pub struct Dircolors;
impl Dircolors {
    pub fn info() -> Language {
        Language {
            name: "dircolors",
            type_: "data",
            color: "",
            extensions: &[".dircolors"],
            tm_scope: "source.dircolors",
            ace_mode: "text",
            language_id: 691605112,
        }
    }
}
pub struct EC;
impl EC {
    pub fn info() -> Language {
        Language {
            name: "eC",
            type_: "programming",
            color: "#913960",
            extensions: &[".ec", ".eh"],
            tm_scope: "source.c.ec",
            ace_mode: "text",
            language_id: 413,
        }
    }
}
pub struct Edn;
impl Edn {
    pub fn info() -> Language {
        Language {
            name: "edn",
            type_: "data",
            color: "",
            extensions: &[".edn"],
            tm_scope: "source.clojure",
            ace_mode: "clojure",
            language_id: 414,
        }
    }
}
pub struct Fish;
impl Fish {
    pub fn info() -> Language {
        Language {
            name: "fish",
            type_: "programming",
            color: "#4aae47",
            extensions: &[".fish"],
            tm_scope: "source.fish",
            ace_mode: "text",
            language_id: 415,
        }
    }
}
pub struct Hoon;
impl Hoon {
    pub fn info() -> Language {
        Language {
            name: "hoon",
            type_: "programming",
            color: "#00b171",
            extensions: &[".hoon"],
            tm_scope: "source.hoon",
            ace_mode: "text",
            language_id: 560883276,
        }
    }
}
pub struct ICalendar;
impl ICalendar {
    pub fn info() -> Language {
        Language {
            name: "iCalendar",
            type_: "data",
            color: "#ec564c",
            extensions: &[".ics", ".ical"],
            tm_scope: "source.iCalendar",
            ace_mode: "properties",
            language_id: 98384424,
        }
    }
}
pub struct Jq;
impl Jq {
    pub fn info() -> Language {
        Language {
            name: "jq",
            type_: "programming",
            color: "#c7254e",
            extensions: &[".jq"],
            tm_scope: "source.jq",
            ace_mode: "text",
            language_id: 905371884,
        }
    }
}
pub struct Kvlang;
impl Kvlang {
    pub fn info() -> Language {
        Language {
            name: "kvlang",
            type_: "markup",
            color: "#1da6e0",
            extensions: &[".kv"],
            tm_scope: "source.python.kivy",
            ace_mode: "text",
            language_id: 970675279,
        }
    }
}
pub struct MIRCScript;
impl MIRCScript {
    pub fn info() -> Language {
        Language {
            name: "mIRC Script",
            type_: "programming",
            color: "#3d57c3",
            extensions: &[".mrc"],
            tm_scope: "source.msl",
            ace_mode: "text",
            language_id: 517654727,
        }
    }
}
pub struct Mcfunction;
impl Mcfunction {
    pub fn info() -> Language {
        Language {
            name: "mcfunction",
            type_: "programming",
            color: "#E22837",
            extensions: &[".mcfunction"],
            tm_scope: "source.mcfunction",
            ace_mode: "text",
            language_id: 462488745,
        }
    }
}
pub struct Mupad;
impl Mupad {
    pub fn info() -> Language {
        Language {
            name: "mupad",
            type_: "programming",
            color: "#244963",
            extensions: &[".mu"],
            tm_scope: "source.mupad",
            ace_mode: "text",
            language_id: 416,
        }
    }
}
pub struct Nanorc;
impl Nanorc {
    pub fn info() -> Language {
        Language {
            name: "nanorc",
            type_: "data",
            color: "#2d004d",
            extensions: &[".nanorc"],
            tm_scope: "source.nanorc",
            ace_mode: "text",
            language_id: 775996197,
        }
    }
}
pub struct NesC;
impl NesC {
    pub fn info() -> Language {
        Language {
            name: "nesC",
            type_: "programming",
            color: "#94B0C7",
            extensions: &[".nc"],
            tm_scope: "source.nesc",
            ace_mode: "text",
            language_id: 417,
        }
    }
}
pub struct Ooc;
impl Ooc {
    pub fn info() -> Language {
        Language {
            name: "ooc",
            type_: "programming",
            color: "#b0b77e",
            extensions: &[".ooc"],
            tm_scope: "source.ooc",
            ace_mode: "text",
            language_id: 418,
        }
    }
}
pub struct Q;
impl Q {
    pub fn info() -> Language {
        Language {
            name: "q",
            type_: "programming",
            color: "#0040cd",
            extensions: &[".q"],
            tm_scope: "source.q",
            ace_mode: "text",
            language_id: 970539067,
        }
    }
}
pub struct ReStructuredText;
impl ReStructuredText {
    pub fn info() -> Language {
        Language {
            name: "reStructuredText",
            type_: "prose",
            color: "#141414",
            extensions: &[".rst", ".rest", ".rest.txt", ".rst.txt"],
            tm_scope: "text.restructuredtext",
            ace_mode: "text",
            language_id: 419,
        }
    }
}
pub struct Robotstxt;
impl Robotstxt {
    pub fn info() -> Language {
        Language {
            name: "robots.txt",
            type_: "data",
            color: "",
            extensions: &[],
            tm_scope: "text.robots-txt",
            ace_mode: "text",
            language_id: 674736065,
        }
    }
}
pub struct Sed;
impl Sed {
    pub fn info() -> Language {
        Language {
            name: "sed",
            type_: "programming",
            color: "#64b970",
            extensions: &[".sed"],
            tm_scope: "source.sed",
            ace_mode: "text",
            language_id: 847830017,
        }
    }
}
pub struct Templ;
impl Templ {
    pub fn info() -> Language {
        Language {
            name: "templ",
            type_: "markup",
            color: "#66D0DD",
            extensions: &[".templ"],
            tm_scope: "source.templ",
            ace_mode: "text",
            language_id: 795579337,
        }
    }
}
pub struct VCard;
impl VCard {
    pub fn info() -> Language {
        Language {
            name: "vCard",
            type_: "data",
            color: "#ee2647",
            extensions: &[".vcf"],
            tm_scope: "source.vcard",
            ace_mode: "properties",
            language_id: 851476558,
        }
    }
}
pub struct Wisp;
impl Wisp {
    pub fn info() -> Language {
        Language {
            name: "wisp",
            type_: "programming",
            color: "#7582D1",
            extensions: &[".wisp"],
            tm_scope: "source.clojure",
            ace_mode: "clojure",
            language_id: 420,
        }
    }
}
pub struct XBase;
impl XBase {
    pub fn info() -> Language {
        Language {
            name: "xBase",
            type_: "programming",
            color: "#403a40",
            extensions: &[".prg", ".ch", ".prw"],
            tm_scope: "source.harbour",
            ace_mode: "text",
            language_id: 421,
        }
    }
}
pub struct Languages {
    languages: Vec<Language>,
    by_name: HashMap<&'static str, usize>,
    by_extension: HashMap<&'static str, usize>,
}
impl Languages {
    pub fn new() -> Self {
        let languages = vec![
            _1CEnterprise::info(), _2DimensionalArray::info(), _4D::info(), ABAP::info(),
            ABAPCDS::info(), ABNF::info(), AGSScript::info(), AIDL::info(), AL::info(),
            AMPL::info(), ANTLR::info(), APIBlueprint::info(), APL::info(), ASL::info(),
            ASN1::info(), ASPNET::info(), ATS::info(), ActionScript::info(), Ada::info(),
            AdblockFilterList::info(), AdobeFontMetrics::info(), Agda::info(),
            Alloy::info(), AlpineAbuild::info(), AltiumDesigner::info(),
            AngelScript::info(), AntBuildSystem::info(), Antlers::info(),
            ApacheConf::info(), Apex::info(), ApolloGuidanceComputer::info(),
            AppleScript::info(), Arc::info(), AsciiDoc::info(), AspectJ::info(),
            Assembly::info(), Astro::info(), Asymptote::info(), Augeas::info(),
            AutoHotkey::info(), AutoIt::info(), AvroIDL::info(), Awk::info(),
            B4X::info(), BASIC::info(), BQN::info(), Ballerina::info(),
            Batchfile::info(), Beef::info(), Befunge::info(), Berry::info(),
            BibTeX::info(), Bicep::info(), Bikeshed::info(), Bison::info(),
            BitBake::info(), Blade::info(), BlitzBasic::info(), BlitzMax::info(),
            Bluespec::info(), BluespecBH::info(), Boo::info(), Boogie::info(),
            Brainfuck::info(), BrighterScript::info(), Brightscript::info(),
            Browserslist::info(), C::info(), Csharp::info(), Cpp::info(),
            CObjDump::info(), C2hsHaskell::info(), CAPCDS::info(), CIL::info(),
            CLIPS::info(), CMake::info(), COBOL::info(), CODEOWNERS::info(),
            COLLADA::info(), CSON::info(), CSS::info(), CSV::info(), CUE::info(),
            CWeb::info(), CabalConfig::info(), Caddyfile::info(), Cadence::info(),
            Cairo::info(), CairoZero::info(), CameLIGO::info(), CapnProto::info(),
            Carbon::info(), CartoCSS::info(), Ceylon::info(), Chapel::info(),
            Charity::info(), Checksums::info(), ChucK::info(), Circom::info(),
            Cirru::info(), Clarion::info(), Clarity::info(), ClassicASP::info(),
            Clean::info(), Click::info(), Clojure::info(), ClosureTemplates::info(),
            CloudFirestoreSecurityRules::info(), CoNLLU::info(), CodeQL::info(),
            CoffeeScript::info(), ColdFusion::info(), ColdFusionCFC::info(),
            CommonLisp::info(), CommonWorkflowLanguage::info(), ComponentPascal::info(),
            Cool::info(), Coq::info(), CppObjDump::info(), Creole::info(),
            Crystal::info(), Csound::info(), CsoundDocument::info(), CsoundScore::info(),
            Cuda::info(), CueSheet::info(), Curry::info(), Cycript::info(), Cylc::info(),
            Cypher::info(), Cython::info(), D::info(), DObjDump::info(), D2::info(),
            DIGITALCommandLanguage::info(), DM::info(), DNSZone::info(), DTrace::info(),
            Dafny::info(), DarcsPatch::info(), Dart::info(), DataWeave::info(),
            DebianPackageControlFile::info(), DenizenScript::info(), Dhall::info(),
            Diff::info(), DirectX3DFile::info(), Dockerfile::info(), Dogescript::info(),
            Dotenv::info(), Dune::info(), Dylan::info(), E::info(), EMail::info(),
            EBNF::info(), ECL::info(), ECLiPSe::info(), EJS::info(), EQ::info(),
            Eagle::info(), Earthly::info(), Easybuild::info(), EcereProjects::info(),
            Ecmarkup::info(), Edge::info(), EdgeQL::info(), EditorConfig::info(),
            EdjeDataCollection::info(), Eiffel::info(), Elixir::info(), Elm::info(),
            Elvish::info(), ElvishTranscript::info(), EmacsLisp::info(),
            EmberScript::info(), Erlang::info(), Euphoria::info(), Fsharp::info(),
            Fstar::info(), FIGletFont::info(), FIRRTL::info(), FLUX::info(),
            Factor::info(), Fancy::info(), Fantom::info(), Faust::info(), Fennel::info(),
            FilebenchWML::info(), Filterscript::info(), Fluent::info(),
            Formatted::info(), Forth::info(), Fortran::info(), FortranFreeForm::info(),
            FreeBasic::info(), FreeMarker::info(), Frege::info(), Futhark::info(),
            GCode::info(), GAML::info(), GAMS::info(), GAP::info(),
            GCCMachineDescription::info(), GDB::info(), GDScript::info(), GEDCOM::info(),
            GLSL::info(), GN::info(), GSC::info(), GameMakerLanguage::info(),
            Gemfilelock::info(), Gemini::info(), Genero4gl::info(), GeneroPer::info(),
            Genie::info(), Genshi::info(), GentooEbuild::info(), GentooEclass::info(),
            GerberImage::info(), GettextCatalog::info(), Gherkin::info(),
            GitAttributes::info(), GitConfig::info(), GitRevisionList::info(),
            Gleam::info(), GlimmerJS::info(), GlimmerTS::info(), Glyph::info(),
            GlyphBitmapDistributionFormat::info(), Gnuplot::info(), Go::info(),
            GoChecksums::info(), GoModule::info(), GoWorkspace::info(),
            GodotResource::info(), Golo::info(), Gosu::info(), Grace::info(),
            Gradle::info(), GradleKotlinDSL::info(), GrammaticalFramework::info(),
            GraphModelingLanguage::info(), GraphQL::info(), GraphvizDOT::info(),
            Groovy::info(), GroovyServerPages::info(), HAProxy::info(), HCL::info(),
            HLSL::info(), HOCON::info(), HTML::info(), HTMLpECR::info(),
            HTMLpEEX::info(), HTMLpERB::info(), HTMLpPHP::info(), HTMLpRazor::info(),
            HTTP::info(), HXML::info(), Hack::info(), Haml::info(), Handlebars::info(),
            Harbour::info(), Haskell::info(), Haxe::info(), HiveQL::info(),
            HolyC::info(), HostsFile::info(), Hy::info(), HyPhy::info(), IDL::info(),
            IGORPro::info(), INI::info(), IRCLog::info(), Idris::info(),
            IgnoreList::info(), ImageJMacro::info(), Imba::info(), Inform7::info(),
            Ink::info(), InnoSetup::info(), Io::info(), Ioke::info(), Isabelle::info(),
            IsabelleROOT::info(), J::info(), JARManifest::info(), JCL::info(),
            JFlex::info(), JSON::info(), JSONWithComments::info(), JSON5::info(),
            JSONLD::info(), JSONiq::info(), Janet::info(), Jasmin::info(), Java::info(),
            JavaProperties::info(), JavaServerPages::info(), JavaTemplateEngine::info(),
            JavaScript::info(), JavaScriptpERB::info(), JestSnapshot::info(),
            JetBrainsMPS::info(), Jinja::info(), Jison::info(), JisonLex::info(),
            Jolie::info(), Jsonnet::info(), Julia::info(), JuliaREPL::info(),
            JupyterNotebook::info(), Just::info(), KRL::info(), KaitaiStruct::info(),
            KakouneScript::info(), KerboScript::info(), KiCadLayout::info(),
            KiCadLegacyLayout::info(), KiCadSchematic::info(), Kickstart::info(),
            Kit::info(), Kotlin::info(), Kusto::info(), LFE::info(), LLVM::info(),
            LOLCODE::info(), LSL::info(), LTspiceSymbol::info(), LabVIEW::info(),
            Lark::info(), Lasso::info(), Latte::info(), Lean::info(), Lean4::info(),
            Less::info(), Lex::info(), LigoLANG::info(), LilyPond::info(), Limbo::info(),
            LinkerScript::info(), LinuxKernelModule::info(), Liquid::info(),
            LiterateAgda::info(), LiterateCoffeeScript::info(), LiterateHaskell::info(),
            LiveCodeScript::info(), LiveScript::info(), Logos::info(), Logtalk::info(),
            LookML::info(), LoomScript::info(), Lua::info(), Luau::info(), M::info(),
            M4::info(), M4Sugar::info(), MATLAB::info(), MAXScript::info(), MDX::info(),
            MLIR::info(), MQL4::info(), MQL5::info(), MTML::info(), MUF::info(),
            Macaulay2::info(), Makefile::info(), Mako::info(), Markdown::info(),
            Marko::info(), Mask::info(), Mathematica::info(), MavenPOM::info(),
            Max::info(), Mercury::info(), Mermaid::info(), Meson::info(), Metal::info(),
            MicrosoftDeveloperStudioProject::info(),
            MicrosoftVisualStudioSolution::info(), MiniD::info(), MiniYAML::info(),
            Mint::info(), Mirah::info(), Modelica::info(), Modula2::info(),
            Modula3::info(), ModuleManagementSystem::info(), Mojo::info(),
            Monkey::info(), MonkeyC::info(), Moocode::info(), MoonScript::info(),
            Motoko::info(), Motorola68KAssembly::info(), Move::info(), Muse::info(),
            Mustache::info(), Myghty::info(), NASL::info(), NCL::info(), NEON::info(),
            NL::info(), NMODL::info(), NPMConfig::info(), NSIS::info(), NWScript::info(),
            Nasal::info(), Nearley::info(), Nemerle::info(), NetLinx::info(),
            NetLinxpERB::info(), NetLogo::info(), NewLisp::info(), Nextflow::info(),
            Nginx::info(), Nim::info(), Ninja::info(), Nit::info(), Nix::info(),
            Noir::info(), Nu::info(), NumPy::info(), Nunjucks::info(), Nushell::info(),
            OASv2Json::info(), OASv2Yaml::info(), OASv3Json::info(), OASv3Yaml::info(),
            OCaml::info(), Oberon::info(), ObjDump::info(),
            ObjectDataInstanceNotation::info(), ObjectScript::info(), ObjectiveC::info(),
            ObjectiveCpp::info(), ObjectiveJ::info(), Odin::info(), Omgrofl::info(),
            Opa::info(), Opal::info(), OpenPolicyAgent::info(),
            OpenAPISpecificationV2::info(), OpenAPISpecificationV3::info(),
            OpenCL::info(), OpenEdgeABL::info(), OpenQASM::info(),
            OpenRCRunscript::info(), OpenSCAD::info(), OpenStepPropertyList::info(),
            OpenTypeFeatureFile::info(), OptionList::info(), Org::info(), Ox::info(),
            Oxygene::info(), Oz::info(), P4::info(), PDDL::info(), PEGjs::info(),
            PHP::info(), PLSQL::info(), PLpgSQL::info(), POVRaySDL::info(), Pact::info(),
            Pan::info(), Papyrus::info(), Parrot::info(), ParrotAssembly::info(),
            ParrotInternalRepresentation::info(), Pascal::info(), Pawn::info(),
            Pep8::info(), Perl::info(), Pic::info(), Pickle::info(), PicoLisp::info(),
            PigLatin::info(), Pike::info(), PipRequirements::info(), Pkl::info(),
            PlantUML::info(), Pod::info(), Pod6::info(), PogoScript::info(),
            Polar::info(), Pony::info(), Portugol::info(), PostCSS::info(),
            PostScript::info(), PowerBuilder::info(), PowerShell::info(), Praat::info(),
            Prisma::info(), Processing::info(), Procfile::info(), Proguard::info(),
            Prolog::info(), Promela::info(), PropellerSpin::info(),
            ProtocolBuffer::info(), ProtocolBufferTextFormat::info(), PublicKey::info(),
            Pug::info(), Puppet::info(), PureData::info(), PureBasic::info(),
            PureScript::info(), Pyret::info(), Python::info(), PythonConsole::info(),
            PythonTraceback::info(), Qsharp::info(), QML::info(), QMake::info(),
            QtScript::info(), Quake::info(), R::info(), RAML::info(), RBS::info(),
            RDoc::info(), REALbasic::info(), REXX::info(), RMarkdown::info(),
            RON::info(), RPC::info(), RPGLE::info(), RPMSpec::info(), RUNOFF::info(),
            Racket::info(), Ragel::info(), Raku::info(), Rascal::info(),
            RawTokenData::info(), ReScript::info(), ReadlineConfig::info(),
            Reason::info(), ReasonLIGO::info(), Rebol::info(), RecordJar::info(),
            Red::info(), Redcode::info(), RedirectRules::info(),
            RegularExpression::info(), RenPy::info(), RenderScript::info(), Rez::info(),
            RichTextFormat::info(), Ring::info(), Riot::info(), RobotFramework::info(),
            Roc::info(), Roff::info(), RoffManpage::info(), Rouge::info(),
            RouterOSScript::info(), Ruby::info(), Rust::info(), SAS::info(),
            SCSS::info(), SELinuxPolicy::info(), SMT::info(), SPARQL::info(),
            SQF::info(), SQL::info(), SQLPL::info(), SRecodeTemplate::info(),
            SSHConfig::info(), STAR::info(), STL::info(), STON::info(), SVG::info(),
            SWIG::info(), Sage::info(), SaltStack::info(), Sass::info(), Scala::info(),
            Scaml::info(), Scenic::info(), Scheme::info(), Scilab::info(), _Self::info(),
            ShaderLab::info(), Shell::info(), ShellCheckConfig::info(),
            ShellSession::info(), Shen::info(), Sieve::info(),
            SimpleFileVerification::info(), Singularity::info(), Slash::info(),
            Slice::info(), Slim::info(), Slint::info(), SmPL::info(), Smali::info(),
            Smalltalk::info(), Smarty::info(), Smithy::info(), Snakemake::info(),
            Solidity::info(), Soong::info(), SourcePawn::info(),
            SplineFontDatabase::info(), Squirrel::info(), Stan::info(),
            StandardML::info(), Starlark::info(), Stata::info(), StringTemplate::info(),
            Stylus::info(), SubRipText::info(), SugarSS::info(), SuperCollider::info(),
            Svelte::info(), Sway::info(), Sweave::info(), Swift::info(),
            SystemVerilog::info(), TIProgram::info(), TLVerilog::info(), TLA::info(),
            TOML::info(), TSQL::info(), TSV::info(), TSX::info(), TXL::info(),
            Talon::info(), Tcl::info(), Tcsh::info(), TeX::info(), Tea::info(),
            Terra::info(), TerraformTemplate::info(), Texinfo::info(), Text::info(),
            TextGrid::info(), TextMateProperties::info(), Textile::info(),
            Thrift::info(), Toit::info(), Turing::info(), Turtle::info(), Twig::info(),
            TypeLanguage::info(), TypeScript::info(), Typst::info(),
            UnifiedParallelC::info(), Unity3DAsset::info(), UnixAssembly::info(),
            Uno::info(), UnrealScript::info(), UrWeb::info(), V::info(), VBA::info(),
            VBScript::info(), VCL::info(), VHDL::info(), Vala::info(),
            ValveDataFormat::info(), VelocityTemplateLanguage::info(), Verilog::info(),
            VimHelpFile::info(), VimScript::info(), VimSnippet::info(),
            VisualBasicNET::info(), VisualBasic60::info(), Volt::info(), Vue::info(),
            Vyper::info(), WDL::info(), WGSL::info(), WavefrontMaterial::info(),
            WavefrontObject::info(), WebOntologyLanguage::info(), WebAssembly::info(),
            WebAssemblyInterfaceType::info(), WebIDL::info(), WebVTT::info(),
            WgetConfig::info(), Whiley::info(), Wikitext::info(),
            Win32MessageFile::info(), WindowsRegistryEntries::info(),
            WitcherScript::info(), Wollok::info(), WorldOfWarcraftAddonData::info(),
            Wren::info(), XBitMap::info(), XFontDirectoryIndex::info(), XPixMap::info(),
            X10::info(), XC::info(), XCompose::info(), XML::info(),
            XMLPropertyList::info(), XPages::info(), XProc::info(), XQuery::info(),
            XS::info(), XSLT::info(), Xojo::info(), Xonsh::info(), Xtend::info(),
            YAML::info(), YANG::info(), YARA::info(), YASnippet::info(), Yacc::info(),
            Yul::info(), ZAP::info(), ZIL::info(), Zeek::info(), ZenScript::info(),
            Zephir::info(), Zig::info(), Zimpl::info(), CURLConfig::info(),
            Crontab::info(), Desktop::info(), Dircolors::info(), EC::info(), Edn::info(),
            Fish::info(), Hoon::info(), ICalendar::info(), Jq::info(), Kvlang::info(),
            MIRCScript::info(), Mcfunction::info(), Mupad::info(), Nanorc::info(),
            NesC::info(), Ooc::info(), Q::info(), ReStructuredText::info(),
            Robotstxt::info(), Sed::info(), Templ::info(), VCard::info(), Wisp::info(),
            XBase::info(),
        ];
        let mut by_name = HashMap::new();
        let mut by_extension = HashMap::new();
        for (index, lang) in languages.iter().enumerate() {
            by_name.insert(lang.name, index);
            for ext in lang.extensions {
                by_extension.insert(*ext, index);
            }
        }
        Self {
            languages,
            by_name,
            by_extension,
        }
    }
    pub fn get_by_name(&self, name: &str) -> Option<&Language> {
        self.by_name.get(name).map(|&index| &self.languages[index])
    }
    pub fn get_by_extension(&self, ext: &str) -> Option<&Language> {
        self.by_extension.get(ext).map(|&index| &self.languages[index])
    }
    pub fn all_languages(&self) -> &[Language] {
        &self.languages
    }
}
pub fn get_languages() -> Languages {
    Languages::new()
}
