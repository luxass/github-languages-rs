#![allow(clippy::all)]
use crate::define_languages;
use phf::phf_map;
define_languages! {
    _1CEnterprise => { name : "1C Enterprise", r#type : "programming", color : "#814CCC",
    extensions : [".bsl", ".os"], aliases : [], tm_scope : "source.bsl", ace_mode :
    "text", language_id : 0u64, filenames : [], interpreters : [], }, _2DimensionalArray
    => { name : "2-Dimensional Array", r#type : "data", color : "#38761D", extensions :
    [".2da"], aliases : [], tm_scope : "source.2da", ace_mode : "text", language_id :
    387204628u64, filenames : [], interpreters : [], }, _4D => { name : "4D", r#type :
    "programming", color : "#004289", extensions : [".4dm"], aliases : [], tm_scope :
    "source.4dm", ace_mode : "text", language_id : 577529595u64, filenames : [],
    interpreters : [], }, ABAP => { name : "ABAP", r#type : "programming", color :
    "#E8274B", extensions : [".abap"], aliases : [], tm_scope : "source.abap", ace_mode :
    "abap", language_id : 1u64, filenames : [], interpreters : [], }, ABAPCDS => { name :
    "ABAP CDS", r#type : "programming", color : "#555e25", extensions : [".asddls"],
    aliases : [], tm_scope : "source.abapcds", ace_mode : "text", language_id :
    452681853u64, filenames : [], interpreters : [], }, ABNF => { name : "ABNF", r#type :
    "data", color : "#000000", extensions : [".abnf"], aliases : [], tm_scope :
    "source.abnf", ace_mode : "text", language_id : 429u64, filenames : [], interpreters
    : [], }, AGSScript => { name : "AGS Script", r#type : "programming", color :
    "#B9D9FF", extensions : [".asc", ".ash"], aliases : ["ags"], tm_scope : "source.c++",
    ace_mode : "c_cpp", language_id : 2u64, codemirror_mode : "clike",
    codemirror_mime_type : "text/x-c++src", filenames : [], interpreters : [], }, AIDL =>
    { name : "AIDL", r#type : "programming", color : "#34EB6B", extensions : [".aidl"],
    aliases : [], tm_scope : "source.aidl", ace_mode : "text", language_id :
    451700185u64, filenames : [], interpreters : ["aidl"], }, AL => { name : "AL", r#type
    : "programming", color : "#3AA2B5", extensions : [".al"], aliases : [], tm_scope :
    "source.al", ace_mode : "text", language_id : 658971832u64, filenames : [],
    interpreters : [], }, AMPL => { name : "AMPL", r#type : "programming", color :
    "#E6EFBB", extensions : [".ampl", ".mod"], aliases : [], tm_scope : "source.ampl",
    ace_mode : "text", language_id : 3u64, filenames : [], interpreters : [], }, ANTLR =>
    { name : "ANTLR", r#type : "programming", color : "#9DC3FF", extensions : [".g4"],
    aliases : [], tm_scope : "source.antlr", ace_mode : "text", language_id : 4u64,
    filenames : [], interpreters : [], }, APIBlueprint => { name : "API Blueprint",
    r#type : "markup", color : "#2ACCA8", extensions : [".apib"], aliases : [], tm_scope
    : "text.html.markdown.source.gfm.apib", ace_mode : "markdown", language_id : 5u64,
    filenames : [], interpreters : [], }, APL => { name : "APL", r#type : "programming",
    color : "#5A8164", extensions : [".apl", ".dyalog"], aliases : [], tm_scope :
    "source.apl", ace_mode : "text", language_id : 6u64, codemirror_mode : "apl",
    codemirror_mime_type : "text/apl", filenames : [], interpreters : ["apl", "aplx",
    "dyalog"], }, ASL => { name : "ASL", r#type : "programming", color : "#000000",
    extensions : [".asl", ".dsl"], aliases : [], tm_scope : "source.asl", ace_mode :
    "asl", language_id : 124996147u64, filenames : [], interpreters : [], }, ASN1 => {
    name : "ASN.1", r#type : "data", color : "#000000", extensions : [".asn", ".asn1"],
    aliases : [], tm_scope : "source.asn", ace_mode : "text", language_id : 7u64,
    codemirror_mode : "asn.1", codemirror_mime_type : "text/x-ttcn-asn", filenames : [],
    interpreters : [], }, ASPNET => { name : "ASP.NET", r#type : "programming", color :
    "#9400ff", extensions : [".asax", ".ascx", ".ashx", ".asmx", ".aspx", ".axd"],
    aliases : ["aspx", "aspx-vb"], tm_scope : "text.html.asp", ace_mode : "text",
    language_id : 564186416u64, codemirror_mode : "htmlembedded", codemirror_mime_type :
    "application/x-aspx", filenames : [], interpreters : [], }, ATS => { name : "ATS",
    r#type : "programming", color : "#1ac620", extensions : [".dats", ".hats", ".sats"],
    aliases : ["ats2"], tm_scope : "source.ats", ace_mode : "ocaml", language_id : 9u64,
    filenames : [], interpreters : [], }, ActionScript => { name : "ActionScript", r#type
    : "programming", color : "#882B0F", extensions : [".as"], aliases :
    ["actionscript 3", "actionscript3", "as3"], tm_scope : "source.actionscript.3",
    ace_mode : "actionscript", language_id : 10u64, filenames : [], interpreters : [], },
    Ada => { name : "Ada", r#type : "programming", color : "#02f88c", extensions :
    [".adb", ".ada", ".ads"], aliases : ["ada95", "ada2005"], tm_scope : "source.ada",
    ace_mode : "ada", language_id : 11u64, filenames : [], interpreters : [], },
    AdblockFilterList => { name : "Adblock Filter List", r#type : "data", color :
    "#800000", extensions : [".txt"], aliases : ["ad block filters", "ad block", "adb",
    "adblock"], tm_scope : "text.adblock", ace_mode : "text", language_id : 884614762u64,
    filenames : [], interpreters : [], }, AdobeFontMetrics => { name :
    "Adobe Font Metrics", r#type : "data", color : "#fa0f00", extensions : [".afm"],
    aliases : ["acfm", "adobe composite font metrics", "adobe multiple font metrics",
    "amfm"], tm_scope : "source.afm", ace_mode : "text", language_id : 147198098u64,
    filenames : [], interpreters : [], }, Agda => { name : "Agda", r#type :
    "programming", color : "#315665", extensions : [".agda"], aliases : [], tm_scope :
    "source.agda", ace_mode : "text", language_id : 12u64, filenames : [], interpreters :
    [], }, Aiken => { name : "Aiken", r#type : "programming", color : "#640ff8",
    extensions : [".ak"], aliases : [], tm_scope : "source.aiken", ace_mode : "text",
    language_id : 899409497u64, filenames : [], interpreters : [], }, Alloy => { name :
    "Alloy", r#type : "programming", color : "#64C800", extensions : [".als"], aliases :
    [], tm_scope : "source.alloy", ace_mode : "text", language_id : 13u64, filenames :
    [], interpreters : [], }, AlpineAbuild => { name : "Alpine Abuild", r#type :
    "programming", color : "#0D597F", extensions : [], aliases : ["abuild", "apkbuild"],
    tm_scope : "source.shell", ace_mode : "sh", language_id : 14u64, codemirror_mode :
    "shell", codemirror_mime_type : "text/x-sh", filenames : ["APKBUILD"], group :
    "Shell", interpreters : [], }, AltiumDesigner => { name : "Altium Designer", r#type :
    "data", color : "#A89663", extensions : [".OutJob", ".PcbDoc", ".PrjPCB", ".SchDoc"],
    aliases : ["altium"], tm_scope : "source.ini", ace_mode : "ini", language_id :
    187772328u64, filenames : [], interpreters : [], }, AngelScript => { name :
    "AngelScript", r#type : "programming", color : "#C7D7DC", extensions : [".as",
    ".angelscript"], aliases : [], tm_scope : "source.angelscript", ace_mode : "text",
    language_id : 389477596u64, codemirror_mode : "clike", codemirror_mime_type :
    "text/x-c++src", filenames : [], interpreters : [], }, AnswerSetProgramming => { name
    : "Answer Set Programming", r#type : "programming", color : "#A9CC29", extensions :
    [".lp"], aliases : [], tm_scope : "source.answersetprogramming", ace_mode : "prolog",
    language_id : 433009171u64, filenames : [], interpreters : ["clingo"], },
    AntBuildSystem => { name : "Ant Build System", r#type : "data", color : "#A9157E",
    extensions : [], aliases : [], tm_scope : "text.xml.ant", ace_mode : "xml",
    language_id : 15u64, codemirror_mode : "xml", codemirror_mime_type :
    "application/xml", filenames : ["ant.xml", "build.xml"], interpreters : [], },
    Antlers => { name : "Antlers", r#type : "markup", color : "#ff269e", extensions :
    [".antlers.html", ".antlers.php", ".antlers.xml"], aliases : [], tm_scope :
    "text.html.statamic", ace_mode : "text", language_id : 1067292663u64, filenames : [],
    interpreters : [], }, ApacheConf => { name : "ApacheConf", r#type : "data", color :
    "#d12127", extensions : [".apacheconf", ".vhost"], aliases : ["aconf", "apache"],
    tm_scope : "source.apacheconf", ace_mode : "apache_conf", language_id : 16u64,
    filenames : [".htaccess", "apache2.conf", "httpd.conf"], interpreters : [], }, Apex
    => { name : "Apex", r#type : "programming", color : "#1797c0", extensions : [".cls",
    ".apex", ".trigger"], aliases : [], tm_scope : "source.apex", ace_mode : "apex",
    language_id : 17u64, codemirror_mode : "clike", codemirror_mime_type : "text/x-java",
    filenames : [], interpreters : [], }, ApolloGuidanceComputer => { name :
    "Apollo Guidance Computer", r#type : "programming", color : "#0B3D91", extensions :
    [".agc"], aliases : [], tm_scope : "source.agc", ace_mode : "assembly_x86",
    language_id : 18u64, filenames : [], group : "Assembly", interpreters : [], },
    AppleScript => { name : "AppleScript", r#type : "programming", color : "#101F1F",
    extensions : [".applescript", ".scpt"], aliases : ["osascript"], tm_scope :
    "source.applescript", ace_mode : "applescript", language_id : 19u64, filenames : [],
    interpreters : ["osascript"], }, Arc => { name : "Arc", r#type : "programming", color
    : "#aa2afe", extensions : [".arc"], aliases : [], tm_scope : "none", ace_mode :
    "text", language_id : 20u64, filenames : [], interpreters : [], }, ArkTS => { name :
    "ArkTS", r#type : "programming", color : "#0080ff", extensions : [".ets"], aliases :
    [], tm_scope : "source.ets", ace_mode : "typescript", language_id : 56341321u64,
    codemirror_mode : "javascript", codemirror_mime_type : "application/typescript",
    filenames : [], interpreters : [], }, AsciiDoc => { name : "AsciiDoc", r#type :
    "prose", color : "#73a0c5", extensions : [".asciidoc", ".adoc", ".asc"], aliases :
    [], tm_scope : "text.html.asciidoc", ace_mode : "asciidoc", language_id : 22u64, wrap
    : true, filenames : [], interpreters : [], }, AspectJ => { name : "AspectJ", r#type :
    "programming", color : "#a957b0", extensions : [".aj"], aliases : [], tm_scope :
    "source.aspectj", ace_mode : "text", language_id : 23u64, filenames : [],
    interpreters : [], }, Assembly => { name : "Assembly", r#type : "programming", color
    : "#6E4C13", extensions : [".asm", ".a51", ".i", ".inc", ".nas", ".nasm", ".s"],
    aliases : ["asm", "nasm"], tm_scope : "source.assembly", ace_mode : "assembly_x86",
    language_id : 24u64, filenames : [], interpreters : [], }, Astro => { name : "Astro",
    r#type : "markup", color : "#ff5a03", extensions : [".astro"], aliases : [], tm_scope
    : "source.astro", ace_mode : "astro", language_id : 578209015u64, codemirror_mode :
    "jsx", codemirror_mime_type : "text/jsx", filenames : [], interpreters : [], },
    Asymptote => { name : "Asymptote", r#type : "programming", color : "#ff0000",
    extensions : [".asy"], aliases : [], tm_scope : "source.c++", ace_mode : "c_cpp",
    language_id : 591605007u64, codemirror_mode : "clike", codemirror_mime_type :
    "text/x-kotlin", filenames : [], interpreters : ["asy"], }, Augeas => { name :
    "Augeas", r#type : "programming", color : "#9CC134", extensions : [".aug"], aliases :
    [], tm_scope : "none", ace_mode : "text", language_id : 25u64, filenames : [],
    interpreters : [], }, AutoHotkey => { name : "AutoHotkey", r#type : "programming",
    color : "#6594b9", extensions : [".ahk", ".ahkl"], aliases : ["ahk"], tm_scope :
    "source.ahk", ace_mode : "autohotkey", language_id : 26u64, filenames : [],
    interpreters : [], }, AutoIt => { name : "AutoIt", r#type : "programming", color :
    "#1C3552", extensions : [".au3"], aliases : ["au3", "AutoIt3", "AutoItScript"],
    tm_scope : "source.autoit", ace_mode : "autohotkey", language_id : 27u64, filenames :
    [], interpreters : [], }, AvroIDL => { name : "Avro IDL", r#type : "data", color :
    "#0040FF", extensions : [".avdl"], aliases : [], tm_scope : "source.avro", ace_mode :
    "text", language_id : 785497837u64, filenames : [], interpreters : [], }, Awk => {
    name : "Awk", r#type : "programming", color : "#c30e9b", extensions : [".awk",
    ".auk", ".gawk", ".mawk", ".nawk"], aliases : [], tm_scope : "source.awk", ace_mode :
    "text", language_id : 28u64, filenames : [], interpreters : ["awk", "gawk", "mawk",
    "nawk"], }, B4X => { name : "B4X", r#type : "programming", color : "#00e4ff",
    extensions : [".bas"], aliases : ["basic for android"], tm_scope : "source.vba",
    ace_mode : "text", language_id : 96642275u64, codemirror_mode : "vb",
    codemirror_mime_type : "text/x-vb", filenames : [], interpreters : [], }, BASIC => {
    name : "BASIC", r#type : "programming", color : "#ff0000", extensions : [".bas"],
    aliases : [], tm_scope : "source.basic", ace_mode : "basic", language_id :
    28923963u64, filenames : [], interpreters : [], }, BQN => { name : "BQN", r#type :
    "programming", color : "#2b7067", extensions : [".bqn"], aliases : [], tm_scope :
    "source.bqn", ace_mode : "text", language_id : 330386870u64, filenames : [],
    interpreters : [], }, Ballerina => { name : "Ballerina", r#type : "programming",
    color : "#FF5000", extensions : [".bal"], aliases : [], tm_scope :
    "source.ballerina", ace_mode : "text", language_id : 720859680u64, filenames : [],
    interpreters : [], }, Batchfile => { name : "Batchfile", r#type : "programming",
    color : "#C1F12E", extensions : [".bat", ".cmd"], aliases : ["bat", "batch",
    "dosbatch", "winbatch"], tm_scope : "source.batchfile", ace_mode : "batchfile",
    language_id : 29u64, filenames : ["gradlew.bat", "mvnw.cmd"], interpreters : [], },
    Beef => { name : "Beef", r#type : "programming", color : "#a52f4e", extensions :
    [".bf"], aliases : [], tm_scope : "source.cs", ace_mode : "csharp", language_id :
    545626333u64, codemirror_mode : "clike", codemirror_mime_type : "text/x-csharp",
    filenames : [], interpreters : [], }, Befunge => { name : "Befunge", r#type :
    "programming", color : "#000000", extensions : [".befunge", ".bf"], aliases : [],
    tm_scope : "source.befunge", ace_mode : "text", language_id : 30u64, filenames : [],
    interpreters : [], }, Berry => { name : "Berry", r#type : "programming", color :
    "#15A13C", extensions : [".be"], aliases : ["be"], tm_scope : "source.berry",
    ace_mode : "text", language_id : 121855308u64, filenames : [], interpreters : [], },
    BibTeX => { name : "BibTeX", r#type : "markup", color : "#778899", extensions :
    [".bib", ".bibtex"], aliases : [], tm_scope : "text.bibtex", ace_mode : "bibtex",
    language_id : 982188347u64, codemirror_mode : "stex", codemirror_mime_type :
    "text/x-stex", filenames : [], group : "TeX", interpreters : [], }, BibTeXStyle => {
    name : "BibTeX Style", r#type : "programming", color : "#000000", extensions :
    [".bst"], aliases : [], tm_scope : "source.bst", ace_mode : "text", language_id :
    909569041u64, filenames : [], interpreters : [], }, Bicep => { name : "Bicep", r#type
    : "programming", color : "#519aba", extensions : [".bicep", ".bicepparam"], aliases :
    [], tm_scope : "source.bicep", ace_mode : "text", language_id : 321200902u64,
    filenames : [], interpreters : [], }, Bikeshed => { name : "Bikeshed", r#type :
    "markup", color : "#5562ac", extensions : [".bs"], aliases : [], tm_scope :
    "source.csswg", ace_mode : "html", language_id : 1055528081u64, codemirror_mode :
    "htmlmixed", codemirror_mime_type : "text/html", filenames : [], interpreters : [],
    }, Bison => { name : "Bison", r#type : "programming", color : "#6A463F", extensions :
    [".bison"], aliases : [], tm_scope : "source.yacc", ace_mode : "text", language_id :
    31u64, filenames : [], group : "Yacc", interpreters : [], }, BitBake => { name :
    "BitBake", r#type : "programming", color : "#00bce4", extensions : [".bb",
    ".bbappend", ".bbclass", ".inc"], aliases : [], tm_scope : "source.bb", ace_mode :
    "text", language_id : 32u64, filenames : [], interpreters : [], }, Blade => { name :
    "Blade", r#type : "markup", color : "#f7523f", extensions : [".blade", ".blade.php"],
    aliases : [], tm_scope : "text.html.php.blade", ace_mode : "php_laravel_blade",
    language_id : 33u64, filenames : [], interpreters : [], }, BlitzBasic => { name :
    "BlitzBasic", r#type : "programming", color : "#00FFAE", extensions : [".bb",
    ".decls"], aliases : ["b3d", "blitz3d", "blitzplus", "bplus"], tm_scope :
    "source.blitzmax", ace_mode : "text", language_id : 34u64, filenames : [],
    interpreters : [], }, BlitzMax => { name : "BlitzMax", r#type : "programming", color
    : "#cd6400", extensions : [".bmx"], aliases : ["bmax"], tm_scope : "source.blitzmax",
    ace_mode : "text", language_id : 35u64, filenames : [], interpreters : [], },
    Bluespec => { name : "Bluespec", r#type : "programming", color : "#12223c",
    extensions : [".bsv"], aliases : ["bluespec bsv", "bsv"], tm_scope : "source.bsv",
    ace_mode : "verilog", language_id : 36u64, codemirror_mode : "verilog",
    codemirror_mime_type : "text/x-systemverilog", filenames : [], interpreters : [], },
    BluespecBH => { name : "Bluespec BH", r#type : "programming", color : "#12223c",
    extensions : [".bs"], aliases : ["bh", "bluespec classic"], tm_scope : "source.bh",
    ace_mode : "haskell", language_id : 641580358u64, codemirror_mode : "haskell",
    codemirror_mime_type : "text/x-haskell", filenames : [], group : "Bluespec",
    interpreters : [], }, Boo => { name : "Boo", r#type : "programming", color :
    "#d4bec1", extensions : [".boo"], aliases : [], tm_scope : "source.boo", ace_mode :
    "text", language_id : 37u64, filenames : [], interpreters : [], }, Boogie => { name :
    "Boogie", r#type : "programming", color : "#c80fa0", extensions : [".bpl"], aliases :
    [], tm_scope : "source.boogie", ace_mode : "text", language_id : 955017407u64,
    filenames : [], interpreters : ["boogie"], }, Brainfuck => { name : "Brainfuck",
    r#type : "programming", color : "#2F2530", extensions : [".b", ".bf"], aliases : [],
    tm_scope : "source.bf", ace_mode : "text", language_id : 38u64, codemirror_mode :
    "brainfuck", codemirror_mime_type : "text/x-brainfuck", filenames : [], interpreters
    : [], }, BrighterScript => { name : "BrighterScript", r#type : "programming", color :
    "#66AABB", extensions : [".bs"], aliases : [], tm_scope : "source.brs", ace_mode :
    "text", language_id : 943571030u64, filenames : [], interpreters : [], },
    Brightscript => { name : "Brightscript", r#type : "programming", color : "#662D91",
    extensions : [".brs"], aliases : [], tm_scope : "source.brs", ace_mode : "text",
    language_id : 39u64, filenames : [], interpreters : [], }, Browserslist => { name :
    "Browserslist", r#type : "data", color : "#ffd539", extensions : [], aliases : [],
    tm_scope : "text.browserslist", ace_mode : "text", language_id : 153503348u64,
    filenames : [".browserslistrc", "browserslist"], interpreters : [], }, Bru => { name
    : "Bru", r#type : "markup", color : "#F4AA41", extensions : [".bru"], aliases : [],
    tm_scope : "source.bru", ace_mode : "text", language_id : 906627898u64, filenames :
    [], interpreters : [], }, BuildStream => { name : "BuildStream", r#type : "data",
    color : "#006bff", extensions : [".bst"], aliases : [], tm_scope : "source.yaml",
    ace_mode : "yaml", language_id : 84359046u64, filenames : [], interpreters : [], }, C
    => { name : "C", r#type : "programming", color : "#555555", extensions : [".c",
    ".cats", ".h", ".h.in", ".idc"], aliases : [], tm_scope : "source.c", ace_mode :
    "c_cpp", language_id : 41u64, codemirror_mode : "clike", codemirror_mime_type :
    "text/x-csrc", filenames : [], interpreters : ["tcc"], }, Csharp => { name : "C#",
    r#type : "programming", color : "#178600", extensions : [".cs", ".cake", ".cs.pp",
    ".csx", ".linq"], aliases : ["csharp", "cake", "cakescript"], tm_scope : "source.cs",
    ace_mode : "csharp", language_id : 42u64, codemirror_mode : "clike",
    codemirror_mime_type : "text/x-csharp", filenames : [], interpreters : [], }, Cpp =>
    { name : "C++", r#type : "programming", color : "#f34b7d", extensions : [".cpp",
    ".c++", ".cc", ".cp", ".cppm", ".cxx", ".h", ".h++", ".hh", ".hpp", ".hxx", ".inc",
    ".inl", ".ino", ".ipp", ".ixx", ".re", ".tcc", ".tpp", ".txx"], aliases : ["cpp"],
    tm_scope : "source.c++", ace_mode : "c_cpp", language_id : 43u64, codemirror_mode :
    "clike", codemirror_mime_type : "text/x-c++src", filenames : [], interpreters : [],
    }, CObjDump => { name : "C-ObjDump", r#type : "data", color : "#000000", extensions :
    [".c-objdump"], aliases : [], tm_scope : "objdump.x86asm", ace_mode : "assembly_x86",
    language_id : 44u64, filenames : [], interpreters : [], }, C2hsHaskell => { name :
    "C2hs Haskell", r#type : "programming", color : "#000000", extensions : [".chs"],
    aliases : ["c2hs"], tm_scope : "source.haskell", ace_mode : "haskell", language_id :
    45u64, codemirror_mode : "haskell", codemirror_mime_type : "text/x-haskell",
    filenames : [], group : "Haskell", interpreters : [], }, C3 => { name : "C3", r#type
    : "programming", color : "#2563eb", extensions : [".c3"], aliases : [], tm_scope :
    "source.c3", ace_mode : "c_cpp", language_id : 769248603u64, codemirror_mode :
    "clike", codemirror_mime_type : "text/x-csrc", filenames : [], interpreters : [], },
    CAPCDS => { name : "CAP CDS", r#type : "programming", color : "#0092d1", extensions :
    [".cds"], aliases : ["cds"], tm_scope : "source.cds", ace_mode : "text", language_id
    : 390788699u64, filenames : [], interpreters : [], }, CIL => { name : "CIL", r#type :
    "data", color : "#000000", extensions : [".cil"], aliases : [], tm_scope :
    "source.cil", ace_mode : "text", language_id : 29176339u64, filenames : [],
    interpreters : [], }, CLIPS => { name : "CLIPS", r#type : "programming", color :
    "#00A300", extensions : [".clp"], aliases : [], tm_scope : "source.clips", ace_mode :
    "text", language_id : 46u64, filenames : [], interpreters : [], }, CMake => { name :
    "CMake", r#type : "programming", color : "#DA3434", extensions : [".cmake",
    ".cmake.in"], aliases : [], tm_scope : "source.cmake", ace_mode : "text", language_id
    : 47u64, codemirror_mode : "cmake", codemirror_mime_type : "text/x-cmake", filenames
    : ["CMakeLists.txt"], interpreters : [], }, COBOL => { name : "COBOL", r#type :
    "programming", color : "#000000", extensions : [".cob", ".cbl", ".ccp", ".cobol",
    ".cpy"], aliases : [], tm_scope : "source.cobol", ace_mode : "cobol", language_id :
    48u64, codemirror_mode : "cobol", codemirror_mime_type : "text/x-cobol", filenames :
    [], interpreters : [], }, CODEOWNERS => { name : "CODEOWNERS", r#type : "data", color
    : "#000000", extensions : [], aliases : [], tm_scope : "text.codeowners", ace_mode :
    "gitignore", language_id : 321684729u64, filenames : ["CODEOWNERS"], interpreters :
    [], }, COLLADA => { name : "COLLADA", r#type : "data", color : "#F1A42B", extensions
    : [".dae"], aliases : [], tm_scope : "text.xml", ace_mode : "xml", language_id :
    49u64, codemirror_mode : "xml", codemirror_mime_type : "text/xml", filenames : [],
    interpreters : [], }, CSON => { name : "CSON", r#type : "data", color : "#244776",
    extensions : [".cson"], aliases : [], tm_scope : "source.coffee", ace_mode :
    "coffee", language_id : 424u64, codemirror_mode : "coffeescript",
    codemirror_mime_type : "text/x-coffeescript", filenames : [], interpreters : [], },
    CSS => { name : "CSS", r#type : "markup", color : "#663399", extensions : [".css"],
    aliases : [], tm_scope : "source.css", ace_mode : "css", language_id : 50u64,
    codemirror_mode : "css", codemirror_mime_type : "text/css", filenames : [],
    interpreters : [], }, CSV => { name : "CSV", r#type : "data", color : "#237346",
    extensions : [".csv"], aliases : [], tm_scope : "none", ace_mode : "csv", language_id
    : 51u64, filenames : [], interpreters : [], }, CUE => { name : "CUE", r#type :
    "programming", color : "#5886E1", extensions : [".cue"], aliases : [], tm_scope :
    "source.cue", ace_mode : "text", language_id : 356063509u64, filenames : [],
    interpreters : [], }, CWeb => { name : "CWeb", r#type : "programming", color :
    "#00007a", extensions : [".w"], aliases : [], tm_scope : "none", ace_mode : "text",
    language_id : 657332628u64, filenames : [], interpreters : [], }, CabalConfig => {
    name : "Cabal Config", r#type : "data", color : "#483465", extensions : [".cabal"],
    aliases : ["Cabal"], tm_scope : "source.cabal", ace_mode : "haskell_cabal",
    language_id : 677095381u64, codemirror_mode : "haskell", codemirror_mime_type :
    "text/x-haskell", filenames : ["cabal.config", "cabal.project"], interpreters : [],
    }, Caddyfile => { name : "Caddyfile", r#type : "data", color : "#22b638", extensions
    : [".caddyfile"], aliases : ["Caddy"], tm_scope : "source.Caddyfile", ace_mode :
    "text", language_id : 615465151u64, filenames : ["Caddyfile"], interpreters : [], },
    Cadence => { name : "Cadence", r#type : "programming", color : "#00ef8b", extensions
    : [".cdc"], aliases : [], tm_scope : "source.cadence", ace_mode : "text", language_id
    : 270184138u64, filenames : [], interpreters : [], }, Cairo => { name : "Cairo",
    r#type : "programming", color : "#ff4a48", extensions : [".cairo"], aliases : [],
    tm_scope : "source.cairo", ace_mode : "text", language_id : 620599567u64, filenames :
    [], group : "Cairo", interpreters : [], }, CairoZero => { name : "Cairo Zero", r#type
    : "programming", color : "#ff4a48", extensions : [".cairo"], aliases : [], tm_scope :
    "source.cairo0", ace_mode : "text", language_id : 891399890u64, filenames : [], group
    : "Cairo", interpreters : [], }, CameLIGO => { name : "CameLIGO", r#type :
    "programming", color : "#3be133", extensions : [".mligo"], aliases : [], tm_scope :
    "source.mligo", ace_mode : "ocaml", language_id : 829207807u64, codemirror_mode :
    "mllike", codemirror_mime_type : "text/x-ocaml", filenames : [], group : "LigoLANG",
    interpreters : [], }, Cangjie => { name : "Cangjie", r#type : "programming", color :
    "#00868B", extensions : [".cj"], aliases : [], tm_scope : "source.cj", ace_mode :
    "swift", language_id : 581895317u64, codemirror_mode : "swift", codemirror_mime_type
    : "text/x-swift", filenames : [], interpreters : [], }, CapnProto => { name :
    "Cap'n Proto", r#type : "programming", color : "#c42727", extensions : [".capnp"],
    aliases : [], tm_scope : "source.capnp", ace_mode : "text", language_id : 52u64,
    filenames : [], interpreters : [], }, Carbon => { name : "Carbon", r#type :
    "programming", color : "#222222", extensions : [".carbon"], aliases : [], tm_scope :
    "source.v", ace_mode : "golang", language_id : 55627273u64, codemirror_mode : "go",
    codemirror_mime_type : "text/x-go", filenames : [], interpreters : [], }, CartoCSS =>
    { name : "CartoCSS", r#type : "programming", color : "#000000", extensions :
    [".mss"], aliases : ["Carto"], tm_scope : "source.css.mss", ace_mode : "text",
    language_id : 53u64, filenames : [], interpreters : [], }, Ceylon => { name :
    "Ceylon", r#type : "programming", color : "#dfa535", extensions : [".ceylon"],
    aliases : [], tm_scope : "source.ceylon", ace_mode : "text", language_id : 54u64,
    filenames : [], interpreters : [], }, Chapel => { name : "Chapel", r#type :
    "programming", color : "#8dc63f", extensions : [".chpl"], aliases : ["chpl"],
    tm_scope : "source.chapel", ace_mode : "text", language_id : 55u64, filenames : [],
    interpreters : [], }, Charity => { name : "Charity", r#type : "programming", color :
    "#000000", extensions : [".ch"], aliases : [], tm_scope : "none", ace_mode : "text",
    language_id : 56u64, filenames : [], interpreters : [], }, Checksums => { name :
    "Checksums", r#type : "data", color : "#000000", extensions : [".crc32", ".md2",
    ".md4", ".md5", ".sha1", ".sha2", ".sha224", ".sha256", ".sha256sum", ".sha3",
    ".sha384", ".sha512"], aliases : ["checksum", "hash", "hashes", "sum", "sums"],
    tm_scope : "text.checksums", ace_mode : "text", language_id : 372063053u64, filenames
    : ["MD5SUMS", "SHA1SUMS", "SHA256SUMS", "SHA256SUMS.txt", "SHA512SUMS",
    "checksums.txt", "cksums", "md5sum.txt"], interpreters : [], }, ChucK => { name :
    "ChucK", r#type : "programming", color : "#3f8000", extensions : [".ck"], aliases :
    [], tm_scope : "source.java", ace_mode : "java", language_id : 57u64, codemirror_mode
    : "clike", codemirror_mime_type : "text/x-java", filenames : [], interpreters : [],
    }, Circom => { name : "Circom", r#type : "programming", color : "#707575", extensions
    : [".circom"], aliases : [], tm_scope : "source.circom", ace_mode : "text",
    language_id : 1042332086u64, filenames : [], interpreters : [], }, Cirru => { name :
    "Cirru", r#type : "programming", color : "#ccccff", extensions : [".cirru"], aliases
    : [], tm_scope : "source.cirru", ace_mode : "cirru", language_id : 58u64, filenames :
    [], interpreters : [], }, Clarion => { name : "Clarion", r#type : "programming",
    color : "#db901e", extensions : [".clw"], aliases : [], tm_scope : "source.clarion",
    ace_mode : "text", language_id : 59u64, filenames : [], interpreters : [], }, Clarity
    => { name : "Clarity", r#type : "programming", color : "#5546ff", extensions :
    [".clar"], aliases : [], tm_scope : "source.clar", ace_mode : "lisp", language_id :
    91493841u64, filenames : [], interpreters : [], }, ClassicASP => { name :
    "Classic ASP", r#type : "programming", color : "#6a40fd", extensions : [".asp"],
    aliases : ["asp"], tm_scope : "text.html.asp", ace_mode : "text", language_id : 8u64,
    filenames : [], interpreters : [], }, Clean => { name : "Clean", r#type :
    "programming", color : "#3F85AF", extensions : [".icl", ".dcl"], aliases : [],
    tm_scope : "source.clean", ace_mode : "text", language_id : 60u64, filenames : [],
    interpreters : [], }, Click => { name : "Click", r#type : "programming", color :
    "#E4E6F3", extensions : [".click"], aliases : [], tm_scope : "source.click", ace_mode
    : "text", language_id : 61u64, filenames : [], interpreters : [], }, Clojure => {
    name : "Clojure", r#type : "programming", color : "#db5855", extensions : [".clj",
    ".bb", ".boot", ".cl2", ".cljc", ".cljs", ".cljs.hl", ".cljscm", ".cljx", ".hic"],
    aliases : [], tm_scope : "source.clojure", ace_mode : "clojure", language_id : 62u64,
    codemirror_mode : "clojure", codemirror_mime_type : "text/x-clojure", filenames :
    ["riemann.config"], interpreters : ["bb"], }, ClosureTemplates => { name :
    "Closure Templates", r#type : "markup", color : "#0d948f", extensions : [".soy"],
    aliases : ["soy"], tm_scope : "text.html.soy", ace_mode : "soy_template", language_id
    : 357046146u64, codemirror_mode : "soy", codemirror_mime_type : "text/x-soy",
    filenames : [], interpreters : [], }, CloudFirestoreSecurityRules => { name :
    "Cloud Firestore Security Rules", r#type : "data", color : "#FFA000", extensions :
    [], aliases : [], tm_scope : "source.firestore", ace_mode : "less", language_id :
    407996372u64, codemirror_mode : "css", codemirror_mime_type : "text/css", filenames :
    ["firestore.rules"], interpreters : [], }, Clue => { name : "Clue", r#type :
    "programming", color : "#0009b5", extensions : [".clue"], aliases : [], tm_scope :
    "source.clue", ace_mode : "text", language_id : 163763508u64, filenames : [],
    interpreters : [], }, CoNLLU => { name : "CoNLL-U", r#type : "data", color :
    "#000000", extensions : [".conllu", ".conll"], aliases : ["CoNLL", "CoNLL-X"],
    tm_scope : "text.conllu", ace_mode : "text", language_id : 421026389u64, filenames :
    [], interpreters : [], }, CodeQL => { name : "CodeQL", r#type : "programming", color
    : "#140f46", extensions : [".ql", ".qll"], aliases : ["ql"], tm_scope : "source.ql",
    ace_mode : "text", language_id : 424259634u64, filenames : [], interpreters : [], },
    CoffeeScript => { name : "CoffeeScript", r#type : "programming", color : "#244776",
    extensions : [".coffee", "._coffee", ".cake", ".cjsx", ".iced"], aliases : ["coffee",
    "coffee-script"], tm_scope : "source.coffee", ace_mode : "coffee", language_id :
    63u64, codemirror_mode : "coffeescript", codemirror_mime_type :
    "text/x-coffeescript", filenames : ["Cakefile"], interpreters : ["coffee"], },
    ColdFusion => { name : "ColdFusion", r#type : "programming", color : "#ed2cd6",
    extensions : [".cfm", ".cfml"], aliases : ["cfm", "cfml", "coldfusion html"],
    tm_scope : "text.html.cfm", ace_mode : "coldfusion", language_id : 64u64, filenames :
    [], interpreters : [], }, ColdFusionCFC => { name : "ColdFusion CFC", r#type :
    "programming", color : "#ed2cd6", extensions : [".cfc"], aliases : ["cfc"], tm_scope
    : "source.cfscript", ace_mode : "coldfusion", language_id : 65u64, filenames : [],
    group : "ColdFusion", interpreters : [], }, CommonLisp => { name : "Common Lisp",
    r#type : "programming", color : "#3fb68b", extensions : [".lisp", ".asd", ".cl",
    ".l", ".lsp", ".ny", ".podsl", ".sexp"], aliases : ["lisp"], tm_scope :
    "source.commonlisp", ace_mode : "lisp", language_id : 66u64, codemirror_mode :
    "commonlisp", codemirror_mime_type : "text/x-common-lisp", filenames : [],
    interpreters : ["lisp", "sbcl", "ccl", "clisp", "ecl"], }, CommonWorkflowLanguage =>
    { name : "Common Workflow Language", r#type : "programming", color : "#B5314C",
    extensions : [".cwl"], aliases : ["cwl"], tm_scope : "source.cwl", ace_mode : "yaml",
    language_id : 988547172u64, codemirror_mode : "yaml", codemirror_mime_type :
    "text/x-yaml", filenames : [], interpreters : ["cwl-runner"], }, ComponentPascal => {
    name : "Component Pascal", r#type : "programming", color : "#B0CE4E", extensions :
    [".cp", ".cps"], aliases : [], tm_scope : "source.pascal", ace_mode : "pascal",
    language_id : 67u64, codemirror_mode : "pascal", codemirror_mime_type :
    "text/x-pascal", filenames : [], interpreters : [], }, Cooklang => { name :
    "Cooklang", r#type : "markup", color : "#E15A29", extensions : [".cook"], aliases :
    [], tm_scope : "source.cooklang", ace_mode : "text", language_id : 788037493u64, wrap
    : true, filenames : [], interpreters : [], }, Cool => { name : "Cool", r#type :
    "programming", color : "#000000", extensions : [".cl"], aliases : [], tm_scope :
    "source.cool", ace_mode : "text", language_id : 68u64, filenames : [], interpreters :
    [], }, CppObjDump => { name : "Cpp-ObjDump", r#type : "data", color : "#000000",
    extensions : [".cppobjdump", ".c++-objdump", ".c++objdump", ".cpp-objdump",
    ".cxx-objdump"], aliases : ["c++-objdump"], tm_scope : "objdump.x86asm", ace_mode :
    "assembly_x86", language_id : 70u64, filenames : [], interpreters : [], }, Creole =>
    { name : "Creole", r#type : "prose", color : "#000000", extensions : [".creole"],
    aliases : [], tm_scope : "text.html.creole", ace_mode : "text", language_id : 71u64,
    wrap : true, filenames : [], interpreters : [], }, Crystal => { name : "Crystal",
    r#type : "programming", color : "#000100", extensions : [".cr"], aliases : [],
    tm_scope : "source.crystal", ace_mode : "crystal", language_id : 72u64,
    codemirror_mode : "crystal", codemirror_mime_type : "text/x-crystal", filenames : [],
    interpreters : ["crystal"], }, Csound => { name : "Csound", r#type : "programming",
    color : "#1a1a1a", extensions : [".orc", ".udo"], aliases : ["csound-orc"], tm_scope
    : "source.csound", ace_mode : "csound_orchestra", language_id : 73u64, filenames :
    [], interpreters : [], }, CsoundDocument => { name : "Csound Document", r#type :
    "programming", color : "#1a1a1a", extensions : [".csd"], aliases : ["csound-csd"],
    tm_scope : "source.csound-document", ace_mode : "csound_document", language_id :
    74u64, filenames : [], interpreters : [], }, CsoundScore => { name : "Csound Score",
    r#type : "programming", color : "#1a1a1a", extensions : [".sco"], aliases :
    ["csound-sco"], tm_scope : "source.csound-score", ace_mode : "csound_score",
    language_id : 75u64, filenames : [], interpreters : [], }, Cuda => { name : "Cuda",
    r#type : "programming", color : "#3A4E3A", extensions : [".cu", ".cuh"], aliases :
    [], tm_scope : "source.cuda-c++", ace_mode : "c_cpp", language_id : 77u64,
    codemirror_mode : "clike", codemirror_mime_type : "text/x-c++src", filenames : [],
    interpreters : [], }, CueSheet => { name : "Cue Sheet", r#type : "data", color :
    "#000000", extensions : [".cue"], aliases : [], tm_scope : "source.cuesheet",
    ace_mode : "text", language_id : 942714150u64, filenames : [], interpreters : [], },
    Curry => { name : "Curry", r#type : "programming", color : "#531242", extensions :
    [".curry"], aliases : [], tm_scope : "source.curry", ace_mode : "haskell",
    language_id : 439829048u64, filenames : [], interpreters : [], }, Cycript => { name :
    "Cycript", r#type : "programming", color : "#000000", extensions : [".cy"], aliases :
    [], tm_scope : "source.js", ace_mode : "javascript", language_id : 78u64,
    codemirror_mode : "javascript", codemirror_mime_type : "text/javascript", filenames :
    [], interpreters : [], }, Cylc => { name : "Cylc", r#type : "data", color :
    "#00b3fd", extensions : [".cylc"], aliases : [], tm_scope : "source.cylc", ace_mode :
    "ini", language_id : 476447814u64, filenames : ["suite.rc"], group : "INI",
    interpreters : [], }, Cypher => { name : "Cypher", r#type : "programming", color :
    "#34c0eb", extensions : [".cyp", ".cypher"], aliases : [], tm_scope :
    "source.cypher", ace_mode : "text", language_id : 850806976u64, codemirror_mode :
    "cypher", codemirror_mime_type : "application/x-cypher-query", filenames : [],
    interpreters : [], }, Cython => { name : "Cython", r#type : "programming", color :
    "#fedf5b", extensions : [".pyx", ".pxd", ".pxi"], aliases : ["pyrex"], tm_scope :
    "source.cython", ace_mode : "text", language_id : 79u64, codemirror_mode : "python",
    codemirror_mime_type : "text/x-cython", filenames : [], interpreters : [], }, D => {
    name : "D", r#type : "programming", color : "#ba595e", extensions : [".d", ".di"],
    aliases : ["Dlang"], tm_scope : "source.d", ace_mode : "d", language_id : 80u64,
    codemirror_mode : "d", codemirror_mime_type : "text/x-d", filenames : [],
    interpreters : [], }, DObjDump => { name : "D-ObjDump", r#type : "data", color :
    "#000000", extensions : [".d-objdump"], aliases : [], tm_scope : "objdump.x86asm",
    ace_mode : "assembly_x86", language_id : 81u64, filenames : [], interpreters : [], },
    D2 => { name : "D2", r#type : "markup", color : "#526ee8", extensions : [".d2"],
    aliases : ["d2lang"], tm_scope : "source.d2", ace_mode : "text", language_id :
    37531557u64, filenames : [], interpreters : [], }, DIGITALCommandLanguage => { name :
    "DIGITAL Command Language", r#type : "programming", color : "#000000", extensions :
    [".com"], aliases : ["dcl"], tm_scope : "none", ace_mode : "text", language_id :
    82u64, filenames : [], interpreters : [], }, DM => { name : "DM", r#type :
    "programming", color : "#447265", extensions : [".dm"], aliases : ["byond"], tm_scope
    : "source.dm", ace_mode : "c_cpp", language_id : 83u64, filenames : [], interpreters
    : [], }, DNSZone => { name : "DNS Zone", r#type : "data", color : "#000000",
    extensions : [".zone", ".arpa"], aliases : [], tm_scope : "text.zone_file", ace_mode
    : "text", language_id : 84u64, filenames : [], interpreters : [], }, DTrace => { name
    : "DTrace", r#type : "programming", color : "#000000", extensions : [".d"], aliases :
    ["dtrace-script"], tm_scope : "source.c", ace_mode : "c_cpp", language_id : 85u64,
    codemirror_mode : "clike", codemirror_mime_type : "text/x-csrc", filenames : [],
    interpreters : ["dtrace"], }, Dafny => { name : "Dafny", r#type : "programming",
    color : "#FFEC25", extensions : [".dfy"], aliases : [], tm_scope : "text.dfy.dafny",
    ace_mode : "text", language_id : 969323346u64, filenames : [], interpreters :
    ["dafny"], }, DarcsPatch => { name : "Darcs Patch", r#type : "data", color :
    "#8eff23", extensions : [".darcspatch", ".dpatch"], aliases : ["dpatch"], tm_scope :
    "none", ace_mode : "text", language_id : 86u64, filenames : [], interpreters : [], },
    Dart => { name : "Dart", r#type : "programming", color : "#00B4AB", extensions :
    [".dart"], aliases : [], tm_scope : "source.dart", ace_mode : "dart", language_id :
    87u64, codemirror_mode : "dart", codemirror_mime_type : "application/dart", filenames
    : [], interpreters : ["dart"], }, Daslang => { name : "Daslang", r#type :
    "programming", color : "#d3d3d3", extensions : [".das"], aliases : [], tm_scope :
    "source.daslang", ace_mode : "text", language_id : 648759486u64, filenames : [],
    interpreters : [], }, DataWeave => { name : "DataWeave", r#type : "programming",
    color : "#003a52", extensions : [".dwl"], aliases : [], tm_scope :
    "source.data-weave", ace_mode : "text", language_id : 974514097u64, filenames : [],
    interpreters : [], }, DebianPackageControlFile => { name :
    "Debian Package Control File", r#type : "data", color : "#D70751", extensions :
    [".dsc"], aliases : [], tm_scope : "source.deb-control", ace_mode : "text",
    language_id : 527438264u64, filenames : [], interpreters : [], }, DenizenScript => {
    name : "DenizenScript", r#type : "programming", color : "#FBEE96", extensions :
    [".dsc"], aliases : [], tm_scope : "source.denizenscript", ace_mode : "yaml",
    language_id : 435000929u64, codemirror_mode : "yaml", codemirror_mime_type :
    "text/x-yaml", filenames : [], interpreters : [], }, Dhall => { name : "Dhall",
    r#type : "programming", color : "#dfafff", extensions : [".dhall"], aliases : [],
    tm_scope : "source.haskell", ace_mode : "haskell", language_id : 793969321u64,
    codemirror_mode : "haskell", codemirror_mime_type : "text/x-haskell", filenames : [],
    interpreters : [], }, Diff => { name : "Diff", r#type : "data", color : "#000000",
    extensions : [".diff", ".patch"], aliases : ["udiff"], tm_scope : "source.diff",
    ace_mode : "diff", language_id : 88u64, codemirror_mode : "diff",
    codemirror_mime_type : "text/x-diff", filenames : [], interpreters : [], },
    DirectX3DFile => { name : "DirectX 3D File", r#type : "data", color : "#aace60",
    extensions : [".x"], aliases : [], tm_scope : "none", ace_mode : "text", language_id
    : 201049282u64, filenames : [], interpreters : [], }, Dockerfile => { name :
    "Dockerfile", r#type : "programming", color : "#384d54", extensions : [".dockerfile",
    ".containerfile"], aliases : ["Containerfile"], tm_scope : "source.dockerfile",
    ace_mode : "dockerfile", language_id : 89u64, codemirror_mode : "dockerfile",
    codemirror_mime_type : "text/x-dockerfile", filenames : ["Containerfile",
    "Dockerfile"], interpreters : [], }, Dogescript => { name : "Dogescript", r#type :
    "programming", color : "#cca760", extensions : [".djs"], aliases : [], tm_scope :
    "none", ace_mode : "text", language_id : 90u64, filenames : [], interpreters : [], },
    Dotenv => { name : "Dotenv", r#type : "data", color : "#e5d559", extensions :
    [".env"], aliases : [], tm_scope : "source.dotenv", ace_mode : "text", language_id :
    111148035u64, filenames : [".env", ".env.ci", ".env.dev", ".env.development",
    ".env.development.local", ".env.example", ".env.local", ".env.prod",
    ".env.production", ".env.sample", ".env.staging", ".env.template", ".env.test",
    ".env.testing"], interpreters : [], }, Dune => { name : "Dune", r#type :
    "programming", color : "#89421e", extensions : [], aliases : [], tm_scope :
    "source.dune", ace_mode : "lisp", language_id : 754574151u64, filenames :
    ["dune-project"], interpreters : [], }, Dylan => { name : "Dylan", r#type :
    "programming", color : "#6c616e", extensions : [".dylan", ".dyl", ".intr", ".lid"],
    aliases : [], tm_scope : "source.dylan", ace_mode : "text", language_id : 91u64,
    codemirror_mode : "dylan", codemirror_mime_type : "text/x-dylan", filenames : [],
    interpreters : [], }, E => { name : "E", r#type : "programming", color : "#ccce35",
    extensions : [".e"], aliases : [], tm_scope : "none", ace_mode : "text", language_id
    : 92u64, filenames : [], interpreters : ["rune"], }, EMail => { name : "E-mail",
    r#type : "data", color : "#000000", extensions : [".eml", ".mbox"], aliases :
    ["email", "eml", "mail", "mbox"], tm_scope : "text.eml.basic", ace_mode : "text",
    language_id : 529653389u64, codemirror_mode : "mbox", codemirror_mime_type :
    "application/mbox", filenames : [], interpreters : [], }, EBNF => { name : "EBNF",
    r#type : "data", color : "#000000", extensions : [".ebnf"], aliases : [], tm_scope :
    "source.ebnf", ace_mode : "text", language_id : 430u64, codemirror_mode : "ebnf",
    codemirror_mime_type : "text/x-ebnf", filenames : [], interpreters : [], }, ECL => {
    name : "ECL", r#type : "programming", color : "#8a1267", extensions : [".ecl",
    ".eclxml"], aliases : [], tm_scope : "source.ecl", ace_mode : "text", language_id :
    93u64, codemirror_mode : "ecl", codemirror_mime_type : "text/x-ecl", filenames : [],
    interpreters : [], }, ECLiPSe => { name : "ECLiPSe", r#type : "programming", color :
    "#001d9d", extensions : [".ecl"], aliases : [], tm_scope : "source.prolog.eclipse",
    ace_mode : "prolog", language_id : 94u64, filenames : [], group : "Prolog",
    interpreters : [], }, EJS => { name : "EJS", r#type : "markup", color : "#a91e50",
    extensions : [".ejs", ".ect", ".ejs.t", ".jst"], aliases : [], tm_scope :
    "text.html.js", ace_mode : "ejs", language_id : 95u64, codemirror_mode :
    "htmlembedded", codemirror_mime_type : "application/x-ejs", filenames : [],
    interpreters : [], }, EQ => { name : "EQ", r#type : "programming", color : "#a78649",
    extensions : [".eq"], aliases : [], tm_scope : "source.cs", ace_mode : "csharp",
    language_id : 96u64, codemirror_mode : "clike", codemirror_mime_type :
    "text/x-csharp", filenames : [], interpreters : [], }, Eagle => { name : "Eagle",
    r#type : "data", color : "#000000", extensions : [".sch", ".brd"], aliases : [],
    tm_scope : "text.xml", ace_mode : "xml", language_id : 97u64, codemirror_mode :
    "xml", codemirror_mime_type : "text/xml", filenames : [], interpreters : [], },
    Earthly => { name : "Earthly", r#type : "programming", color : "#2af0ff", extensions
    : [], aliases : ["Earthfile"], tm_scope : "source.earthfile", ace_mode : "text",
    language_id : 963512632u64, filenames : ["Earthfile"], interpreters : [], },
    Easybuild => { name : "Easybuild", r#type : "data", color : "#069406", extensions :
    [".eb"], aliases : [], tm_scope : "source.python", ace_mode : "python", language_id :
    342840477u64, codemirror_mode : "python", codemirror_mime_type : "text/x-python",
    filenames : [], group : "Python", interpreters : [], }, EcereProjects => { name :
    "Ecere Projects", r#type : "data", color : "#913960", extensions : [".epj"], aliases
    : [], tm_scope : "source.json", ace_mode : "json", language_id : 98u64,
    codemirror_mode : "javascript", codemirror_mime_type : "application/json", filenames
    : [], group : "JavaScript", interpreters : [], }, Ecmarkup => { name : "Ecmarkup",
    r#type : "markup", color : "#eb8131", extensions : [".html"], aliases :
    ["ecmarkdown"], tm_scope : "text.html.ecmarkup", ace_mode : "html", language_id :
    844766630u64, codemirror_mode : "htmlmixed", codemirror_mime_type : "text/html",
    filenames : [], group : "HTML", interpreters : [], }, Edge => { name : "Edge", r#type
    : "markup", color : "#0dffe0", extensions : [".edge"], aliases : [], tm_scope :
    "text.html.edge", ace_mode : "html", language_id : 460509620u64, filenames : [],
    interpreters : [], }, EdgeQL => { name : "EdgeQL", r#type : "programming", color :
    "#31A7FF", extensions : [".edgeql", ".esdl"], aliases : ["esdl"], tm_scope :
    "source.edgeql", ace_mode : "text", language_id : 925235833u64, filenames : [],
    interpreters : [], }, EditorConfig => { name : "EditorConfig", r#type : "data", color
    : "#fff1f2", extensions : [".editorconfig"], aliases : ["editor-config"], tm_scope :
    "source.editorconfig", ace_mode : "ini", language_id : 96139566u64, codemirror_mode :
    "properties", codemirror_mime_type : "text/x-properties", filenames :
    [".editorconfig"], group : "INI", interpreters : [], }, EdjeDataCollection => { name
    : "Edje Data Collection", r#type : "data", color : "#000000", extensions : [".edc"],
    aliases : [], tm_scope : "source.c++", ace_mode : "c_cpp", language_id :
    342840478u64, codemirror_mode : "clike", codemirror_mime_type : "text/x-c++src",
    filenames : [], interpreters : [], }, Eiffel => { name : "Eiffel", r#type :
    "programming", color : "#4d6977", extensions : [".e"], aliases : [], tm_scope :
    "source.eiffel", ace_mode : "eiffel", language_id : 99u64, codemirror_mode :
    "eiffel", codemirror_mime_type : "text/x-eiffel", filenames : [], interpreters : [],
    }, Elixir => { name : "Elixir", r#type : "programming", color : "#6e4a7e", extensions
    : [".ex", ".exs"], aliases : [], tm_scope : "source.elixir", ace_mode : "elixir",
    language_id : 100u64, filenames : ["mix.lock"], interpreters : ["elixir"], }, Elm =>
    { name : "Elm", r#type : "programming", color : "#60B5CC", extensions : [".elm"],
    aliases : [], tm_scope : "source.elm", ace_mode : "elm", language_id : 101u64,
    codemirror_mode : "elm", codemirror_mime_type : "text/x-elm", filenames : [],
    interpreters : [], }, Elvish => { name : "Elvish", r#type : "programming", color :
    "#55BB55", extensions : [".elv"], aliases : [], tm_scope : "source.elvish", ace_mode
    : "text", language_id : 570996448u64, filenames : [], interpreters : ["elvish"], },
    ElvishTranscript => { name : "Elvish Transcript", r#type : "programming", color :
    "#55BB55", extensions : [], aliases : [], tm_scope : "source.elvish-transcript",
    ace_mode : "text", language_id : 452025714u64, filenames : [], group : "Elvish",
    interpreters : [], }, EmacsLisp => { name : "Emacs Lisp", r#type : "programming",
    color : "#c065db", extensions : [".el", ".emacs", ".emacs.desktop"], aliases :
    ["elisp", "emacs"], tm_scope : "source.emacs.lisp", ace_mode : "lisp", language_id :
    102u64, codemirror_mode : "commonlisp", codemirror_mime_type : "text/x-common-lisp",
    filenames : [".abbrev_defs", ".emacs", ".emacs.desktop", ".gnus", ".spacemacs",
    ".viper", "Cask", "Project.ede", "_emacs", "abbrev_defs"], interpreters : [], },
    EmberScript => { name : "EmberScript", r#type : "programming", color : "#FFF4F3",
    extensions : [".em", ".emberscript"], aliases : [], tm_scope : "source.coffee",
    ace_mode : "coffee", language_id : 103u64, codemirror_mode : "coffeescript",
    codemirror_mime_type : "text/x-coffeescript", filenames : [], interpreters : [], },
    Erlang => { name : "Erlang", r#type : "programming", color : "#B83998", extensions :
    [".erl", ".app", ".app.src", ".es", ".escript", ".hrl", ".xrl", ".yrl"], aliases :
    [], tm_scope : "source.erlang", ace_mode : "erlang", language_id : 104u64,
    codemirror_mode : "erlang", codemirror_mime_type : "text/x-erlang", filenames :
    ["Emakefile", "rebar.config", "rebar.config.lock", "rebar.lock"], interpreters :
    ["escript"], }, Euphoria => { name : "Euphoria", r#type : "programming", color :
    "#FF790B", extensions : [".e", ".ex"], aliases : [], tm_scope : "source.euphoria",
    ace_mode : "text", language_id : 880693982u64, filenames : [], interpreters : ["eui",
    "euiw"], }, Fsharp => { name : "F#", r#type : "programming", color : "#b845fc",
    extensions : [".fs", ".fsi", ".fsx"], aliases : ["fsharp"], tm_scope :
    "source.fsharp", ace_mode : "fsharp", language_id : 105u64, codemirror_mode :
    "mllike", codemirror_mime_type : "text/x-fsharp", filenames : [], interpreters : [],
    }, Fstar => { name : "F*", r#type : "programming", color : "#572e30", extensions :
    [".fst", ".fsti"], aliases : ["fstar"], tm_scope : "source.fstar", ace_mode : "text",
    language_id : 336943375u64, filenames : [], interpreters : [], fs_name : "Fstar", },
    FIGletFont => { name : "FIGlet Font", r#type : "data", color : "#FFDDBB", extensions
    : [".flf"], aliases : ["FIGfont"], tm_scope : "source.figfont", ace_mode : "text",
    language_id : 686129783u64, filenames : [], interpreters : [], }, FIRRTL => { name :
    "FIRRTL", r#type : "programming", color : "#2f632f", extensions : [".fir"], aliases :
    [], tm_scope : "source.firrtl", ace_mode : "text", language_id : 906694254u64,
    filenames : [], interpreters : [], }, FLUX => { name : "FLUX", r#type :
    "programming", color : "#88ccff", extensions : [".fx", ".flux"], aliases : [],
    tm_scope : "none", ace_mode : "text", language_id : 106u64, filenames : [],
    interpreters : [], }, Factor => { name : "Factor", r#type : "programming", color :
    "#636746", extensions : [".factor"], aliases : [], tm_scope : "source.factor",
    ace_mode : "text", language_id : 108u64, codemirror_mode : "factor",
    codemirror_mime_type : "text/x-factor", filenames : [".factor-boot-rc",
    ".factor-rc"], interpreters : [], }, Fancy => { name : "Fancy", r#type :
    "programming", color : "#7b9db4", extensions : [".fy", ".fancypack"], aliases : [],
    tm_scope : "source.fancy", ace_mode : "text", language_id : 109u64, filenames :
    ["Fakefile"], interpreters : [], }, Fantom => { name : "Fantom", r#type :
    "programming", color : "#14253c", extensions : [".fan"], aliases : [], tm_scope :
    "source.fan", ace_mode : "text", language_id : 110u64, filenames : [], interpreters :
    [], }, Faust => { name : "Faust", r#type : "programming", color : "#c37240",
    extensions : [".dsp"], aliases : [], tm_scope : "source.faust", ace_mode : "text",
    language_id : 622529198u64, filenames : [], interpreters : [], }, Fennel => { name :
    "Fennel", r#type : "programming", color : "#fff3d7", extensions : [".fnl"], aliases :
    [], tm_scope : "source.fnl", ace_mode : "text", language_id : 239946126u64, filenames
    : [], interpreters : ["fennel"], }, FilebenchWML => { name : "Filebench WML", r#type
    : "programming", color : "#F6B900", extensions : [".f"], aliases : [], tm_scope :
    "none", ace_mode : "text", language_id : 111u64, filenames : [], interpreters : [],
    }, Filterscript => { name : "Filterscript", r#type : "programming", color :
    "#000000", extensions : [".fs"], aliases : [], tm_scope : "none", ace_mode : "text",
    language_id : 112u64, filenames : [], group : "RenderScript", interpreters : [], },
    Flix => { name : "Flix", r#type : "programming", color : "#d44a45", extensions :
    [".flix"], aliases : [], tm_scope : "source.flix", ace_mode : "flix", language_id :
    800935960u64, filenames : [], interpreters : [], }, Fluent => { name : "Fluent",
    r#type : "programming", color : "#ffcc33", extensions : [".ftl"], aliases : [],
    tm_scope : "source.ftl", ace_mode : "text", language_id : 206353404u64, filenames :
    [], interpreters : [], }, Formatted => { name : "Formatted", r#type : "data", color :
    "#000000", extensions : [".for", ".eam.fs"], aliases : [], tm_scope : "none",
    ace_mode : "text", language_id : 113u64, filenames : [], interpreters : [], }, Forth
    => { name : "Forth", r#type : "programming", color : "#341708", extensions : [".fth",
    ".4th", ".f", ".for", ".forth", ".fr", ".frt", ".fs"], aliases : [], tm_scope :
    "source.forth", ace_mode : "forth", language_id : 114u64, codemirror_mode : "forth",
    codemirror_mime_type : "text/x-forth", filenames : [], interpreters : [], }, Fortran
    => { name : "Fortran", r#type : "programming", color : "#4d41b1", extensions : [".f",
    ".f77", ".for", ".fpp"], aliases : [], tm_scope : "source.fortran", ace_mode :
    "fortran", language_id : 107u64, codemirror_mode : "fortran", codemirror_mime_type :
    "text/x-fortran", filenames : [], group : "Fortran", interpreters : [], },
    FortranFreeForm => { name : "Fortran Free Form", r#type : "programming", color :
    "#4d41b1", extensions : [".f90", ".f03", ".f08", ".f95"], aliases : [], tm_scope :
    "source.fortran.modern", ace_mode : "fortran", language_id : 761352333u64,
    codemirror_mode : "fortran", codemirror_mime_type : "text/x-fortran", filenames : [],
    group : "Fortran", interpreters : [], }, FreeBASIC => { name : "FreeBASIC", r#type :
    "programming", color : "#141AC9", extensions : [".bi", ".bas"], aliases : ["fb"],
    tm_scope : "source.vbnet", ace_mode : "text", language_id : 472896659u64,
    codemirror_mode : "vb", codemirror_mime_type : "text/x-vb", filenames : [],
    interpreters : [], }, FreeMarker => { name : "FreeMarker", r#type : "programming",
    color : "#0050b2", extensions : [".ftl", ".ftlh"], aliases : ["ftl"], tm_scope :
    "text.html.ftl", ace_mode : "ftl", language_id : 115u64, filenames : [], interpreters
    : [], }, Frege => { name : "Frege", r#type : "programming", color : "#00cafe",
    extensions : [".fr"], aliases : [], tm_scope : "source.haskell", ace_mode :
    "haskell", language_id : 116u64, filenames : [], interpreters : [], }, Futhark => {
    name : "Futhark", r#type : "programming", color : "#5f021f", extensions : [".fut"],
    aliases : [], tm_scope : "source.futhark", ace_mode : "text", language_id :
    97358117u64, filenames : [], interpreters : [], }, GCode => { name : "G-code", r#type
    : "programming", color : "#D08CF2", extensions : [".g", ".cnc", ".gco", ".gcode"],
    aliases : [], tm_scope : "source.gcode", ace_mode : "gcode", language_id : 117u64,
    filenames : [], interpreters : [], }, GAML => { name : "GAML", r#type :
    "programming", color : "#FFC766", extensions : [".gaml"], aliases : [], tm_scope :
    "none", ace_mode : "text", language_id : 290345951u64, filenames : [], interpreters :
    [], }, GAMS => { name : "GAMS", r#type : "programming", color : "#f49a22", extensions
    : [".gms"], aliases : [], tm_scope : "none", ace_mode : "text", language_id : 118u64,
    filenames : [], interpreters : [], }, GAP => { name : "GAP", r#type : "programming",
    color : "#0000cc", extensions : [".g", ".gap", ".gd", ".gi", ".tst"], aliases : [],
    tm_scope : "source.gap", ace_mode : "text", language_id : 119u64, filenames : [],
    interpreters : [], }, GCCMachineDescription => { name : "GCC Machine Description",
    r#type : "programming", color : "#FFCFAB", extensions : [".md"], aliases : [],
    tm_scope : "source.lisp", ace_mode : "lisp", language_id : 121u64, codemirror_mode :
    "commonlisp", codemirror_mime_type : "text/x-common-lisp", filenames : [],
    interpreters : [], }, GDB => { name : "GDB", r#type : "programming", color :
    "#000000", extensions : [".gdb", ".gdbinit"], aliases : [], tm_scope : "source.gdb",
    ace_mode : "text", language_id : 122u64, filenames : [], interpreters : [], },
    GDScript => { name : "GDScript", r#type : "programming", color : "#355570",
    extensions : [".gd"], aliases : [], tm_scope : "source.gdscript", ace_mode : "text",
    language_id : 123u64, filenames : [], interpreters : [], }, GDShader => { name :
    "GDShader", r#type : "programming", color : "#478CBF", extensions : [".gdshader",
    ".gdshaderinc"], aliases : [], tm_scope : "source.gdshader", ace_mode : "glsl",
    language_id : 694638086u64, filenames : [], interpreters : [], }, GEDCOM => { name :
    "GEDCOM", r#type : "data", color : "#003058", extensions : [".ged"], aliases : [],
    tm_scope : "source.gedcom", ace_mode : "text", language_id : 459577965u64, filenames
    : [], interpreters : [], }, GLSL => { name : "GLSL", r#type : "programming", color :
    "#5686a5", extensions : [".glsl", ".fp", ".frag", ".frg", ".fs", ".fsh", ".fshader",
    ".geo", ".geom", ".glslf", ".glslv", ".gs", ".gshader", ".rchit", ".rmiss",
    ".shader", ".tesc", ".tese", ".vert", ".vrx", ".vs", ".vsh", ".vshader"], aliases :
    [], tm_scope : "source.glsl", ace_mode : "glsl", language_id : 124u64, filenames :
    [], interpreters : [], }, GN => { name : "GN", r#type : "data", color : "#000000",
    extensions : [".gn", ".gni"], aliases : [], tm_scope : "source.gn", ace_mode :
    "python", language_id : 302957008u64, codemirror_mode : "python",
    codemirror_mime_type : "text/x-python", filenames : [".gn"], interpreters : ["gn"],
    }, GSC => { name : "GSC", r#type : "programming", color : "#FF6800", extensions :
    [".gsc", ".csc", ".gsh"], aliases : [], tm_scope : "source.gsc", ace_mode : "c_cpp",
    language_id : 257856279u64, codemirror_mode : "clike", codemirror_mime_type :
    "text/x-csrc", filenames : [], interpreters : [], }, GameMakerLanguage => { name :
    "Game Maker Language", r#type : "programming", color : "#71b417", extensions :
    [".gml"], aliases : [], tm_scope : "source.c++", ace_mode : "c_cpp", language_id :
    125u64, codemirror_mode : "clike", codemirror_mime_type : "text/x-c++src", filenames
    : [], interpreters : [], }, Gemfilelock => { name : "Gemfile.lock", r#type : "data",
    color : "#701516", extensions : [], aliases : [], tm_scope : "source.gemfile-lock",
    ace_mode : "text", language_id : 907065713u64, filenames : ["Gemfile.lock"],
    interpreters : [], searchable : false, }, Gemini => { name : "Gemini", r#type :
    "prose", color : "#ff6900", extensions : [".gmi"], aliases : ["gemtext"], tm_scope :
    "source.gemini", ace_mode : "text", language_id : 310828396u64, wrap : true,
    filenames : [], interpreters : [], }, Genero4gl => { name : "Genero 4gl", r#type :
    "programming", color : "#63408e", extensions : [".4gl"], aliases : [], tm_scope :
    "source.genero-4gl", ace_mode : "text", language_id : 986054050u64, filenames : [],
    interpreters : [], }, GeneroPer => { name : "Genero per", r#type : "markup", color :
    "#d8df39", extensions : [".per"], aliases : [], tm_scope : "source.genero-per",
    ace_mode : "text", language_id : 902995658u64, filenames : [], interpreters : [], },
    Genie => { name : "Genie", r#type : "programming", color : "#fb855d", extensions :
    [".gs"], aliases : [], tm_scope : "none", ace_mode : "text", language_id :
    792408528u64, filenames : [], interpreters : [], }, Genshi => { name : "Genshi",
    r#type : "programming", color : "#951531", extensions : [".kid"], aliases :
    ["xml+genshi", "xml+kid"], tm_scope : "text.xml.genshi", ace_mode : "xml",
    language_id : 126u64, codemirror_mode : "xml", codemirror_mime_type : "text/xml",
    filenames : [], interpreters : [], }, GentooEbuild => { name : "Gentoo Ebuild",
    r#type : "programming", color : "#9400ff", extensions : [".ebuild"], aliases : [],
    tm_scope : "source.shell", ace_mode : "sh", language_id : 127u64, codemirror_mode :
    "shell", codemirror_mime_type : "text/x-sh", filenames : [], group : "Shell",
    interpreters : [], }, GentooEclass => { name : "Gentoo Eclass", r#type :
    "programming", color : "#9400ff", extensions : [".eclass"], aliases : [], tm_scope :
    "source.shell", ace_mode : "sh", language_id : 128u64, codemirror_mode : "shell",
    codemirror_mime_type : "text/x-sh", filenames : [], group : "Shell", interpreters :
    [], }, GerberImage => { name : "Gerber Image", r#type : "data", color : "#d20b00",
    extensions : [".gbr", ".cmp", ".gbl", ".gbo", ".gbp", ".gbs", ".gko", ".gml", ".gpb",
    ".gpt", ".gtl", ".gto", ".gtp", ".gts", ".ncl", ".sol"], aliases : ["rs-274x"],
    tm_scope : "source.gerber", ace_mode : "text", language_id : 404627610u64, filenames
    : [], interpreters : ["gerbv", "gerbview"], }, GettextCatalog => { name :
    "Gettext Catalog", r#type : "prose", color : "#000000", extensions : [".po", ".pot"],
    aliases : ["pot"], tm_scope : "source.po", ace_mode : "text", language_id : 129u64,
    filenames : [], interpreters : [], }, Gherkin => { name : "Gherkin", r#type :
    "programming", color : "#5B2063", extensions : [".feature", ".story"], aliases :
    ["cucumber"], tm_scope : "text.gherkin.feature", ace_mode : "gherkin", language_id :
    76u64, codemirror_mode : "gherkin", codemirror_mime_type : "text/x-feature",
    filenames : [], interpreters : [], }, GitAttributes => { name : "Git Attributes",
    r#type : "data", color : "#F44D27", extensions : [], aliases : ["gitattributes"],
    tm_scope : "source.gitattributes", ace_mode : "gitignore", language_id :
    956324166u64, codemirror_mode : "shell", codemirror_mime_type : "text/x-sh",
    filenames : [".gitattributes"], interpreters : [], }, GitCommit => { name :
    "Git Commit", r#type : "data", color : "#F44D27", extensions : [], aliases :
    ["commit"], tm_scope : "text.git-commit", ace_mode : "text", language_id :
    131750475u64, wrap : true, filenames : ["COMMIT_EDITMSG"], interpreters : [], },
    GitConfig => { name : "Git Config", r#type : "data", color : "#F44D27", extensions :
    [".gitconfig"], aliases : ["gitconfig", "gitmodules"], tm_scope : "source.gitconfig",
    ace_mode : "ini", language_id : 807968997u64, codemirror_mode : "properties",
    codemirror_mime_type : "text/x-properties", filenames : [".gitconfig",
    ".gitmodules"], group : "INI", interpreters : [], }, GitRevisionList => { name :
    "Git Revision List", r#type : "data", color : "#F44D27", extensions : [], aliases :
    ["Git Blame Ignore Revs"], tm_scope : "source.git-revlist", ace_mode : "text",
    language_id : 461881235u64, filenames : [".git-blame-ignore-revs"], interpreters :
    [], }, Gleam => { name : "Gleam", r#type : "programming", color : "#ffaff3",
    extensions : [".gleam"], aliases : [], tm_scope : "source.gleam", ace_mode : "text",
    language_id : 1054258749u64, filenames : [], interpreters : [], }, GlimmerJS => {
    name : "Glimmer JS", r#type : "programming", color : "#F5835F", extensions :
    [".gjs"], aliases : [], tm_scope : "source.gjs", ace_mode : "javascript", language_id
    : 5523150u64, filenames : [], group : "JavaScript", interpreters : [], }, GlimmerTS
    => { name : "Glimmer TS", r#type : "programming", color : "#3178c6", extensions :
    [".gts"], aliases : [], tm_scope : "source.gts", ace_mode : "typescript", language_id
    : 95110458u64, filenames : [], group : "TypeScript", interpreters : [], }, Glyph => {
    name : "Glyph", r#type : "programming", color : "#c1ac7f", extensions : [".glf"],
    aliases : [], tm_scope : "source.tcl", ace_mode : "tcl", language_id : 130u64,
    codemirror_mode : "tcl", codemirror_mime_type : "text/x-tcl", filenames : [],
    interpreters : [], }, GlyphBitmapDistributionFormat => { name :
    "Glyph Bitmap Distribution Format", r#type : "data", color : "#000000", extensions :
    [".bdf"], aliases : [], tm_scope : "source.bdf", ace_mode : "text", language_id :
    997665271u64, filenames : [], interpreters : [], }, Gnuplot => { name : "Gnuplot",
    r#type : "programming", color : "#f0a9f0", extensions : [".gp", ".gnu", ".gnuplot",
    ".p", ".plot", ".plt"], aliases : [], tm_scope : "source.gnuplot", ace_mode : "text",
    language_id : 131u64, filenames : [], interpreters : ["gnuplot"], }, Go => { name :
    "Go", r#type : "programming", color : "#00ADD8", extensions : [".go"], aliases :
    ["golang"], tm_scope : "source.go", ace_mode : "golang", language_id : 132u64,
    codemirror_mode : "go", codemirror_mime_type : "text/x-go", filenames : [],
    interpreters : [], }, GoChecksums => { name : "Go Checksums", r#type : "data", color
    : "#00ADD8", extensions : [], aliases : ["go.sum", "go sum", "go.work.sum",
    "go work sum"], tm_scope : "go.sum", ace_mode : "text", language_id : 1054391671u64,
    filenames : ["go.sum", "go.work.sum"], interpreters : [], }, GoModule => { name :
    "Go Module", r#type : "data", color : "#00ADD8", extensions : [], aliases :
    ["go.mod", "go mod"], tm_scope : "go.mod", ace_mode : "text", language_id :
    947461016u64, filenames : ["go.mod"], interpreters : [], }, GoTemplate => { name :
    "Go Template", r#type : "markup", color : "#00ADD8", extensions : [".gohtml",
    ".gotmpl", ".html.tmpl", ".tmpl", ".tpl"], aliases : ["gotmpl"], tm_scope :
    "source.go-template", ace_mode : "text", language_id : 247918769u64, filenames :
    ["_helpers.tpl"], interpreters : [], }, GoWorkspace => { name : "Go Workspace",
    r#type : "data", color : "#00ADD8", extensions : [], aliases : ["go.work",
    "go work"], tm_scope : "go.mod", ace_mode : "text", language_id : 934546256u64,
    filenames : ["go.work"], interpreters : [], }, GodotResource => { name :
    "Godot Resource", r#type : "data", color : "#355570", extensions : [".gdnlib",
    ".gdns", ".tres", ".tscn"], aliases : [], tm_scope : "source.gdresource", ace_mode :
    "text", language_id : 738107771u64, filenames : ["project.godot"], interpreters : [],
    }, Golo => { name : "Golo", r#type : "programming", color : "#88562A", extensions :
    [".golo"], aliases : [], tm_scope : "source.golo", ace_mode : "text", language_id :
    133u64, filenames : [], interpreters : [], }, Gosu => { name : "Gosu", r#type :
    "programming", color : "#82937f", extensions : [".gs", ".gst", ".gsx", ".vark"],
    aliases : [], tm_scope : "source.gosu.2", ace_mode : "text", language_id : 134u64,
    filenames : [], interpreters : [], }, Grace => { name : "Grace", r#type :
    "programming", color : "#615f8b", extensions : [".grace"], aliases : [], tm_scope :
    "source.grace", ace_mode : "text", language_id : 135u64, filenames : [], interpreters
    : [], }, Gradle => { name : "Gradle", r#type : "data", color : "#02303a", extensions
    : [".gradle"], aliases : [], tm_scope : "source.groovy.gradle", ace_mode : "text",
    language_id : 136u64, filenames : [], interpreters : [], }, GradleKotlinDSL => { name
    : "Gradle Kotlin DSL", r#type : "data", color : "#02303a", extensions :
    [".gradle.kts"], aliases : [], tm_scope : "source.kotlin", ace_mode : "text",
    language_id : 432600901u64, filenames : [], group : "Gradle", interpreters : [], },
    GrammaticalFramework => { name : "Grammatical Framework", r#type : "programming",
    color : "#ff0000", extensions : [".gf"], aliases : ["gf"], tm_scope : "source.gf",
    ace_mode : "haskell", language_id : 137u64, codemirror_mode : "haskell",
    codemirror_mime_type : "text/x-haskell", filenames : [], interpreters : [], },
    GraphModelingLanguage => { name : "Graph Modeling Language", r#type : "data", color :
    "#000000", extensions : [".gml"], aliases : [], tm_scope : "none", ace_mode : "text",
    language_id : 138u64, filenames : [], interpreters : [], }, GraphQL => { name :
    "GraphQL", r#type : "data", color : "#e10098", extensions : [".graphql", ".gql",
    ".graphqls"], aliases : [], tm_scope : "source.graphql", ace_mode : "graphqlschema",
    language_id : 139u64, filenames : [], interpreters : [], }, GraphvizDOT => { name :
    "Graphviz (DOT)", r#type : "data", color : "#2596be", extensions : [".dot", ".gv"],
    aliases : [], tm_scope : "source.dot", ace_mode : "dot", language_id : 140u64,
    filenames : [], interpreters : [], }, Groovy => { name : "Groovy", r#type :
    "programming", color : "#4298b8", extensions : [".groovy", ".grt", ".gtpl", ".gvy"],
    aliases : [], tm_scope : "source.groovy", ace_mode : "groovy", language_id : 142u64,
    codemirror_mode : "groovy", codemirror_mime_type : "text/x-groovy", filenames :
    ["Jenkinsfile"], interpreters : ["groovy"], }, GroovyServerPages => { name :
    "Groovy Server Pages", r#type : "programming", color : "#4298b8", extensions :
    [".gsp"], aliases : ["gsp", "java server page"], tm_scope : "text.html.jsp", ace_mode
    : "jsp", language_id : 143u64, codemirror_mode : "htmlembedded", codemirror_mime_type
    : "application/x-jsp", filenames : [], group : "Groovy", interpreters : [], },
    HAProxy => { name : "HAProxy", r#type : "data", color : "#106da9", extensions :
    [".cfg"], aliases : [], tm_scope : "source.haproxy-config", ace_mode : "text",
    language_id : 366607477u64, filenames : ["haproxy.cfg"], interpreters : [], }, HCL =>
    { name : "HCL", r#type : "programming", color : "#844FBA", extensions : [".hcl",
    ".nomad", ".tf", ".tfvars", ".workflow"], aliases :
    ["HashiCorp Configuration Language", "terraform"], tm_scope : "source.hcl", ace_mode
    : "terraform", language_id : 144u64, codemirror_mode : "ruby", codemirror_mime_type :
    "text/x-ruby", filenames : [], interpreters : [], }, HIP => { name : "HIP", r#type :
    "programming", color : "#4F3A4F", extensions : [".hip"], aliases : [], tm_scope :
    "source.c++", ace_mode : "c_cpp", language_id : 674379998u64, codemirror_mode :
    "clike", codemirror_mime_type : "text/x-c++src", filenames : [], interpreters : [],
    }, HLSL => { name : "HLSL", r#type : "programming", color : "#aace60", extensions :
    [".hlsl", ".cginc", ".fx", ".fxh", ".hlsli"], aliases : [], tm_scope : "source.hlsl",
    ace_mode : "text", language_id : 145u64, filenames : [], interpreters : [], }, HOCON
    => { name : "HOCON", r#type : "data", color : "#9ff8ee", extensions : [".hocon"],
    aliases : [], tm_scope : "source.hocon", ace_mode : "text", language_id :
    679725279u64, filenames : [".scalafix.conf", ".scalafmt.conf"], interpreters : [], },
    HTML => { name : "HTML", r#type : "markup", color : "#e34c26", extensions : [".html",
    ".hta", ".htm", ".html.hl", ".inc", ".xht", ".xhtml"], aliases : ["xhtml"], tm_scope
    : "text.html.basic", ace_mode : "html", language_id : 146u64, codemirror_mode :
    "htmlmixed", codemirror_mime_type : "text/html", filenames : [], interpreters : [],
    }, HTMLpECR => { name : "HTML+ECR", r#type : "markup", color : "#2e1052", extensions
    : [".ecr"], aliases : ["ecr"], tm_scope : "text.html.ecr", ace_mode : "html_ruby",
    language_id : 148u64, codemirror_mode : "htmlmixed", codemirror_mime_type :
    "text/html", filenames : [], group : "HTML", interpreters : [], }, HTMLpEEX => { name
    : "HTML+EEX", r#type : "markup", color : "#6e4a7e", extensions : [".html.eex",
    ".heex", ".leex"], aliases : ["eex", "heex", "leex"], tm_scope : "text.html.elixir",
    ace_mode : "html_elixir", language_id : 149u64, codemirror_mode : "htmlmixed",
    codemirror_mime_type : "text/html", filenames : [], group : "HTML", interpreters :
    [], }, HTMLpERB => { name : "HTML+ERB", r#type : "markup", color : "#701516",
    extensions : [".erb", ".erb.deface", ".rhtml"], aliases : ["erb", "rhtml",
    "html+ruby"], tm_scope : "text.html.erb", ace_mode : "html_ruby", language_id :
    150u64, codemirror_mode : "htmlembedded", codemirror_mime_type : "application/x-erb",
    filenames : [], group : "HTML", interpreters : [], }, HTMLpPHP => { name :
    "HTML+PHP", r#type : "markup", color : "#4f5d95", extensions : [".phtml"], aliases :
    [], tm_scope : "text.html.php", ace_mode : "php", language_id : 151u64,
    codemirror_mode : "php", codemirror_mime_type : "application/x-httpd-php", filenames
    : [], group : "HTML", interpreters : [], }, HTMLpRazor => { name : "HTML+Razor",
    r#type : "markup", color : "#512be4", extensions : [".cshtml", ".razor"], aliases :
    ["razor"], tm_scope : "text.html.cshtml", ace_mode : "razor", language_id :
    479039817u64, codemirror_mode : "htmlmixed", codemirror_mime_type : "text/html",
    filenames : [], group : "HTML", interpreters : [], }, HTTP => { name : "HTTP", r#type
    : "data", color : "#005C9C", extensions : [".http"], aliases : [], tm_scope :
    "source.httpspec", ace_mode : "text", language_id : 152u64, codemirror_mode : "http",
    codemirror_mime_type : "message/http", filenames : [], interpreters : [], }, HXML =>
    { name : "HXML", r#type : "data", color : "#f68712", extensions : [".hxml"], aliases
    : [], tm_scope : "source.hxml", ace_mode : "text", language_id : 786683730u64,
    codemirror_mode : "haxe", codemirror_mime_type : "text/x-hxml", filenames : [],
    interpreters : [], }, Hack => { name : "Hack", r#type : "programming", color :
    "#878787", extensions : [".hack", ".hh", ".hhi", ".php"], aliases : [], tm_scope :
    "source.hack", ace_mode : "php", language_id : 153u64, codemirror_mode : "php",
    codemirror_mime_type : "application/x-httpd-php", filenames : [], interpreters : [],
    }, Haml => { name : "Haml", r#type : "markup", color : "#ece2a9", extensions :
    [".haml", ".haml.deface"], aliases : [], tm_scope : "text.haml", ace_mode : "haml",
    language_id : 154u64, codemirror_mode : "haml", codemirror_mime_type : "text/x-haml",
    filenames : [], interpreters : [], }, Handlebars => { name : "Handlebars", r#type :
    "markup", color : "#f7931e", extensions : [".handlebars", ".hbs"], aliases : ["hbs",
    "htmlbars"], tm_scope : "text.html.handlebars", ace_mode : "handlebars", language_id
    : 155u64, filenames : [], interpreters : [], }, Harbour => { name : "Harbour", r#type
    : "programming", color : "#0e60e3", extensions : [".hb"], aliases : [], tm_scope :
    "source.harbour", ace_mode : "text", language_id : 156u64, filenames : [],
    interpreters : [], }, Hare => { name : "Hare", r#type : "programming", color :
    "#9d7424", extensions : [".ha"], aliases : [], tm_scope : "none", ace_mode : "text",
    language_id : 463518941u64, filenames : [], interpreters : [], }, Haskell => { name :
    "Haskell", r#type : "programming", color : "#5e5086", extensions : [".hs",
    ".hs-boot", ".hsc"], aliases : [], tm_scope : "source.haskell", ace_mode : "haskell",
    language_id : 157u64, codemirror_mode : "haskell", codemirror_mime_type :
    "text/x-haskell", filenames : [], interpreters : ["runghc", "runhaskell", "runhugs"],
    }, Haxe => { name : "Haxe", r#type : "programming", color : "#df7900", extensions :
    [".hx", ".hxsl"], aliases : [], tm_scope : "source.hx", ace_mode : "haxe",
    language_id : 158u64, codemirror_mode : "haxe", codemirror_mime_type : "text/x-haxe",
    filenames : [], interpreters : [], }, HiveQL => { name : "HiveQL", r#type :
    "programming", color : "#dce200", extensions : [".q", ".hql"], aliases : [], tm_scope
    : "source.hql", ace_mode : "sql", language_id : 931814087u64, filenames : [],
    interpreters : [], }, HolyC => { name : "HolyC", r#type : "programming", color :
    "#ffefaf", extensions : [".hc"], aliases : [], tm_scope : "source.hc", ace_mode :
    "c_cpp", language_id : 928121743u64, codemirror_mode : "clike", codemirror_mime_type
    : "text/x-csrc", filenames : [], interpreters : [], }, HostsFile => { name :
    "Hosts File", r#type : "data", color : "#308888", extensions : [], aliases :
    ["hosts"], tm_scope : "source.hosts", ace_mode : "text", language_id : 231021894u64,
    filenames : ["HOSTS", "hosts", "hosts.txt"], interpreters : [], }, Hurl => { name :
    "Hurl", r#type : "programming", color : "#FF0288", extensions : [".hurl"], aliases :
    [], tm_scope : "source.hurl", ace_mode : "text", language_id : 959040217u64,
    filenames : [], interpreters : [], }, Hy => { name : "Hy", r#type : "programming",
    color : "#7790B2", extensions : [".hy"], aliases : ["hylang"], tm_scope :
    "source.hy", ace_mode : "text", language_id : 159u64, filenames : [], interpreters :
    ["hy"], }, HyPhy => { name : "HyPhy", r#type : "programming", color : "#000000",
    extensions : [".bf"], aliases : [], tm_scope : "none", ace_mode : "text", language_id
    : 160u64, filenames : [], interpreters : [], }, IDL => { name : "IDL", r#type :
    "programming", color : "#a3522f", extensions : [".pro", ".dlm"], aliases : [],
    tm_scope : "source.idl", ace_mode : "text", language_id : 161u64, codemirror_mode :
    "idl", codemirror_mime_type : "text/x-idl", filenames : [], interpreters : [], },
    IGORPro => { name : "IGOR Pro", r#type : "programming", color : "#0000cc", extensions
    : [".ipf"], aliases : ["igor", "igorpro"], tm_scope : "source.igor", ace_mode :
    "text", language_id : 162u64, filenames : [], interpreters : [], }, INI => { name :
    "INI", r#type : "data", color : "#d1dbe0", extensions : [".ini", ".cfg", ".cnf",
    ".dof", ".frm", ".lektorproject", ".prefs", ".pro", ".properties", ".url"], aliases :
    ["dosini"], tm_scope : "source.ini", ace_mode : "ini", language_id : 163u64,
    codemirror_mode : "properties", codemirror_mime_type : "text/x-properties", filenames
    : [".buckconfig", ".coveragerc", ".flake8", ".pylintrc", "HOSTS", "buildozer.spec",
    "hosts", "pylintrc", "vlcrc"], interpreters : [], }, IRCLog => { name : "IRC log",
    r#type : "data", color : "#000000", extensions : [".irclog", ".weechatlog"], aliases
    : ["irc", "irc logs"], tm_scope : "none", ace_mode : "text", language_id : 164u64,
    codemirror_mode : "mirc", codemirror_mime_type : "text/mirc", filenames : [],
    interpreters : [], }, ISPC => { name : "ISPC", r#type : "programming", color :
    "#2D68B1", extensions : [".ispc"], aliases : [], tm_scope : "source.ispc", ace_mode :
    "c_cpp", language_id : 327071u64, codemirror_mode : "clike", codemirror_mime_type :
    "text/x-csrc", filenames : [], interpreters : [], }, Idris => { name : "Idris",
    r#type : "programming", color : "#b30000", extensions : [".idr", ".lidr"], aliases :
    [], tm_scope : "source.idris", ace_mode : "text", language_id : 165u64, filenames :
    [], interpreters : [], }, IgnoreList => { name : "Ignore List", r#type : "data",
    color : "#000000", extensions : [".gitignore"], aliases : ["ignore", "gitignore",
    "git-ignore"], tm_scope : "source.gitignore", ace_mode : "gitignore", language_id :
    74444240u64, codemirror_mode : "shell", codemirror_mime_type : "text/x-sh", filenames
    : [".atomignore", ".babelignore", ".bzrignore", ".coffeelintignore", ".cvsignore",
    ".dockerignore", ".easignore", ".eleventyignore", ".eslintignore", ".gitignore",
    ".ignore", ".markdownlintignore", ".nodemonignore", ".npmignore", ".prettierignore",
    ".stylelintignore", ".vercelignore", ".vscodeignore", "gitignore-global",
    "gitignore_global"], interpreters : [], }, ImageJMacro => { name : "ImageJ Macro",
    r#type : "programming", color : "#99AAFF", extensions : [".ijm"], aliases : ["ijm"],
    tm_scope : "none", ace_mode : "text", language_id : 575143428u64, filenames : [],
    interpreters : [], }, Imba => { name : "Imba", r#type : "programming", color :
    "#16cec6", extensions : [".imba"], aliases : [], tm_scope : "source.imba", ace_mode :
    "text", language_id : 1057618448u64, filenames : [], interpreters : [], }, Inform7 =>
    { name : "Inform 7", r#type : "programming", color : "#000000", extensions : [".ni",
    ".i7x"], aliases : ["i7", "inform7"], tm_scope : "source.inform7", ace_mode : "text",
    language_id : 166u64, wrap : true, filenames : [], interpreters : [], }, Ink => {
    name : "Ink", r#type : "programming", color : "#000000", extensions : [".ink"],
    aliases : [], tm_scope : "source.ink", ace_mode : "text", language_id : 838252715u64,
    wrap : true, filenames : [], interpreters : [], }, InnoSetup => { name :
    "Inno Setup", r#type : "programming", color : "#264b99", extensions : [".iss",
    ".isl"], aliases : [], tm_scope : "source.inno", ace_mode : "text", language_id :
    167u64, filenames : [], interpreters : [], }, Io => { name : "Io", r#type :
    "programming", color : "#a9188d", extensions : [".io"], aliases : [], tm_scope :
    "source.io", ace_mode : "io", language_id : 168u64, filenames : [], interpreters :
    ["io"], }, Ioke => { name : "Ioke", r#type : "programming", color : "#078193",
    extensions : [".ik"], aliases : [], tm_scope : "source.ioke", ace_mode : "text",
    language_id : 169u64, filenames : [], interpreters : ["ioke"], }, Isabelle => { name
    : "Isabelle", r#type : "programming", color : "#FEFE00", extensions : [".thy"],
    aliases : [], tm_scope : "source.isabelle.theory", ace_mode : "text", language_id :
    170u64, filenames : [], interpreters : [], }, IsabelleROOT => { name :
    "Isabelle ROOT", r#type : "programming", color : "#FEFE00", extensions : [], aliases
    : [], tm_scope : "source.isabelle.root", ace_mode : "text", language_id : 171u64,
    filenames : ["ROOT"], group : "Isabelle", interpreters : [], }, J => { name : "J",
    r#type : "programming", color : "#9EEDFF", extensions : [".ijs"], aliases : [],
    tm_scope : "source.j", ace_mode : "text", language_id : 172u64, filenames : [],
    interpreters : ["jconsole"], }, JARManifest => { name : "JAR Manifest", r#type :
    "data", color : "#b07219", extensions : [], aliases : [], tm_scope : "source.yaml",
    ace_mode : "text", language_id : 447261135u64, filenames : ["MANIFEST.MF"],
    interpreters : [], }, JCL => { name : "JCL", r#type : "programming", color :
    "#d90e09", extensions : [".jcl"], aliases : [], tm_scope : "source.jcl", ace_mode :
    "text", language_id : 316620079u64, filenames : [], interpreters : [], }, JFlex => {
    name : "JFlex", r#type : "programming", color : "#DBCA00", extensions : [".flex",
    ".jflex"], aliases : [], tm_scope : "source.jflex", ace_mode : "text", language_id :
    173u64, filenames : [], group : "Lex", interpreters : [], }, JSON => { name : "JSON",
    r#type : "data", color : "#292929", extensions : [".json", ".4DForm", ".4DProject",
    ".avsc", ".geojson", ".gltf", ".har", ".ice", ".JSON-tmLanguage", ".json.example",
    ".jsonl", ".mcmeta", ".sarif", ".tact", ".tfstate", ".tfstate.backup", ".topojson",
    ".webapp", ".webmanifest", ".yy", ".yyp"], aliases : ["geojson", "jsonl", "sarif",
    "topojson"], tm_scope : "source.json", ace_mode : "json", language_id : 174u64,
    codemirror_mode : "javascript", codemirror_mime_type : "application/json", filenames
    : [".all-contributorsrc", ".arcconfig", ".auto-changelog", ".c8rc", ".htmlhintrc",
    ".imgbotconfig", ".nycrc", ".tern-config", ".tern-project", ".watchmanconfig",
    "MODULE.bazel.lock", "Package.resolved", "Pipfile.lock", "bun.lock", "composer.lock",
    "deno.lock", "flake.lock", "mcmod.info"], interpreters : [], }, JSONWithComments => {
    name : "JSON with Comments", r#type : "data", color : "#292929", extensions :
    [".jsonc", ".code-snippets", ".code-workspace", ".sublime-build",
    ".sublime-color-scheme", ".sublime-commands", ".sublime-completions",
    ".sublime-keymap", ".sublime-macro", ".sublime-menu", ".sublime-mousemap",
    ".sublime-project", ".sublime-settings", ".sublime-theme", ".sublime-workspace",
    ".sublime_metrics", ".sublime_session", ".tsconfig.json"], aliases : ["jsonc"],
    tm_scope : "source.json.comments", ace_mode : "javascript", language_id : 423u64,
    codemirror_mode : "javascript", codemirror_mime_type : "text/javascript", filenames :
    [".babelrc", ".devcontainer.json", ".eslintrc.json", ".jscsrc", ".jshintrc",
    ".jslintrc", ".oxlintrc.json", ".swcrc", "api-extractor.json", "devcontainer.json",
    "jsconfig.json", "language-configuration.json", "tsconfig.json", "tslint.json"],
    group : "JSON", interpreters : [], }, JSON5 => { name : "JSON5", r#type : "data",
    color : "#267CB9", extensions : [".json5"], aliases : [], tm_scope : "source.js",
    ace_mode : "json5", language_id : 175u64, codemirror_mode : "javascript",
    codemirror_mime_type : "application/json", filenames : [], interpreters : [], },
    JSONLD => { name : "JSONLD", r#type : "data", color : "#0c479c", extensions :
    [".jsonld"], aliases : [], tm_scope : "source.js", ace_mode : "javascript",
    language_id : 176u64, codemirror_mode : "javascript", codemirror_mime_type :
    "application/ld+json", filenames : [], interpreters : [], }, JSONiq => { name :
    "JSONiq", r#type : "programming", color : "#40d47e", extensions : [".jq"], aliases :
    [], tm_scope : "source.jsoniq", ace_mode : "jsoniq", language_id : 177u64,
    codemirror_mode : "javascript", codemirror_mime_type : "application/json", filenames
    : [], interpreters : [], }, Jai => { name : "Jai", r#type : "programming", color :
    "#ab8b4b", extensions : [".jai"], aliases : [], tm_scope : "source.jai", ace_mode :
    "text", language_id : 70127133u64, filenames : [], interpreters : [], }, Janet => {
    name : "Janet", r#type : "programming", color : "#0886a5", extensions : [".janet"],
    aliases : [], tm_scope : "source.janet", ace_mode : "scheme", language_id :
    1028705371u64, codemirror_mode : "scheme", codemirror_mime_type : "text/x-scheme",
    filenames : [], interpreters : ["janet"], }, Jasmin => { name : "Jasmin", r#type :
    "programming", color : "#d03600", extensions : [".j"], aliases : [], tm_scope :
    "source.jasmin", ace_mode : "java", language_id : 180u64, filenames : [],
    interpreters : [], }, Java => { name : "Java", r#type : "programming", color :
    "#b07219", extensions : [".java", ".jav", ".jsh"], aliases : [], tm_scope :
    "source.java", ace_mode : "java", language_id : 181u64, codemirror_mode : "clike",
    codemirror_mime_type : "text/x-java", filenames : [], interpreters : [], },
    JavaProperties => { name : "Java Properties", r#type : "data", color : "#2A6277",
    extensions : [".properties"], aliases : [], tm_scope : "source.java-properties",
    ace_mode : "properties", language_id : 519377561u64, codemirror_mode : "properties",
    codemirror_mime_type : "text/x-properties", filenames : [], interpreters : [], },
    JavaServerPages => { name : "Java Server Pages", r#type : "programming", color :
    "#2A6277", extensions : [".jsp", ".tag"], aliases : ["jsp"], tm_scope :
    "text.html.jsp", ace_mode : "jsp", language_id : 182u64, codemirror_mode :
    "htmlembedded", codemirror_mime_type : "application/x-jsp", filenames : [], group :
    "Java", interpreters : [], }, JavaTemplateEngine => { name : "Java Template Engine",
    r#type : "programming", color : "#2A6277", extensions : [".jte"], aliases : ["jte"],
    tm_scope : "text.html.jte", ace_mode : "text", language_id : 599494012u64, filenames
    : [], group : "Java", interpreters : [], }, JavaScript => { name : "JavaScript",
    r#type : "programming", color : "#f1e05a", extensions : [".js", "._js", ".bones",
    ".cjs", ".es", ".es6", ".frag", ".gs", ".jake", ".javascript", ".jsb", ".jscad",
    ".jsfl", ".jslib", ".jsm", ".jspre", ".jss", ".jsx", ".mjs", ".njs", ".pac", ".sjs",
    ".ssjs", ".xsjs", ".xsjslib"], aliases : ["js", "node"], tm_scope : "source.js",
    ace_mode : "javascript", language_id : 183u64, codemirror_mode : "javascript",
    codemirror_mime_type : "text/javascript", filenames : ["Jakefile"], interpreters :
    ["chakra", "d8", "gjs", "js", "node", "nodejs", "qjs", "rhino", "v8", "v8-shell"], },
    JavaScriptpERB => { name : "JavaScript+ERB", r#type : "programming", color :
    "#f1e05a", extensions : [".js.erb"], aliases : [], tm_scope : "source.js", ace_mode :
    "javascript", language_id : 914318960u64, codemirror_mode : "javascript",
    codemirror_mime_type : "application/javascript", filenames : [], group :
    "JavaScript", interpreters : [], }, JestSnapshot => { name : "Jest Snapshot", r#type
    : "data", color : "#15c213", extensions : [".snap"], aliases : [], tm_scope :
    "source.jest.snap", ace_mode : "javascript", language_id : 774635084u64,
    codemirror_mode : "javascript", codemirror_mime_type : "application/javascript",
    filenames : [], interpreters : [], }, JetBrainsMPS => { name : "JetBrains MPS",
    r#type : "programming", color : "#21D789", extensions : [".mps", ".mpl", ".msd"],
    aliases : ["mps"], tm_scope : "none", ace_mode : "xml", language_id : 465165328u64,
    codemirror_mode : "xml", codemirror_mime_type : "text/xml", filenames : [],
    interpreters : [], }, Jinja => { name : "Jinja", r#type : "markup", color :
    "#a52a22", extensions : [".jinja", ".j2", ".jinja2"], aliases : ["django",
    "html+django", "html+jinja", "htmldjango"], tm_scope : "text.html.django", ace_mode :
    "django", language_id : 147u64, codemirror_mode : "jinja2", codemirror_mime_type :
    "text/jinja2", filenames : [], interpreters : [], }, Jison => { name : "Jison",
    r#type : "programming", color : "#56b3cb", extensions : [".jison"], aliases : [],
    tm_scope : "source.jison", ace_mode : "text", language_id : 284531423u64, filenames :
    [], group : "Yacc", interpreters : [], }, JisonLex => { name : "Jison Lex", r#type :
    "programming", color : "#56b3cb", extensions : [".jisonlex"], aliases : [], tm_scope
    : "source.jisonlex", ace_mode : "text", language_id : 406395330u64, filenames : [],
    group : "Lex", interpreters : [], }, Jolie => { name : "Jolie", r#type :
    "programming", color : "#843179", extensions : [".ol", ".iol"], aliases : [],
    tm_scope : "source.jolie", ace_mode : "text", language_id : 998078858u64, filenames :
    [], interpreters : ["jolie"], }, Jsonnet => { name : "Jsonnet", r#type :
    "programming", color : "#0064bd", extensions : [".jsonnet", ".libsonnet"], aliases :
    [], tm_scope : "source.jsonnet", ace_mode : "text", language_id : 664885656u64,
    filenames : [], interpreters : [], }, Julia => { name : "Julia", r#type :
    "programming", color : "#a270ba", extensions : [".jl"], aliases : [], tm_scope :
    "source.julia", ace_mode : "julia", language_id : 184u64, codemirror_mode : "julia",
    codemirror_mime_type : "text/x-julia", filenames : [], interpreters : ["julia"], },
    JuliaREPL => { name : "Julia REPL", r#type : "programming", color : "#a270ba",
    extensions : [], aliases : [], tm_scope : "source.julia.console", ace_mode : "text",
    language_id : 220689142u64, filenames : [], group : "Julia", interpreters : [], },
    JupyterNotebook => { name : "Jupyter Notebook", r#type : "markup", color : "#DA5B0B",
    extensions : [".ipynb"], aliases : ["IPython Notebook"], tm_scope : "source.json",
    ace_mode : "json", language_id : 185u64, codemirror_mode : "javascript",
    codemirror_mime_type : "application/json", filenames : ["Notebook"], interpreters :
    [], }, Just => { name : "Just", r#type : "programming", color : "#384d54", extensions
    : [".just"], aliases : ["Justfile"], tm_scope : "source.just", ace_mode : "text",
    language_id : 128447695u64, filenames : [".JUSTFILE", ".Justfile", ".justfile",
    "JUSTFILE", "Justfile", "justfile"], interpreters : [], }, KCL => { name : "KCL",
    r#type : "programming", color : "#7ABABF", extensions : [".k"], aliases : [],
    tm_scope : "source.kcl", ace_mode : "text", language_id : 1052003890u64, filenames :
    ["kcl.mod", "kcl.mod.lock"], interpreters : [], }, KDL => { name : "KDL", r#type :
    "data", color : "#ffb3b3", extensions : [".kdl"], aliases : [], tm_scope :
    "source.kdl", ace_mode : "tcl", language_id : 931123626u64, codemirror_mode :
    "yacas", codemirror_mime_type : "text/x-yacas", filenames : [], interpreters : [], },
    KFramework => { name : "KFramework", r#type : "programming", color : "#4195c5",
    extensions : [".k"], aliases : [], tm_scope : "text.k", ace_mode : "text",
    language_id : 9479532u64, filenames : [], interpreters : [], }, KRL => { name :
    "KRL", r#type : "programming", color : "#28430A", extensions : [".krl"], aliases :
    [], tm_scope : "none", ace_mode : "text", language_id : 186u64, filenames : [],
    interpreters : [], }, KaitaiStruct => { name : "Kaitai Struct", r#type :
    "programming", color : "#773b37", extensions : [".ksy"], aliases : ["ksy"], tm_scope
    : "source.yaml", ace_mode : "yaml", language_id : 818804755u64, codemirror_mode :
    "yaml", codemirror_mime_type : "text/x-yaml", filenames : [], interpreters : [], },
    KakouneScript => { name : "KakouneScript", r#type : "programming", color : "#6f8042",
    extensions : [".kak"], aliases : ["kak", "kakscript"], tm_scope : "source.kakscript",
    ace_mode : "text", language_id : 603336474u64, filenames : ["kakrc"], interpreters :
    [], }, KerboScript => { name : "KerboScript", r#type : "programming", color :
    "#41adf0", extensions : [".ks"], aliases : [], tm_scope : "source.kerboscript",
    ace_mode : "text", language_id : 59716426u64, filenames : [], interpreters : [], },
    KiCadLayout => { name : "KiCad Layout", r#type : "data", color : "#2f4aab",
    extensions : [".kicad_pcb", ".kicad_mod", ".kicad_wks"], aliases : ["pcbnew"],
    tm_scope : "source.pcb.sexp", ace_mode : "lisp", language_id : 187u64,
    codemirror_mode : "commonlisp", codemirror_mime_type : "text/x-common-lisp",
    filenames : ["fp-lib-table"], interpreters : [], }, KiCadLegacyLayout => { name :
    "KiCad Legacy Layout", r#type : "data", color : "#2f4aab", extensions : [".brd"],
    aliases : [], tm_scope : "source.pcb.board", ace_mode : "text", language_id :
    140848857u64, filenames : [], interpreters : [], }, KiCadSchematic => { name :
    "KiCad Schematic", r#type : "data", color : "#2f4aab", extensions : [".kicad_sch",
    ".kicad_sym", ".sch"], aliases : ["eeschema schematic"], tm_scope :
    "source.pcb.schematic", ace_mode : "text", language_id : 622447435u64, filenames :
    [], interpreters : [], }, Kickstart => { name : "Kickstart", r#type : "data", color :
    "#000000", extensions : [".ks"], aliases : [], tm_scope : "source.kickstart",
    ace_mode : "text", language_id : 692635484u64, filenames : [], interpreters : [], },
    Kit => { name : "Kit", r#type : "markup", color : "#000000", extensions : [".kit"],
    aliases : [], tm_scope : "text.html.basic", ace_mode : "html", language_id : 188u64,
    codemirror_mode : "htmlmixed", codemirror_mime_type : "text/html", filenames : [],
    interpreters : [], }, KoLmafiaASH => { name : "KoLmafia ASH", r#type : "programming",
    color : "#B9D9B9", extensions : [".ash"], aliases : [], tm_scope : "source.ash",
    ace_mode : "text", language_id : 852099832u64, filenames : [], interpreters : [], },
    Koka => { name : "Koka", r#type : "programming", color : "#215166", extensions :
    [".kk"], aliases : [], tm_scope : "source.koka", ace_mode : "text", language_id :
    597930447u64, filenames : [], interpreters : ["koka"], }, Kotlin => { name :
    "Kotlin", r#type : "programming", color : "#A97BFF", extensions : [".kt", ".ktm",
    ".kts"], aliases : [], tm_scope : "source.kotlin", ace_mode : "kotlin", language_id :
    189u64, codemirror_mode : "clike", codemirror_mime_type : "text/x-kotlin", filenames
    : [], interpreters : [], }, Kusto => { name : "Kusto", r#type : "data", color :
    "#000000", extensions : [".csl", ".kql"], aliases : [], tm_scope : "source.kusto",
    ace_mode : "text", language_id : 225697190u64, filenames : [], interpreters : [], },
    LFE => { name : "LFE", r#type : "programming", color : "#4C3023", extensions :
    [".lfe"], aliases : [], tm_scope : "source.lisp", ace_mode : "lisp", language_id :
    190u64, codemirror_mode : "commonlisp", codemirror_mime_type : "text/x-common-lisp",
    filenames : [], interpreters : [], }, LLVM => { name : "LLVM", r#type :
    "programming", color : "#185619", extensions : [".ll"], aliases : [], tm_scope :
    "source.llvm", ace_mode : "text", language_id : 191u64, filenames : [], interpreters
    : [], }, LOLCODE => { name : "LOLCODE", r#type : "programming", color : "#cc9900",
    extensions : [".lol"], aliases : [], tm_scope : "source.lolcode", ace_mode : "text",
    language_id : 192u64, filenames : [], interpreters : [], }, LSL => { name : "LSL",
    r#type : "programming", color : "#3d9970", extensions : [".lsl", ".lslp"], aliases :
    [], tm_scope : "source.lsl", ace_mode : "lsl", language_id : 193u64, filenames : [],
    interpreters : ["lsl"], }, LTspiceSymbol => { name : "LTspice Symbol", r#type :
    "data", color : "#000000", extensions : [".asy"], aliases : [], tm_scope :
    "source.ltspice.symbol", ace_mode : "text", language_id : 1013566805u64,
    codemirror_mode : "spreadsheet", codemirror_mime_type : "text/x-spreadsheet",
    filenames : [], interpreters : [], }, LabVIEW => { name : "LabVIEW", r#type :
    "programming", color : "#fede06", extensions : [".lvproj", ".lvclass", ".lvlib"],
    aliases : [], tm_scope : "text.xml", ace_mode : "xml", language_id : 194u64,
    codemirror_mode : "xml", codemirror_mime_type : "text/xml", filenames : [],
    interpreters : [], }, Langium => { name : "Langium", r#type : "programming", color :
    "#2c8c87", extensions : [".langium"], aliases : [], tm_scope : "source.langium",
    ace_mode : "text", language_id : 548603830u64, filenames : [], interpreters : [], },
    Lark => { name : "Lark", r#type : "data", color : "#2980B9", extensions : [".lark"],
    aliases : [], tm_scope : "source.lark", ace_mode : "text", language_id :
    758480799u64, codemirror_mode : "ebnf", codemirror_mime_type : "text/x-ebnf",
    filenames : [], interpreters : [], }, Lasso => { name : "Lasso", r#type :
    "programming", color : "#999999", extensions : [".lasso", ".las", ".lasso8",
    ".lasso9"], aliases : ["lassoscript"], tm_scope : "file.lasso", ace_mode : "text",
    language_id : 195u64, filenames : [], interpreters : [], }, Latte => { name :
    "Latte", r#type : "markup", color : "#f2a542", extensions : [".latte"], aliases : [],
    tm_scope : "text.html.smarty", ace_mode : "latte", language_id : 196u64,
    codemirror_mode : "smarty", codemirror_mime_type : "text/x-smarty", filenames : [],
    interpreters : [], }, Lean => { name : "Lean", r#type : "programming", color :
    "#000000", extensions : [".lean", ".hlean"], aliases : [], tm_scope : "source.lean",
    ace_mode : "text", language_id : 197u64, filenames : [], interpreters : [], }, Lean4
    => { name : "Lean 4", r#type : "programming", color : "#000000", extensions :
    [".lean"], aliases : ["lean4"], tm_scope : "source.lean4", ace_mode : "text",
    language_id : 455147478u64, filenames : [], group : "Lean", interpreters : [], }, Leo
    => { name : "Leo", r#type : "programming", color : "#C4FFC2", extensions : [".leo"],
    aliases : [], tm_scope : "source.leo", ace_mode : "text", language_id : 916034822u64,
    wrap : true, filenames : [], interpreters : [], }, Less => { name : "Less", r#type :
    "markup", color : "#1d365d", extensions : [".less"], aliases : ["less-css"], tm_scope
    : "source.css.less", ace_mode : "less", language_id : 198u64, codemirror_mode :
    "css", codemirror_mime_type : "text/x-less", filenames : [], interpreters : [], },
    Lex => { name : "Lex", r#type : "programming", color : "#DBCA00", extensions : [".l",
    ".lex"], aliases : ["flex"], tm_scope : "source.lex", ace_mode : "text", language_id
    : 199u64, filenames : ["Lexer.x", "lexer.x"], interpreters : [], }, LigoLANG => {
    name : "LigoLANG", r#type : "programming", color : "#0e74ff", extensions : [".ligo"],
    aliases : [], tm_scope : "source.ligo", ace_mode : "pascal", language_id :
    1040646257u64, codemirror_mode : "pascal", codemirror_mime_type : "text/x-pascal",
    filenames : [], group : "LigoLANG", interpreters : [], }, LilyPond => { name :
    "LilyPond", r#type : "programming", color : "#9ccc7c", extensions : [".ly", ".ily"],
    aliases : [], tm_scope : "source.lilypond", ace_mode : "text", language_id : 200u64,
    filenames : [], interpreters : [], }, Limbo => { name : "Limbo", r#type :
    "programming", color : "#000000", extensions : [".b", ".m"], aliases : [], tm_scope :
    "none", ace_mode : "text", language_id : 201u64, filenames : [], interpreters : [],
    }, LinearProgramming => { name : "Linear Programming", r#type : "programming", color
    : "#000000", extensions : [".lp"], aliases : [], tm_scope : "none", ace_mode :
    "text", language_id : 377204539u64, filenames : [], interpreters : [], },
    LinkerScript => { name : "Linker Script", r#type : "programming", color : "#000000",
    extensions : [".ld", ".lds", ".x"], aliases : [], tm_scope : "source.c.linker",
    ace_mode : "text", language_id : 202u64, filenames : ["ld.script"], interpreters :
    [], }, LinuxKernelModule => { name : "Linux Kernel Module", r#type : "data", color :
    "#000000", extensions : [".mod"], aliases : [], tm_scope : "none", ace_mode : "text",
    language_id : 203u64, filenames : [], interpreters : [], }, Liquid => { name :
    "Liquid", r#type : "markup", color : "#67b8de", extensions : [".liquid"], aliases :
    [], tm_scope : "text.html.liquid", ace_mode : "liquid", language_id : 204u64,
    filenames : [], interpreters : [], }, LiterateAgda => { name : "Literate Agda",
    r#type : "programming", color : "#315665", extensions : [".lagda"], aliases : [],
    tm_scope : "none", ace_mode : "text", language_id : 205u64, filenames : [], group :
    "Agda", interpreters : [], }, LiterateCoffeeScript => { name :
    "Literate CoffeeScript", r#type : "programming", color : "#244776", extensions :
    [".litcoffee", ".coffee.md"], aliases : ["litcoffee"], tm_scope : "source.litcoffee",
    ace_mode : "text", language_id : 206u64, wrap : true, filenames : [], group :
    "CoffeeScript", interpreters : [], }, LiterateHaskell => { name : "Literate Haskell",
    r#type : "programming", color : "#5e5086", extensions : [".lhs"], aliases :
    ["lhaskell", "lhs"], tm_scope : "text.tex.latex.haskell", ace_mode : "text",
    language_id : 207u64, codemirror_mode : "haskell-literate", codemirror_mime_type :
    "text/x-literate-haskell", filenames : [], group : "Haskell", interpreters : [], },
    LiveCodeScript => { name : "LiveCode Script", r#type : "programming", color :
    "#0c5ba5", extensions : [".livecodescript"], aliases : [], tm_scope :
    "source.livecodescript", ace_mode : "text", language_id : 891017u64, filenames : [],
    interpreters : [], }, LiveScript => { name : "LiveScript", r#type : "programming",
    color : "#499886", extensions : [".ls", "._ls"], aliases : ["live-script", "ls"],
    tm_scope : "source.livescript", ace_mode : "livescript", language_id : 208u64,
    codemirror_mode : "livescript", codemirror_mime_type : "text/x-livescript", filenames
    : ["Slakefile"], interpreters : [], }, Logos => { name : "Logos", r#type :
    "programming", color : "#000000", extensions : [".xm", ".x", ".xi"], aliases : [],
    tm_scope : "source.logos", ace_mode : "text", language_id : 209u64, filenames : [],
    interpreters : [], }, Logtalk => { name : "Logtalk", r#type : "programming", color :
    "#295b9a", extensions : [".lgt", ".logtalk"], aliases : [], tm_scope :
    "source.logtalk", ace_mode : "logtalk", language_id : 210u64, filenames : [],
    interpreters : [], }, LookML => { name : "LookML", r#type : "programming", color :
    "#652B81", extensions : [".lkml", ".lookml"], aliases : [], tm_scope : "source.yaml",
    ace_mode : "yaml", language_id : 211u64, codemirror_mode : "yaml",
    codemirror_mime_type : "text/x-yaml", filenames : [], interpreters : [], },
    LoomScript => { name : "LoomScript", r#type : "programming", color : "#000000",
    extensions : [".ls"], aliases : [], tm_scope : "source.loomscript", ace_mode :
    "text", language_id : 212u64, filenames : [], interpreters : [], }, Lua => { name :
    "Lua", r#type : "programming", color : "#000080", extensions : [".lua", ".fcgi",
    ".nse", ".p8", ".pd_lua", ".rbxs", ".rockspec", ".wlua"], aliases : [], tm_scope :
    "source.lua", ace_mode : "lua", language_id : 213u64, codemirror_mode : "lua",
    codemirror_mime_type : "text/x-lua", filenames : [".luacheckrc"], interpreters :
    ["lua", "luajit"], }, Luau => { name : "Luau", r#type : "programming", color :
    "#00A2FF", extensions : [".luau"], aliases : [], tm_scope : "source.luau", ace_mode :
    "lua", language_id : 365050359u64, codemirror_mode : "lua", codemirror_mime_type :
    "text/x-lua", filenames : [], interpreters : ["luau"], }, M => { name : "M", r#type :
    "programming", color : "#000000", extensions : [".mumps", ".m"], aliases : ["mumps"],
    tm_scope : "none", ace_mode : "text", language_id : 214u64, codemirror_mode :
    "mumps", codemirror_mime_type : "text/x-mumps", filenames : [], interpreters : [], },
    M3U => { name : "M3U", r#type : "data", color : "#179C7D", extensions : [".m3u",
    ".m3u8"], aliases : ["hls playlist", "m3u playlist"], tm_scope : "source.m3u",
    ace_mode : "text", language_id : 89638692u64, filenames : [], interpreters : [], },
    M4 => { name : "M4", r#type : "programming", color : "#000000", extensions : [".m4",
    ".mc"], aliases : [], tm_scope : "source.m4", ace_mode : "text", language_id :
    215u64, filenames : [], interpreters : [], }, M4Sugar => { name : "M4Sugar", r#type :
    "programming", color : "#000000", extensions : [".m4"], aliases : ["autoconf"],
    tm_scope : "source.m4", ace_mode : "text", language_id : 216u64, filenames :
    ["configure.ac"], group : "M4", interpreters : [], }, MATLAB => { name : "MATLAB",
    r#type : "programming", color : "#e16737", extensions : [".matlab", ".m"], aliases :
    ["octave"], tm_scope : "source.matlab", ace_mode : "matlab", language_id : 225u64,
    codemirror_mode : "octave", codemirror_mime_type : "text/x-octave", filenames : [],
    interpreters : [], }, MAXScript => { name : "MAXScript", r#type : "programming",
    color : "#00a6a6", extensions : [".ms", ".mcr"], aliases : [], tm_scope :
    "source.maxscript", ace_mode : "text", language_id : 217u64, filenames : [],
    interpreters : [], }, MDX => { name : "MDX", r#type : "markup", color : "#fcb32c",
    extensions : [".mdx"], aliases : [], tm_scope : "source.mdx", ace_mode : "markdown",
    language_id : 512838272u64, codemirror_mode : "gfm", codemirror_mime_type :
    "text/x-gfm", wrap : true, filenames : [], interpreters : [], }, MLIR => { name :
    "MLIR", r#type : "programming", color : "#5EC8DB", extensions : [".mlir"], aliases :
    [], tm_scope : "source.mlir", ace_mode : "text", language_id : 448253929u64,
    filenames : [], interpreters : [], }, MQL4 => { name : "MQL4", r#type :
    "programming", color : "#62A8D6", extensions : [".mq4", ".mqh"], aliases : [],
    tm_scope : "source.mql5", ace_mode : "c_cpp", language_id : 426u64, filenames : [],
    interpreters : [], }, MQL5 => { name : "MQL5", r#type : "programming", color :
    "#4A76B8", extensions : [".mq5", ".mqh"], aliases : [], tm_scope : "source.mql5",
    ace_mode : "c_cpp", language_id : 427u64, filenames : [], interpreters : [], }, MTML
    => { name : "MTML", r#type : "markup", color : "#b7e1f4", extensions : [".mtml"],
    aliases : [], tm_scope : "text.html.basic", ace_mode : "html", language_id : 218u64,
    codemirror_mode : "htmlmixed", codemirror_mime_type : "text/html", filenames : [],
    interpreters : [], }, MUF => { name : "MUF", r#type : "programming", color :
    "#000000", extensions : [".muf", ".m"], aliases : [], tm_scope : "none", ace_mode :
    "forth", language_id : 219u64, codemirror_mode : "forth", codemirror_mime_type :
    "text/x-forth", filenames : [], group : "Forth", interpreters : [], }, Macaulay2 => {
    name : "Macaulay2", r#type : "programming", color : "#d8ffff", extensions : [".m2"],
    aliases : ["m2"], tm_scope : "source.m2", ace_mode : "text", language_id :
    34167825u64, filenames : [], interpreters : ["M2"], }, Makefile => { name :
    "Makefile", r#type : "programming", color : "#427819", extensions : [".mak", ".d",
    ".make", ".makefile", ".mk", ".mkfile"], aliases : ["bsdmake", "make", "mf"],
    tm_scope : "source.makefile", ace_mode : "makefile", language_id : 220u64,
    codemirror_mode : "cmake", codemirror_mime_type : "text/x-cmake", filenames :
    ["BSDmakefile", "GNUmakefile", "Kbuild", "Makefile", "Makefile.am", "Makefile.boot",
    "Makefile.frag", "Makefile.in", "Makefile.inc", "Makefile.wat", "makefile",
    "makefile.sco", "mkfile"], interpreters : ["make"], }, Mako => { name : "Mako",
    r#type : "programming", color : "#7e858d", extensions : [".mako", ".mao"], aliases :
    [], tm_scope : "text.html.mako", ace_mode : "text", language_id : 221u64, filenames :
    [], interpreters : [], }, Markdown => { name : "Markdown", r#type : "prose", color :
    "#083fa1", extensions : [".md", ".livemd", ".markdown", ".mdown", ".mdwn", ".mkd",
    ".mkdn", ".mkdown", ".ronn", ".scd", ".workbook"], aliases : ["md", "pandoc"],
    tm_scope : "text.md", ace_mode : "markdown", language_id : 222u64, codemirror_mode :
    "gfm", codemirror_mime_type : "text/x-gfm", wrap : true, filenames : ["contents.lr"],
    interpreters : [], }, Marko => { name : "Marko", r#type : "markup", color :
    "#42bff2", extensions : [".marko"], aliases : ["markojs"], tm_scope : "text.marko",
    ace_mode : "text", language_id : 932782397u64, codemirror_mode : "htmlmixed",
    codemirror_mime_type : "text/html", filenames : [], interpreters : [], }, Mask => {
    name : "Mask", r#type : "markup", color : "#f97732", extensions : [".mask"], aliases
    : [], tm_scope : "source.mask", ace_mode : "mask", language_id : 223u64, filenames :
    [], interpreters : [], }, MavenPOM => { name : "Maven POM", r#type : "data", color :
    "#000000", extensions : [], aliases : [], tm_scope : "text.xml.pom", ace_mode :
    "xml", language_id : 226u64, codemirror_mode : "xml", codemirror_mime_type :
    "text/xml", filenames : ["pom.xml"], group : "XML", interpreters : [], }, Max => {
    name : "Max", r#type : "programming", color : "#c4a79c", extensions : [".maxpat",
    ".maxhelp", ".maxproj", ".mxt", ".pat"], aliases : ["max/msp", "maxmsp"], tm_scope :
    "source.json", ace_mode : "json", language_id : 227u64, codemirror_mode :
    "javascript", codemirror_mime_type : "application/json", filenames : [], interpreters
    : [], }, Mercury => { name : "Mercury", r#type : "programming", color : "#ff2b2b",
    extensions : [".m", ".moo"], aliases : [], tm_scope : "source.mercury", ace_mode :
    "prolog", language_id : 229u64, filenames : [], interpreters : ["mmi"], }, Mermaid =>
    { name : "Mermaid", r#type : "markup", color : "#ff3670", extensions : [".mmd",
    ".mermaid"], aliases : ["mermaid example"], tm_scope : "source.mermaid", ace_mode :
    "text", language_id : 385992043u64, filenames : [], interpreters : [], }, Meson => {
    name : "Meson", r#type : "programming", color : "#007800", extensions : [], aliases :
    [], tm_scope : "source.meson", ace_mode : "text", language_id : 799141244u64,
    filenames : ["meson.build", "meson_options.txt"], interpreters : [], }, Metal => {
    name : "Metal", r#type : "programming", color : "#8f14e9", extensions : [".metal"],
    aliases : [], tm_scope : "source.c++", ace_mode : "c_cpp", language_id : 230u64,
    codemirror_mode : "clike", codemirror_mime_type : "text/x-c++src", filenames : [],
    interpreters : [], }, MicrosoftDeveloperStudioProject => { name :
    "Microsoft Developer Studio Project", r#type : "data", color : "#000000", extensions
    : [".dsp"], aliases : [], tm_scope : "none", ace_mode : "text", language_id :
    800983837u64, filenames : [], interpreters : [], }, MicrosoftVisualStudioSolution =>
    { name : "Microsoft Visual Studio Solution", r#type : "data", color : "#000000",
    extensions : [".sln"], aliases : [], tm_scope : "source.solution", ace_mode : "text",
    language_id : 849523096u64, filenames : [], interpreters : [], }, MiniD => { name :
    "MiniD", r#type : "programming", color : "#000000", extensions : [".minid"], aliases
    : [], tm_scope : "none", ace_mode : "text", language_id : 231u64, filenames : [],
    interpreters : [], }, MiniYAML => { name : "MiniYAML", r#type : "data", color :
    "#ff1111", extensions : [".yaml", ".yml"], aliases : [], tm_scope :
    "source.miniyaml", ace_mode : "yaml", language_id : 4896465u64, codemirror_mode :
    "yaml", codemirror_mime_type : "text/x-yaml", filenames : [], interpreters : [], },
    MiniZinc => { name : "MiniZinc", r#type : "programming", color : "#06a9e6",
    extensions : [".mzn"], aliases : [], tm_scope : "source.mzn", ace_mode : "text",
    language_id : 238874535u64, filenames : [], interpreters : [], }, MiniZincData => {
    name : "MiniZinc Data", r#type : "data", color : "#000000", extensions : [".dzn"],
    aliases : [], tm_scope : "source.mzn", ace_mode : "text", language_id : 938193433u64,
    filenames : [], interpreters : [], }, Mint => { name : "Mint", r#type :
    "programming", color : "#02b046", extensions : [".mint"], aliases : [], tm_scope :
    "source.mint", ace_mode : "text", language_id : 968740319u64, filenames : [],
    interpreters : [], }, Mirah => { name : "Mirah", r#type : "programming", color :
    "#c7a938", extensions : [".druby", ".duby", ".mirah"], aliases : [], tm_scope :
    "source.ruby", ace_mode : "ruby", language_id : 232u64, codemirror_mode : "ruby",
    codemirror_mime_type : "text/x-ruby", filenames : [], interpreters : [], }, Modelica
    => { name : "Modelica", r#type : "programming", color : "#de1d31", extensions :
    [".mo"], aliases : [], tm_scope : "source.modelica", ace_mode : "text", language_id :
    233u64, codemirror_mode : "modelica", codemirror_mime_type : "text/x-modelica",
    filenames : [], interpreters : [], }, Modula2 => { name : "Modula-2", r#type :
    "programming", color : "#10253f", extensions : [".mod"], aliases : [], tm_scope :
    "source.modula2", ace_mode : "text", language_id : 234u64, filenames : [],
    interpreters : [], }, Modula3 => { name : "Modula-3", r#type : "programming", color :
    "#223388", extensions : [".i3", ".ig", ".m3", ".mg"], aliases : [], tm_scope :
    "source.modula-3", ace_mode : "text", language_id : 564743864u64, filenames : [],
    interpreters : [], }, ModuleManagementSystem => { name : "Module Management System",
    r#type : "programming", color : "#000000", extensions : [".mms", ".mmk"], aliases :
    [], tm_scope : "none", ace_mode : "text", language_id : 235u64, filenames :
    ["descrip.mmk", "descrip.mms"], interpreters : [], }, Mojo => { name : "Mojo", r#type
    : "programming", color : "#ff4c1f", extensions : [".mojo"], aliases : [], tm_scope :
    "source.mojo", ace_mode : "python", language_id : 1045019587u64, codemirror_mode :
    "python", codemirror_mime_type : "text/x-python", filenames : [], interpreters : [],
    }, Monkey => { name : "Monkey", r#type : "programming", color : "#000000", extensions
    : [".monkey", ".monkey2"], aliases : [], tm_scope : "source.monkey", ace_mode :
    "text", language_id : 236u64, filenames : [], interpreters : [], }, MonkeyC => { name
    : "Monkey C", r#type : "programming", color : "#8D6747", extensions : [".mc"],
    aliases : [], tm_scope : "source.mc", ace_mode : "c_cpp", language_id : 231751931u64,
    codemirror_mode : "clike", codemirror_mime_type : "text/x-csrc", filenames : [],
    interpreters : [], }, Moocode => { name : "Moocode", r#type : "programming", color :
    "#000000", extensions : [".moo"], aliases : [], tm_scope : "none", ace_mode : "text",
    language_id : 237u64, filenames : [], interpreters : [], }, MoonBit => { name :
    "MoonBit", r#type : "programming", color : "#b92381", extensions : [".mbt"], aliases
    : [], tm_scope : "source.moonbit", ace_mode : "text", language_id : 181453007u64,
    filenames : [], interpreters : [], }, MoonScript => { name : "MoonScript", r#type :
    "programming", color : "#ff4585", extensions : [".moon"], aliases : [], tm_scope :
    "source.moonscript", ace_mode : "text", language_id : 238u64, filenames : [],
    interpreters : ["moon"], }, Motoko => { name : "Motoko", r#type : "programming",
    color : "#fbb03b", extensions : [".mo"], aliases : [], tm_scope : "source.mo",
    ace_mode : "text", language_id : 202937027u64, filenames : [], interpreters : [], },
    Motorola68KAssembly => { name : "Motorola 68K Assembly", r#type : "programming",
    color : "#005daa", extensions : [".asm", ".i", ".inc", ".s", ".x68"], aliases :
    ["m68k"], tm_scope : "source.m68k", ace_mode : "assembly_x86", language_id :
    477582706u64, filenames : [], group : "Assembly", interpreters : [], }, MoveLang => {
    name : "Move", r#type : "programming", color : "#4a137a", extensions : [".move"],
    aliases : [], tm_scope : "source.move", ace_mode : "text", language_id :
    638334599u64, filenames : [], interpreters : [], }, Muse => { name : "Muse", r#type :
    "prose", color : "#000000", extensions : [".muse"], aliases : ["amusewiki",
    "emacs muse"], tm_scope : "text.muse", ace_mode : "text", language_id : 474864066u64,
    wrap : true, filenames : [], interpreters : [], }, Mustache => { name : "Mustache",
    r#type : "markup", color : "#724b3b", extensions : [".mustache"], aliases : [],
    tm_scope : "text.html.smarty", ace_mode : "smarty", language_id : 638334590u64,
    codemirror_mode : "smarty", codemirror_mime_type : "text/x-smarty", filenames : [],
    interpreters : [], }, Myghty => { name : "Myghty", r#type : "programming", color :
    "#000000", extensions : [".myt"], aliases : [], tm_scope : "none", ace_mode : "text",
    language_id : 239u64, filenames : [], interpreters : [], }, NASL => { name : "NASL",
    r#type : "programming", color : "#000000", extensions : [".nasl", ".inc"], aliases :
    [], tm_scope : "source.nasl", ace_mode : "text", language_id : 171666519u64,
    filenames : [], interpreters : [], }, NCL => { name : "NCL", r#type : "programming",
    color : "#28431f", extensions : [".ncl"], aliases : [], tm_scope : "source.ncl",
    ace_mode : "text", language_id : 240u64, filenames : [], interpreters : [], }, NEON
    => { name : "NEON", r#type : "data", color : "#000000", extensions : [".neon"],
    aliases : ["nette object notation", "ne-on"], tm_scope : "source.neon", ace_mode :
    "text", language_id : 481192983u64, filenames : [], interpreters : [], }, NL => {
    name : "NL", r#type : "data", color : "#000000", extensions : [".nl"], aliases : [],
    tm_scope : "none", ace_mode : "text", language_id : 241u64, filenames : [],
    interpreters : [], }, NMODL => { name : "NMODL", r#type : "programming", color :
    "#00356B", extensions : [".mod"], aliases : [], tm_scope : "none", ace_mode : "text",
    language_id : 136456478u64, filenames : [], interpreters : [], }, NPMConfig => { name
    : "NPM Config", r#type : "data", color : "#cb3837", extensions : [], aliases :
    ["npmrc"], tm_scope : "source.ini.npmrc", ace_mode : "text", language_id :
    685022663u64, filenames : [".npmrc"], group : "INI", interpreters : [], }, NSIS => {
    name : "NSIS", r#type : "programming", color : "#000000", extensions : [".nsi",
    ".nsh"], aliases : [], tm_scope : "source.nsis", ace_mode : "nsis", language_id :
    242u64, codemirror_mode : "nsis", codemirror_mime_type : "text/x-nsis", filenames :
    [], interpreters : [], }, NWScript => { name : "NWScript", r#type : "programming",
    color : "#111522", extensions : [".nss"], aliases : [], tm_scope :
    "source.c.nwscript", ace_mode : "c_cpp", language_id : 731233819u64, codemirror_mode
    : "clike", codemirror_mime_type : "text/x-csrc", filenames : [], interpreters : [],
    }, Nasal => { name : "Nasal", r#type : "programming", color : "#1d2c4e", extensions :
    [".nas"], aliases : [], tm_scope : "source.nasal", ace_mode : "nasal", language_id :
    178322513u64, filenames : [], interpreters : [], }, Nearley => { name : "Nearley",
    r#type : "programming", color : "#990000", extensions : [".ne", ".nearley"], aliases
    : [], tm_scope : "source.ne", ace_mode : "text", language_id : 521429430u64,
    filenames : [], interpreters : [], }, Nemerle => { name : "Nemerle", r#type :
    "programming", color : "#3d3c6e", extensions : [".n"], aliases : [], tm_scope :
    "source.nemerle", ace_mode : "text", language_id : 243u64, filenames : [],
    interpreters : [], }, NetLinx => { name : "NetLinx", r#type : "programming", color :
    "#0aa0ff", extensions : [".axs", ".axi"], aliases : [], tm_scope : "source.netlinx",
    ace_mode : "text", language_id : 244u64, filenames : [], interpreters : [], },
    NetLinxpERB => { name : "NetLinx+ERB", r#type : "programming", color : "#747faa",
    extensions : [".axs.erb", ".axi.erb"], aliases : [], tm_scope : "source.netlinx.erb",
    ace_mode : "text", language_id : 245u64, filenames : [], interpreters : [], },
    NetLogo => { name : "NetLogo", r#type : "programming", color : "#ff6375", extensions
    : [".nlogo"], aliases : [], tm_scope : "source.lisp", ace_mode : "lisp", language_id
    : 246u64, codemirror_mode : "commonlisp", codemirror_mime_type :
    "text/x-common-lisp", filenames : [], interpreters : [], }, NewLisp => { name :
    "NewLisp", r#type : "programming", color : "#87AED7", extensions : [".nl", ".lisp",
    ".lsp"], aliases : [], tm_scope : "source.lisp", ace_mode : "lisp", language_id :
    247u64, codemirror_mode : "commonlisp", codemirror_mime_type : "text/x-common-lisp",
    filenames : [], interpreters : ["newlisp"], }, Nextflow => { name : "Nextflow",
    r#type : "programming", color : "#3ac486", extensions : [".nf"], aliases : [],
    tm_scope : "source.nextflow", ace_mode : "groovy", language_id : 506780613u64,
    filenames : ["nextflow.config"], interpreters : ["nextflow"], }, Nginx => { name :
    "Nginx", r#type : "data", color : "#009639", extensions : [".nginx", ".nginxconf",
    ".vhost"], aliases : ["nginx configuration file"], tm_scope : "source.nginx",
    ace_mode : "nginx", language_id : 248u64, codemirror_mode : "nginx",
    codemirror_mime_type : "text/x-nginx-conf", filenames : ["nginx.conf"], interpreters
    : [], }, Nickel => { name : "Nickel", r#type : "programming", color : "#E0C3FC",
    extensions : [".ncl"], aliases : [], tm_scope : "source.nickel", ace_mode : "text",
    language_id : 1067292664u64, filenames : [], interpreters : [], }, Nim => { name :
    "Nim", r#type : "programming", color : "#ffc200", extensions : [".nim", ".nim.cfg",
    ".nimble", ".nimrod", ".nims"], aliases : [], tm_scope : "source.nim", ace_mode :
    "nim", language_id : 249u64, filenames : ["nim.cfg"], interpreters : [], }, Ninja =>
    { name : "Ninja", r#type : "data", color : "#000000", extensions : [".ninja"],
    aliases : [], tm_scope : "source.ninja", ace_mode : "text", language_id : 250u64,
    filenames : [], interpreters : [], }, Nit => { name : "Nit", r#type : "programming",
    color : "#009917", extensions : [".nit"], aliases : [], tm_scope : "source.nit",
    ace_mode : "text", language_id : 251u64, filenames : [], interpreters : [], }, Nix =>
    { name : "Nix", r#type : "programming", color : "#7e7eff", extensions : [".nix"],
    aliases : ["nixos"], tm_scope : "source.nix", ace_mode : "nix", language_id : 252u64,
    filenames : [], interpreters : [], }, Noir => { name : "Noir", r#type :
    "programming", color : "#2f1f49", extensions : [".nr"], aliases : ["nargo"], tm_scope
    : "source.nr", ace_mode : "rust", language_id : 813068465u64, codemirror_mode :
    "rust", codemirror_mime_type : "text/x-rustsrc", filenames : [], interpreters : [],
    }, Nu => { name : "Nu", r#type : "programming", color : "#c9df40", extensions :
    [".nu"], aliases : ["nush"], tm_scope : "source.nu", ace_mode : "scheme", language_id
    : 253u64, codemirror_mode : "scheme", codemirror_mime_type : "text/x-scheme",
    filenames : ["Nukefile"], interpreters : ["nush"], }, NumPy => { name : "NumPy",
    r#type : "programming", color : "#9C8AF9", extensions : [".numpy", ".numpyw",
    ".numsc"], aliases : [], tm_scope : "none", ace_mode : "text", language_id : 254u64,
    codemirror_mode : "python", codemirror_mime_type : "text/x-python", filenames : [],
    group : "Python", interpreters : [], }, Nunjucks => { name : "Nunjucks", r#type :
    "markup", color : "#3d8137", extensions : [".njk"], aliases : ["njk"], tm_scope :
    "text.html.nunjucks", ace_mode : "nunjucks", language_id : 461856962u64, filenames :
    [], interpreters : [], }, Nushell => { name : "Nushell", r#type : "programming",
    color : "#4E9906", extensions : [".nu"], aliases : ["nu-script", "nushell-script"],
    tm_scope : "source.nushell", ace_mode : "sh", language_id : 446573572u64,
    codemirror_mode : "shell", codemirror_mime_type : "text/x-sh", filenames : [],
    interpreters : ["nu"], }, OASv2Json => { name : "OASv2-json", r#type : "data", color
    : "#85ea2d", extensions : [".json"], aliases : [], tm_scope : "source.json", ace_mode
    : "json", language_id : 834374816u64, codemirror_mode : "javascript",
    codemirror_mime_type : "application/json", filenames : [], group :
    "OpenAPI Specification v2", interpreters : [], }, OASv2Yaml => { name : "OASv2-yaml",
    r#type : "data", color : "#85ea2d", extensions : [".yaml", ".yml"], aliases : [],
    tm_scope : "source.yaml", ace_mode : "yaml", language_id : 105187618u64,
    codemirror_mode : "yaml", codemirror_mime_type : "text/x-yaml", filenames : [], group
    : "OpenAPI Specification v2", interpreters : [], }, OASv3Json => { name :
    "OASv3-json", r#type : "data", color : "#85ea2d", extensions : [".json"], aliases :
    [], tm_scope : "source.json", ace_mode : "json", language_id : 980062566u64,
    codemirror_mode : "javascript", codemirror_mime_type : "application/json", filenames
    : [], group : "OpenAPI Specification v3", interpreters : [], }, OASv3Yaml => { name :
    "OASv3-yaml", r#type : "data", color : "#85ea2d", extensions : [".yaml", ".yml"],
    aliases : [], tm_scope : "source.yaml", ace_mode : "yaml", language_id : 51239111u64,
    codemirror_mode : "yaml", codemirror_mime_type : "text/x-yaml", filenames : [], group
    : "OpenAPI Specification v3", interpreters : [], }, OCaml => { name : "OCaml", r#type
    : "programming", color : "#ef7a08", extensions : [".ml", ".eliom", ".eliomi", ".ml4",
    ".mli", ".mll", ".mly"], aliases : [], tm_scope : "source.ocaml", ace_mode : "ocaml",
    language_id : 255u64, codemirror_mode : "mllike", codemirror_mime_type :
    "text/x-ocaml", filenames : [], interpreters : ["ocaml", "ocamlrun", "ocamlscript"],
    }, OMNeTppMSG => { name : "OMNeT++ MSG", r#type : "programming", color : "#a0e0a0",
    extensions : [".msg"], aliases : ["omnetpp-msg"], tm_scope : "source.msg", ace_mode :
    "text", language_id : 664100008u64, filenames : [], interpreters : [], }, OMNeTppNED
    => { name : "OMNeT++ NED", r#type : "programming", color : "#08607c", extensions :
    [".ned"], aliases : ["omnetpp-ned"], tm_scope : "source.ned", ace_mode : "text",
    language_id : 924868392u64, filenames : [], interpreters : [], }, Oberon => { name :
    "Oberon", r#type : "programming", color : "#000000", extensions : [".ob2"], aliases :
    [], tm_scope : "source.modula2", ace_mode : "text", language_id : 677210597u64,
    filenames : [], interpreters : [], }, ObjDump => { name : "ObjDump", r#type : "data",
    color : "#000000", extensions : [".objdump"], aliases : [], tm_scope :
    "objdump.x86asm", ace_mode : "assembly_x86", language_id : 256u64, filenames : [],
    interpreters : [], }, ObjectDataInstanceNotation => { name :
    "Object Data Instance Notation", r#type : "data", color : "#000000", extensions :
    [".odin"], aliases : [], tm_scope : "source.odin-ehr", ace_mode : "text", language_id
    : 985227236u64, filenames : [], interpreters : [], }, ObjectScript => { name :
    "ObjectScript", r#type : "programming", color : "#424893", extensions : [".cls"],
    aliases : [], tm_scope : "source.objectscript", ace_mode : "text", language_id :
    202735509u64, filenames : [], interpreters : [], }, ObjectiveC => { name :
    "Objective-C", r#type : "programming", color : "#438eff", extensions : [".m", ".h"],
    aliases : ["obj-c", "objc", "objectivec"], tm_scope : "source.objc", ace_mode :
    "objectivec", language_id : 257u64, codemirror_mode : "clike", codemirror_mime_type :
    "text/x-objectivec", filenames : [], interpreters : [], }, ObjectiveCpp => { name :
    "Objective-C++", r#type : "programming", color : "#6866fb", extensions : [".mm"],
    aliases : ["obj-c++", "objc++", "objectivec++"], tm_scope : "source.objc++", ace_mode
    : "objectivec", language_id : 258u64, codemirror_mode : "clike", codemirror_mime_type
    : "text/x-objectivec++", filenames : [], interpreters : [], }, ObjectiveJ => { name :
    "Objective-J", r#type : "programming", color : "#ff0c5a", extensions : [".j", ".sj"],
    aliases : ["obj-j", "objectivej", "objj"], tm_scope : "source.js.objj", ace_mode :
    "text", language_id : 259u64, filenames : [], interpreters : [], }, Odin => { name :
    "Odin", r#type : "programming", color : "#60AFFE", extensions : [".odin"], aliases :
    ["odinlang", "odin-lang"], tm_scope : "source.odin", ace_mode : "odin", language_id :
    889244082u64, filenames : [], interpreters : [], }, Omgrofl => { name : "Omgrofl",
    r#type : "programming", color : "#cabbff", extensions : [".omgrofl"], aliases : [],
    tm_scope : "none", ace_mode : "text", language_id : 260u64, filenames : [],
    interpreters : [], }, Opa => { name : "Opa", r#type : "programming", color :
    "#000000", extensions : [".opa"], aliases : [], tm_scope : "source.opa", ace_mode :
    "text", language_id : 261u64, filenames : [], interpreters : [], }, Opal => { name :
    "Opal", r#type : "programming", color : "#f7ede0", extensions : [".opal"], aliases :
    [], tm_scope : "source.opal", ace_mode : "text", language_id : 262u64, filenames :
    [], interpreters : [], }, OpenPolicyAgent => { name : "Open Policy Agent", r#type :
    "programming", color : "#7d9199", extensions : [".rego"], aliases : [], tm_scope :
    "source.rego", ace_mode : "text", language_id : 840483232u64, filenames : [],
    interpreters : [], }, OpenAPISpecificationV2 => { name : "OpenAPI Specification v2",
    r#type : "data", color : "#85ea2d", extensions : [], aliases : ["oasv2"], tm_scope :
    "none", ace_mode : "text", language_id : 848295328u64, filenames : [], interpreters :
    [], }, OpenAPISpecificationV3 => { name : "OpenAPI Specification v3", r#type :
    "data", color : "#85ea2d", extensions : [], aliases : ["oasv3"], tm_scope : "none",
    ace_mode : "text", language_id : 557959099u64, filenames : [], interpreters : [], },
    OpenCL => { name : "OpenCL", r#type : "programming", color : "#ed2e2d", extensions :
    [".cl", ".opencl"], aliases : [], tm_scope : "source.c", ace_mode : "c_cpp",
    language_id : 263u64, codemirror_mode : "clike", codemirror_mime_type :
    "text/x-csrc", filenames : [], group : "C", interpreters : [], }, OpenEdgeABL => {
    name : "OpenEdge ABL", r#type : "programming", color : "#5ce600", extensions : [".p",
    ".cls", ".w"], aliases : ["progress", "openedge", "abl"], tm_scope : "source.abl",
    ace_mode : "text", language_id : 264u64, filenames : [], interpreters : [], },
    OpenQASM => { name : "OpenQASM", r#type : "programming", color : "#AA70FF",
    extensions : [".qasm"], aliases : [], tm_scope : "source.qasm", ace_mode : "text",
    language_id : 153739399u64, filenames : [], interpreters : [], }, OpenRCRunscript =>
    { name : "OpenRC runscript", r#type : "programming", color : "#000000", extensions :
    [], aliases : ["openrc"], tm_scope : "source.shell", ace_mode : "sh", language_id :
    265u64, codemirror_mode : "shell", codemirror_mime_type : "text/x-sh", filenames :
    [], group : "Shell", interpreters : ["openrc-run"], }, OpenSCAD => { name :
    "OpenSCAD", r#type : "programming", color : "#e5cd45", extensions : [".scad"],
    aliases : [], tm_scope : "source.scad", ace_mode : "scad", language_id : 266u64,
    filenames : [], interpreters : [], }, OpenStepPropertyList => { name :
    "OpenStep Property List", r#type : "data", color : "#000000", extensions : [".plist",
    ".glyphs"], aliases : [], tm_scope : "source.plist", ace_mode : "text", language_id :
    598917541u64, filenames : [], interpreters : [], }, OpenTypeFeatureFile => { name :
    "OpenType Feature File", r#type : "data", color : "#000000", extensions : [".fea"],
    aliases : ["AFDKO"], tm_scope : "source.opentype", ace_mode : "text", language_id :
    374317347u64, filenames : [], interpreters : [], }, OptionList => { name :
    "Option List", r#type : "data", color : "#476732", extensions : [], aliases :
    ["opts", "ackrc"], tm_scope : "source.opts", ace_mode : "sh", language_id :
    723589315u64, codemirror_mode : "shell", codemirror_mime_type : "text/x-sh",
    filenames : [".ackrc", ".rspec", ".yardopts", "ackrc", "mocha.opts"], interpreters :
    [], }, Org => { name : "Org", r#type : "prose", color : "#77aa99", extensions :
    [".org"], aliases : [], tm_scope : "none", ace_mode : "text", language_id : 267u64,
    wrap : true, filenames : [], interpreters : [], }, OverpassQL => { name :
    "OverpassQL", r#type : "programming", color : "#cce2aa", extensions :
    [".overpassql"], aliases : [], tm_scope : "source.overpassql", ace_mode : "text",
    language_id : 689079655u64, wrap : true, filenames : [], interpreters : [], }, Ox =>
    { name : "Ox", r#type : "programming", color : "#000000", extensions : [".ox",
    ".oxh", ".oxo"], aliases : [], tm_scope : "source.ox", ace_mode : "text", language_id
    : 268u64, filenames : [], interpreters : [], }, Oxygene => { name : "Oxygene", r#type
    : "programming", color : "#cdd0e3", extensions : [".oxygene"], aliases : [], tm_scope
    : "none", ace_mode : "text", language_id : 269u64, filenames : [], interpreters : [],
    }, Oz => { name : "Oz", r#type : "programming", color : "#fab738", extensions :
    [".oz"], aliases : [], tm_scope : "source.oz", ace_mode : "text", language_id :
    270u64, codemirror_mode : "oz", codemirror_mime_type : "text/x-oz", filenames : [],
    interpreters : [], }, P4 => { name : "P4", r#type : "programming", color : "#7055b5",
    extensions : [".p4"], aliases : [], tm_scope : "source.p4", ace_mode : "text",
    language_id : 348895984u64, filenames : [], interpreters : [], }, PDDL => { name :
    "PDDL", r#type : "programming", color : "#0d00ff", extensions : [".pddl"], aliases :
    [], tm_scope : "source.pddl", ace_mode : "text", language_id : 736235603u64,
    filenames : [], interpreters : [], }, PEGjs => { name : "PEG.js", r#type :
    "programming", color : "#234d6b", extensions : [".pegjs", ".peggy"], aliases : [],
    tm_scope : "source.peggy", ace_mode : "javascript", language_id : 81442128u64,
    codemirror_mode : "javascript", codemirror_mime_type : "text/javascript", filenames :
    [], interpreters : [], }, PHP => { name : "PHP", r#type : "programming", color :
    "#4F5D95", extensions : [".php", ".aw", ".ctp", ".fcgi", ".inc", ".php3", ".php4",
    ".php5", ".phps", ".phpt"], aliases : ["inc"], tm_scope : "text.html.php", ace_mode :
    "php", language_id : 272u64, codemirror_mode : "php", codemirror_mime_type :
    "application/x-httpd-php", filenames : [".php", ".php_cs", ".php_cs.dist",
    "Phakefile"], interpreters : ["php"], }, PLSQL => { name : "PLSQL", r#type :
    "programming", color : "#dad8d8", extensions : [".pls", ".bdy", ".ddl", ".fnc",
    ".pck", ".pkb", ".pks", ".plb", ".plsql", ".prc", ".spc", ".sql", ".tpb", ".tps",
    ".trg", ".vw"], aliases : [], tm_scope : "none", ace_mode : "plsql", language_id :
    273u64, codemirror_mode : "sql", codemirror_mime_type : "text/x-plsql", filenames :
    [], interpreters : [], }, PLpgSQL => { name : "PLpgSQL", r#type : "programming",
    color : "#336790", extensions : [".pgsql", ".sql"], aliases : [], tm_scope :
    "source.sql", ace_mode : "pgsql", language_id : 274u64, codemirror_mode : "sql",
    codemirror_mime_type : "text/x-sql", filenames : [], interpreters : [], }, POVRaySDL
    => { name : "POV-Ray SDL", r#type : "programming", color : "#6bac65", extensions :
    [".pov", ".inc"], aliases : ["pov-ray", "povray"], tm_scope : "source.pov-ray sdl",
    ace_mode : "text", language_id : 275u64, filenames : [], interpreters : [], }, Pact
    => { name : "Pact", r#type : "programming", color : "#F7A8B8", extensions :
    [".pact"], aliases : [], tm_scope : "source.pact", ace_mode : "text", language_id :
    756774415u64, filenames : [], interpreters : [], }, Pan => { name : "Pan", r#type :
    "programming", color : "#cc0000", extensions : [".pan"], aliases : [], tm_scope :
    "source.pan", ace_mode : "text", language_id : 276u64, filenames : [], interpreters :
    [], }, Papyrus => { name : "Papyrus", r#type : "programming", color : "#6600cc",
    extensions : [".psc"], aliases : [], tm_scope : "source.papyrus.skyrim", ace_mode :
    "text", language_id : 277u64, filenames : [], interpreters : [], }, Parrot => { name
    : "Parrot", r#type : "programming", color : "#f3ca0a", extensions : [".parrot"],
    aliases : [], tm_scope : "none", ace_mode : "text", language_id : 278u64, filenames :
    [], interpreters : [], }, ParrotAssembly => { name : "Parrot Assembly", r#type :
    "programming", color : "#000000", extensions : [".pasm"], aliases : ["pasm"],
    tm_scope : "none", ace_mode : "text", language_id : 279u64, filenames : [], group :
    "Parrot", interpreters : ["parrot"], }, ParrotInternalRepresentation => { name :
    "Parrot Internal Representation", r#type : "programming", color : "#000000",
    extensions : [".pir"], aliases : ["pir"], tm_scope : "source.parrot.pir", ace_mode :
    "text", language_id : 280u64, filenames : [], group : "Parrot", interpreters :
    ["parrot"], }, Pascal => { name : "Pascal", r#type : "programming", color :
    "#E3F171", extensions : [".pas", ".dfm", ".dpr", ".inc", ".lpr", ".pascal", ".pp"],
    aliases : ["delphi", "objectpascal"], tm_scope : "source.pascal", ace_mode :
    "pascal", language_id : 281u64, codemirror_mode : "pascal", codemirror_mime_type :
    "text/x-pascal", filenames : [], interpreters : ["instantfpc"], }, Pawn => { name :
    "Pawn", r#type : "programming", color : "#dbb284", extensions : [".pwn", ".inc",
    ".sma"], aliases : [], tm_scope : "source.pawn", ace_mode : "text", language_id :
    271u64, filenames : [], interpreters : [], }, Pep8 => { name : "Pep8", r#type :
    "programming", color : "#C76F5B", extensions : [".pep"], aliases : [], tm_scope :
    "source.pep8", ace_mode : "text", language_id : 840372442u64, filenames : [],
    interpreters : [], }, Perl => { name : "Perl", r#type : "programming", color :
    "#0298c3", extensions : [".pl", ".al", ".cgi", ".fcgi", ".perl", ".ph", ".plx",
    ".pm", ".psgi", ".t"], aliases : ["cperl"], tm_scope : "source.perl", ace_mode :
    "perl", language_id : 282u64, codemirror_mode : "perl", codemirror_mime_type :
    "text/x-perl", filenames : [".latexmkrc", "Makefile.PL", "Rexfile", "ack",
    "cpanfile", "latexmkrc"], interpreters : ["cperl", "perl"], }, Pic => { name : "Pic",
    r#type : "markup", color : "#000000", extensions : [".pic", ".chem"], aliases :
    ["pikchr"], tm_scope : "source.pic", ace_mode : "text", language_id : 425u64,
    codemirror_mode : "troff", codemirror_mime_type : "text/troff", filenames : [], group
    : "Roff", interpreters : [], }, Pickle => { name : "Pickle", r#type : "data", color :
    "#000000", extensions : [".pkl"], aliases : [], tm_scope : "none", ace_mode : "text",
    language_id : 284u64, filenames : [], interpreters : [], }, PicoLisp => { name :
    "PicoLisp", r#type : "programming", color : "#6067af", extensions : [".l"], aliases :
    [], tm_scope : "source.lisp", ace_mode : "lisp", language_id : 285u64, filenames :
    [], interpreters : ["picolisp", "pil"], }, PigLatin => { name : "PigLatin", r#type :
    "programming", color : "#fcd7de", extensions : [".pig"], aliases : [], tm_scope :
    "source.pig_latin", ace_mode : "pig", language_id : 286u64, codemirror_mode : "pig",
    codemirror_mime_type : "text/x-pig", filenames : [], interpreters : [], }, Pike => {
    name : "Pike", r#type : "programming", color : "#005390", extensions : [".pike",
    ".pmod"], aliases : [], tm_scope : "source.pike", ace_mode : "text", language_id :
    287u64, filenames : [], interpreters : ["pike"], }, PipRequirements => { name :
    "Pip Requirements", r#type : "data", color : "#FFD343", extensions : [], aliases :
    [], tm_scope : "source.pip-requirements", ace_mode : "text", language_id :
    684385621u64, filenames : ["dev-requirements.txt", "requirements-dev.txt",
    "requirements.lock.txt", "requirements.txt"], interpreters : [], }, Pkl => { name :
    "Pkl", r#type : "programming", color : "#6b9543", extensions : [".pkl"], aliases :
    [], tm_scope : "source.pkl", ace_mode : "text", language_id : 288822799u64, filenames
    : [], interpreters : ["pkl"], }, PlantUML => { name : "PlantUML", r#type : "data",
    color : "#fbbd16", extensions : [".puml", ".iuml", ".plantuml"], aliases : [],
    tm_scope : "source.wsd", ace_mode : "text", language_id : 833504686u64, filenames :
    [], interpreters : [], }, Pod => { name : "Pod", r#type : "prose", color : "#000000",
    extensions : [".pod"], aliases : [], tm_scope : "none", ace_mode : "perl",
    language_id : 288u64, codemirror_mode : "perl", codemirror_mime_type : "text/x-perl",
    wrap : true, filenames : [], interpreters : ["perl"], }, Pod6 => { name : "Pod 6",
    r#type : "prose", color : "#000000", extensions : [".pod", ".pod6"], aliases : [],
    tm_scope : "source.raku", ace_mode : "perl", language_id : 155357471u64, wrap : true,
    filenames : [], interpreters : ["perl6"], }, PogoScript => { name : "PogoScript",
    r#type : "programming", color : "#d80074", extensions : [".pogo"], aliases : [],
    tm_scope : "source.pogoscript", ace_mode : "text", language_id : 289u64, filenames :
    [], interpreters : [], }, Polar => { name : "Polar", r#type : "programming", color :
    "#ae81ff", extensions : [".polar"], aliases : [], tm_scope : "source.polar", ace_mode
    : "text", language_id : 839112914u64, filenames : [], interpreters : [], }, Pony => {
    name : "Pony", r#type : "programming", color : "#000000", extensions : [".pony"],
    aliases : [], tm_scope : "source.pony", ace_mode : "text", language_id : 290u64,
    filenames : [], interpreters : [], }, Portugol => { name : "Portugol", r#type :
    "programming", color : "#f8bd00", extensions : [".por"], aliases : [], tm_scope :
    "source.portugol", ace_mode : "text", language_id : 832391833u64, filenames : [],
    interpreters : [], }, PostCSS => { name : "PostCSS", r#type : "markup", color :
    "#dc3a0c", extensions : [".pcss", ".postcss"], aliases : [], tm_scope :
    "source.postcss", ace_mode : "text", language_id : 262764437u64, filenames : [],
    group : "CSS", interpreters : [], }, PostScript => { name : "PostScript", r#type :
    "markup", color : "#da291c", extensions : [".ps", ".eps", ".epsi", ".pfa"], aliases :
    ["postscr"], tm_scope : "source.postscript", ace_mode : "text", language_id : 291u64,
    filenames : [], interpreters : [], }, PowerBuilder => { name : "PowerBuilder", r#type
    : "programming", color : "#8f0f8d", extensions : [".pbt", ".sra", ".sru", ".srw"],
    aliases : [], tm_scope : "source.powerbuilder", ace_mode : "text", language_id :
    292u64, filenames : [], interpreters : [], }, PowerShell => { name : "PowerShell",
    r#type : "programming", color : "#012456", extensions : [".ps1", ".psd1", ".psm1"],
    aliases : ["posh", "pwsh"], tm_scope : "source.powershell", ace_mode : "powershell",
    language_id : 293u64, codemirror_mode : "powershell", codemirror_mime_type :
    "application/x-powershell", filenames : [], interpreters : ["pwsh"], }, Praat => {
    name : "Praat", r#type : "programming", color : "#c8506d", extensions : [".praat"],
    aliases : [], tm_scope : "source.praat", ace_mode : "praat", language_id :
    106029007u64, filenames : [], interpreters : [], }, Prisma => { name : "Prisma",
    r#type : "data", color : "#0c344b", extensions : [".prisma"], aliases : [], tm_scope
    : "source.prisma", ace_mode : "prisma", language_id : 499933428u64, filenames : [],
    interpreters : [], }, Processing => { name : "Processing", r#type : "programming",
    color : "#0096D8", extensions : [".pde"], aliases : [], tm_scope :
    "source.processing", ace_mode : "text", language_id : 294u64, filenames : [],
    interpreters : [], }, Procfile => { name : "Procfile", r#type : "programming", color
    : "#3B2F63", extensions : [], aliases : [], tm_scope : "source.procfile", ace_mode :
    "batchfile", language_id : 305313959u64, filenames : ["Procfile"], interpreters : [],
    }, Proguard => { name : "Proguard", r#type : "data", color : "#000000", extensions :
    [".pro"], aliases : [], tm_scope : "none", ace_mode : "text", language_id :
    716513858u64, filenames : [], interpreters : [], }, Prolog => { name : "Prolog",
    r#type : "programming", color : "#74283c", extensions : [".pl", ".plt", ".pro",
    ".prolog", ".yap"], aliases : [], tm_scope : "source.prolog", ace_mode : "prolog",
    language_id : 295u64, filenames : [], interpreters : ["swipl", "yap"], }, Promela =>
    { name : "Promela", r#type : "programming", color : "#de0000", extensions : [".pml"],
    aliases : [], tm_scope : "source.promela", ace_mode : "text", language_id :
    441858312u64, filenames : [], interpreters : [], }, PropellerSpin => { name :
    "Propeller Spin", r#type : "programming", color : "#7fa2a7", extensions : [".spin"],
    aliases : [], tm_scope : "source.spin", ace_mode : "text", language_id : 296u64,
    filenames : [], interpreters : [], }, ProtocolBuffer => { name : "Protocol Buffer",
    r#type : "data", color : "#000000", extensions : [".proto"], aliases : ["proto",
    "protobuf", "Protocol Buffers"], tm_scope : "source.proto", ace_mode : "protobuf",
    language_id : 297u64, codemirror_mode : "protobuf", codemirror_mime_type :
    "text/x-protobuf", filenames : [], interpreters : [], }, ProtocolBufferTextFormat =>
    { name : "Protocol Buffer Text Format", r#type : "data", color : "#000000",
    extensions : [".textproto", ".pbt", ".pbtxt"], aliases : ["text proto",
    "protobuf text format"], tm_scope : "source.textproto", ace_mode : "text",
    language_id : 436568854u64, filenames : [], interpreters : [], }, PublicKey => { name
    : "Public Key", r#type : "data", color : "#000000", extensions : [".asc", ".pub"],
    aliases : [], tm_scope : "none", ace_mode : "text", language_id : 298u64,
    codemirror_mode : "asciiarmor", codemirror_mime_type : "application/pgp", filenames :
    [], interpreters : [], }, Pug => { name : "Pug", r#type : "markup", color :
    "#a86454", extensions : [".jade", ".pug"], aliases : [], tm_scope : "text.jade",
    ace_mode : "jade", language_id : 179u64, codemirror_mode : "pug",
    codemirror_mime_type : "text/x-pug", filenames : [], interpreters : [], }, Puppet =>
    { name : "Puppet", r#type : "programming", color : "#302B6D", extensions : [".pp"],
    aliases : [], tm_scope : "source.puppet", ace_mode : "puppet", language_id : 299u64,
    codemirror_mode : "puppet", codemirror_mime_type : "text/x-puppet", filenames :
    ["Modulefile"], interpreters : [], }, PureData => { name : "Pure Data", r#type :
    "data", color : "#000000", extensions : [".pd"], aliases : [], tm_scope : "none",
    ace_mode : "text", language_id : 300u64, filenames : [], interpreters : [], },
    PureBasic => { name : "PureBasic", r#type : "programming", color : "#5a6986",
    extensions : [".pb", ".pbi"], aliases : [], tm_scope : "none", ace_mode : "text",
    language_id : 301u64, filenames : [], interpreters : [], }, PureScript => { name :
    "PureScript", r#type : "programming", color : "#1D222D", extensions : [".purs"],
    aliases : [], tm_scope : "source.purescript", ace_mode : "haskell", language_id :
    302u64, codemirror_mode : "haskell", codemirror_mime_type : "text/x-haskell",
    filenames : [], interpreters : [], }, Pyret => { name : "Pyret", r#type :
    "programming", color : "#ee1e10", extensions : [".arr"], aliases : [], tm_scope :
    "source.arr", ace_mode : "python", language_id : 252961827u64, filenames : [],
    interpreters : [], }, Python => { name : "Python", r#type : "programming", color :
    "#3572A5", extensions : [".py", ".cgi", ".fcgi", ".gyp", ".gypi", ".lmi", ".py3",
    ".pyde", ".pyi", ".pyp", ".pyt", ".pyw", ".rpy", ".spec", ".tac", ".wsgi", ".xpy"],
    aliases : ["python3", "rusthon"], tm_scope : "source.python", ace_mode : "python",
    language_id : 303u64, codemirror_mode : "python", codemirror_mime_type :
    "text/x-python", filenames : [".gclient", "DEPS", "SConscript", "SConstruct",
    "wscript"], interpreters : ["python", "python2", "python3", "py", "pypy", "pypy3",
    "uv"], }, PythonConsole => { name : "Python console", r#type : "programming", color :
    "#3572A5", extensions : [], aliases : ["pycon"], tm_scope : "text.python.console",
    ace_mode : "text", language_id : 428u64, filenames : [], group : "Python",
    interpreters : [], }, PythonTraceback => { name : "Python traceback", r#type :
    "data", color : "#3572A5", extensions : [".pytb"], aliases : [], tm_scope :
    "text.python.traceback", ace_mode : "text", language_id : 304u64, filenames : [],
    group : "Python", interpreters : [], }, Qsharp => { name : "Q#", r#type :
    "programming", color : "#fed659", extensions : [".qs"], aliases : ["qsharp"],
    tm_scope : "source.qsharp", ace_mode : "text", language_id : 697448245u64, filenames
    : [], interpreters : [], }, QML => { name : "QML", r#type : "programming", color :
    "#44a51c", extensions : [".qml", ".qbs"], aliases : [], tm_scope : "source.qml",
    ace_mode : "qml", language_id : 305u64, filenames : [], interpreters : [], }, QMake
    => { name : "QMake", r#type : "programming", color : "#000000", extensions : [".pro",
    ".pri"], aliases : [], tm_scope : "source.qmake", ace_mode : "text", language_id :
    306u64, filenames : [], interpreters : ["qmake"], }, QtScript => { name :
    "Qt Script", r#type : "programming", color : "#00b841", extensions : [".qs"], aliases
    : [], tm_scope : "source.js", ace_mode : "javascript", language_id : 558193693u64,
    codemirror_mode : "javascript", codemirror_mime_type : "text/javascript", filenames :
    ["installscript.qs", "toolchain_installscript.qs"], interpreters : [], }, Quake => {
    name : "Quake", r#type : "programming", color : "#882233", extensions : [], aliases :
    [], tm_scope : "source.quake", ace_mode : "text", language_id : 375265331u64,
    filenames : ["m3makefile", "m3overrides"], interpreters : [], }, QuakeC => { name :
    "QuakeC", r#type : "programming", color : "#975777", extensions : [".qc"], aliases :
    [], tm_scope : "source.quakec", ace_mode : "text", language_id : 472308069u64,
    filenames : [], interpreters : [], }, QuickBASIC => { name : "QuickBASIC", r#type :
    "programming", color : "#008080", extensions : [".bas", ".bi"], aliases : ["qb",
    "qbasic", "qb64", "classic qbasic", "classic quickbasic"], tm_scope : "source.QB64",
    ace_mode : "text", language_id : 593107205u64, codemirror_mode : "vb",
    codemirror_mime_type : "text/x-vb", filenames : [], interpreters : [], }, R => { name
    : "R", r#type : "programming", color : "#198CE7", extensions : [".r", ".rd", ".rsx"],
    aliases : ["Rscript", "splus"], tm_scope : "source.r", ace_mode : "r", language_id :
    307u64, codemirror_mode : "r", codemirror_mime_type : "text/x-rsrc", filenames :
    [".Rprofile", "expr-dist"], interpreters : ["Rscript"], }, RAML => { name : "RAML",
    r#type : "markup", color : "#77d9fb", extensions : [".raml"], aliases : [], tm_scope
    : "source.yaml", ace_mode : "yaml", language_id : 308u64, codemirror_mode : "yaml",
    codemirror_mime_type : "text/x-yaml", filenames : [], interpreters : [], }, RBS => {
    name : "RBS", r#type : "data", color : "#701516", extensions : [".rbs"], aliases :
    [], tm_scope : "source.rbs", ace_mode : "ruby", language_id : 899227493u64,
    codemirror_mode : "ruby", codemirror_mime_type : "text/x-ruby", filenames : [], group
    : "Ruby", interpreters : [], }, RDoc => { name : "RDoc", r#type : "prose", color :
    "#701516", extensions : [".rdoc"], aliases : [], tm_scope : "text.rdoc", ace_mode :
    "rdoc", language_id : 309u64, wrap : true, filenames : [], interpreters : [], },
    REALbasic => { name : "REALbasic", r#type : "programming", color : "#000000",
    extensions : [".rbbas", ".rbfrm", ".rbmnu", ".rbres", ".rbtbar", ".rbuistate"],
    aliases : [], tm_scope : "source.vbnet", ace_mode : "text", language_id : 310u64,
    filenames : [], interpreters : [], }, REXX => { name : "REXX", r#type :
    "programming", color : "#d90e09", extensions : [".rexx", ".pprx", ".rex"], aliases :
    ["arexx"], tm_scope : "source.rexx", ace_mode : "text", language_id : 311u64,
    filenames : [], interpreters : ["regina", "rexx"], }, RMarkdown => { name :
    "RMarkdown", r#type : "prose", color : "#198ce7", extensions : [".qmd", ".rmd"],
    aliases : [], tm_scope : "text.md", ace_mode : "markdown", language_id : 313u64,
    codemirror_mode : "gfm", codemirror_mime_type : "text/x-gfm", wrap : true, filenames
    : [], interpreters : [], }, RON => { name : "RON", r#type : "data", color :
    "#a62c00", extensions : [".ron"], aliases : [], tm_scope : "source.ron", ace_mode :
    "rust", language_id : 587855233u64, filenames : [], interpreters : [], },
    ROSInterface => { name : "ROS Interface", r#type : "data", color : "#22314e",
    extensions : [".msg", ".action", ".srv"], aliases : ["rosmsg"], tm_scope :
    "source.rosmsg", ace_mode : "text", language_id : 809230569u64, filenames : [],
    interpreters : [], }, RPC => { name : "RPC", r#type : "programming", color :
    "#000000", extensions : [".x"], aliases : ["rpcgen", "oncrpc", "xdr"], tm_scope :
    "source.c", ace_mode : "c_cpp", language_id : 1031374237u64, filenames : [],
    interpreters : [], }, RPGLE => { name : "RPGLE", r#type : "programming", color :
    "#2BDE21", extensions : [".rpgle", ".sqlrpgle"], aliases : ["ile rpg", "sqlrpgle"],
    tm_scope : "source.rpgle", ace_mode : "text", language_id : 609977990u64, filenames :
    [], interpreters : [], }, RPMSpec => { name : "RPM Spec", r#type : "data", color :
    "#000000", extensions : [".spec"], aliases : ["specfile"], tm_scope :
    "source.rpm-spec", ace_mode : "text", language_id : 314u64, codemirror_mode : "rpm",
    codemirror_mime_type : "text/x-rpm-spec", filenames : [], interpreters : [], },
    RUNOFF => { name : "RUNOFF", r#type : "markup", color : "#665a4e", extensions :
    [".rnh", ".rno"], aliases : [], tm_scope : "text.runoff", ace_mode : "text",
    language_id : 315u64, wrap : true, filenames : [], interpreters : [], }, Racket => {
    name : "Racket", r#type : "programming", color : "#3c5caa", extensions : [".rkt",
    ".rktd", ".rktl", ".scrbl"], aliases : [], tm_scope : "source.racket", ace_mode :
    "lisp", language_id : 316u64, filenames : [], interpreters : ["racket"], }, Ragel =>
    { name : "Ragel", r#type : "programming", color : "#9d5200", extensions : [".rl"],
    aliases : ["ragel-rb", "ragel-ruby"], tm_scope : "none", ace_mode : "text",
    language_id : 317u64, filenames : [], interpreters : [], }, Raku => { name : "Raku",
    r#type : "programming", color : "#0000fb", extensions : [".6pl", ".6pm", ".nqp",
    ".p6", ".p6l", ".p6m", ".pl", ".pl6", ".pm", ".pm6", ".raku", ".rakumod", ".t"],
    aliases : ["perl6", "perl-6"], tm_scope : "source.raku", ace_mode : "raku",
    language_id : 283u64, codemirror_mode : "perl", codemirror_mime_type : "text/x-perl",
    filenames : [], interpreters : ["perl6", "raku", "rakudo"], }, Rascal => { name :
    "Rascal", r#type : "programming", color : "#fffaa0", extensions : [".rsc"], aliases :
    [], tm_scope : "source.rascal", ace_mode : "text", language_id : 173616037u64,
    filenames : [], interpreters : [], }, RawTokenData => { name : "Raw token data",
    r#type : "data", color : "#000000", extensions : [".raw"], aliases : ["raw"],
    tm_scope : "none", ace_mode : "text", language_id : 318u64, filenames : [],
    interpreters : [], }, ReScript => { name : "ReScript", r#type : "programming", color
    : "#ed5051", extensions : [".res", ".resi"], aliases : [], tm_scope :
    "source.rescript", ace_mode : "rust", language_id : 501875647u64, codemirror_mode :
    "rust", codemirror_mime_type : "text/x-rustsrc", filenames : [], interpreters :
    ["ocaml"], }, ReadlineConfig => { name : "Readline Config", r#type : "data", color :
    "#000000", extensions : [], aliases : ["inputrc", "readline"], tm_scope :
    "source.inputrc", ace_mode : "text", language_id : 538732839u64, filenames :
    [".inputrc", "inputrc"], group : "INI", interpreters : [], }, Reason => { name :
    "Reason", r#type : "programming", color : "#ff5847", extensions : [".re", ".rei"],
    aliases : [], tm_scope : "source.reason", ace_mode : "rust", language_id :
    869538413u64, codemirror_mode : "rust", codemirror_mime_type : "text/x-rustsrc",
    filenames : [], interpreters : [], }, ReasonLIGO => { name : "ReasonLIGO", r#type :
    "programming", color : "#ff5847", extensions : [".religo"], aliases : [], tm_scope :
    "source.religo", ace_mode : "rust", language_id : 319002153u64, codemirror_mode :
    "rust", codemirror_mime_type : "text/x-rustsrc", filenames : [], group : "LigoLANG",
    interpreters : [], }, Rebol => { name : "Rebol", r#type : "programming", color :
    "#358a5b", extensions : [".reb", ".r", ".r2", ".r3", ".rebol"], aliases : [],
    tm_scope : "source.rebol", ace_mode : "text", language_id : 319u64, filenames : [],
    interpreters : [], }, RecordJar => { name : "Record Jar", r#type : "data", color :
    "#0673ba", extensions : [], aliases : [], tm_scope : "source.record-jar", ace_mode :
    "text", language_id : 865765202u64, codemirror_mode : "properties",
    codemirror_mime_type : "text/x-properties", filenames :
    ["language-subtag-registry.txt"], interpreters : [], }, Red => { name : "Red", r#type
    : "programming", color : "#f50000", extensions : [".red", ".reds"], aliases :
    ["red/system"], tm_scope : "source.red", ace_mode : "red", language_id : 320u64,
    filenames : [], interpreters : [], }, Redcode => { name : "Redcode", r#type :
    "programming", color : "#000000", extensions : [".cw"], aliases : [], tm_scope :
    "none", ace_mode : "text", language_id : 321u64, filenames : [], interpreters : [],
    }, RedirectRules => { name : "Redirect Rules", r#type : "data", color : "#000000",
    extensions : [], aliases : ["redirects"], tm_scope : "source.redirects", ace_mode :
    "text", language_id : 1020148948u64, filenames : ["_redirects"], interpreters : [],
    }, RegularExpression => { name : "Regular Expression", r#type : "data", color :
    "#009a00", extensions : [".regexp", ".regex"], aliases : ["regexp", "regex"],
    tm_scope : "source.regexp", ace_mode : "text", language_id : 363378884u64, filenames
    : [], interpreters : [], }, RenPy => { name : "Ren'Py", r#type : "programming", color
    : "#ff7f7f", extensions : [".rpy"], aliases : ["renpy"], tm_scope : "source.renpy",
    ace_mode : "python", language_id : 322u64, filenames : [], interpreters : [], },
    RenderScript => { name : "RenderScript", r#type : "programming", color : "#000000",
    extensions : [".rs", ".rsh"], aliases : [], tm_scope : "none", ace_mode : "text",
    language_id : 323u64, filenames : [], interpreters : [], }, Rez => { name : "Rez",
    r#type : "programming", color : "#FFDAB3", extensions : [".r"], aliases : [],
    tm_scope : "source.rez", ace_mode : "text", language_id : 498022874u64, filenames :
    [], interpreters : [], }, RichTextFormat => { name : "Rich Text Format", r#type :
    "markup", color : "#000000", extensions : [".rtf"], aliases : [], tm_scope :
    "text.rtf", ace_mode : "text", language_id : 51601661u64, filenames : [],
    interpreters : [], }, Ring => { name : "Ring", r#type : "programming", color :
    "#2D54CB", extensions : [".ring"], aliases : [], tm_scope : "source.ring", ace_mode :
    "text", language_id : 431u64, filenames : [], interpreters : [], }, Riot => { name :
    "Riot", r#type : "markup", color : "#A71E49", extensions : [".riot"], aliases : [],
    tm_scope : "text.html.riot", ace_mode : "html", language_id : 878396783u64, filenames
    : [], interpreters : [], }, RobotFramework => { name : "RobotFramework", r#type :
    "programming", color : "#00c0b5", extensions : [".robot", ".resource"], aliases : [],
    tm_scope : "text.robot", ace_mode : "robot", language_id : 324u64, filenames : [],
    interpreters : [], }, Roc => { name : "Roc", r#type : "programming", color :
    "#7c38f5", extensions : [".roc"], aliases : [], tm_scope : "source.roc", ace_mode :
    "text", language_id : 440182480u64, filenames : [], interpreters : [], }, RocqProver
    => { name : "Rocq Prover", r#type : "programming", color : "#d0b68c", extensions :
    [".v", ".coq"], aliases : ["coq", "rocq"], tm_scope : "source.coq", ace_mode :
    "text", language_id : 69u64, filenames : [], interpreters : [], }, Roff => { name :
    "Roff", r#type : "markup", color : "#ecdebe", extensions : [".roff", ".1", ".1in",
    ".1m", ".1x", ".2", ".3", ".3in", ".3m", ".3p", ".3pm", ".3qt", ".3x", ".4", ".5",
    ".6", ".7", ".8", ".9", ".l", ".man", ".mdoc", ".me", ".ms", ".n", ".nr", ".rno",
    ".tmac"], aliases : ["groff", "man", "manpage", "man page", "man-page", "mdoc",
    "nroff", "troff"], tm_scope : "text.roff", ace_mode : "text", language_id : 141u64,
    codemirror_mode : "troff", codemirror_mime_type : "text/troff", wrap : true,
    filenames : ["eqnrc", "mmn", "mmt", "troffrc", "troffrc-end"], interpreters : [], },
    RoffManpage => { name : "Roff Manpage", r#type : "markup", color : "#ecdebe",
    extensions : [".1", ".1in", ".1m", ".1x", ".2", ".3", ".3in", ".3m", ".3p", ".3pm",
    ".3qt", ".3x", ".4", ".5", ".6", ".7", ".8", ".9", ".man", ".mdoc"], aliases : [],
    tm_scope : "text.roff", ace_mode : "text", language_id : 612669833u64,
    codemirror_mode : "troff", codemirror_mime_type : "text/troff", wrap : true,
    filenames : [], group : "Roff", interpreters : [], }, Rouge => { name : "Rouge",
    r#type : "programming", color : "#cc0088", extensions : [".rg"], aliases : [],
    tm_scope : "source.clojure", ace_mode : "clojure", language_id : 325u64,
    codemirror_mode : "clojure", codemirror_mime_type : "text/x-clojure", filenames : [],
    interpreters : [], }, RouterOSScript => { name : "RouterOS Script", r#type :
    "programming", color : "#DE3941", extensions : [".rsc"], aliases : [], tm_scope :
    "none", ace_mode : "text", language_id : 592853203u64, filenames : [], interpreters :
    ["RouterOS"], }, Ruby => { name : "Ruby", r#type : "programming", color : "#701516",
    extensions : [".rb", ".builder", ".eye", ".fcgi", ".gemspec", ".god", ".jbuilder",
    ".mspec", ".pluginspec", ".podspec", ".prawn", ".rabl", ".rake", ".rbi", ".rbuild",
    ".rbw", ".rbx", ".ru", ".ruby", ".spec", ".thor", ".watchr"], aliases : ["jruby",
    "macruby", "rake", "rb", "rbx"], tm_scope : "source.ruby", ace_mode : "ruby",
    language_id : 326u64, codemirror_mode : "ruby", codemirror_mime_type : "text/x-ruby",
    filenames : [".irbrc", ".pryrc", ".simplecov", "Appraisals", "Berksfile", "Brewfile",
    "Buildfile", "Capfile", "Dangerfile", "Deliverfile", "Fastfile", "Gemfile",
    "Guardfile", "Jarfile", "Mavenfile", "Podfile", "Puppetfile", "Rakefile", "Snapfile",
    "Steepfile", "Thorfile", "Vagrantfile", "buildfile"], interpreters : ["ruby",
    "macruby", "rake", "jruby", "rbx"], }, Rust => { name : "Rust", r#type :
    "programming", color : "#dea584", extensions : [".rs", ".rs.in"], aliases : ["rs"],
    tm_scope : "source.rust", ace_mode : "rust", language_id : 327u64, codemirror_mode :
    "rust", codemirror_mime_type : "text/x-rustsrc", filenames : [], interpreters :
    ["rust-script"], }, SAS => { name : "SAS", r#type : "programming", color : "#B34936",
    extensions : [".sas"], aliases : [], tm_scope : "source.sas", ace_mode : "text",
    language_id : 328u64, codemirror_mode : "sas", codemirror_mime_type : "text/x-sas",
    filenames : [], interpreters : [], }, SCSS => { name : "SCSS", r#type : "markup",
    color : "#c6538c", extensions : [".scss"], aliases : [], tm_scope :
    "source.css.scss", ace_mode : "scss", language_id : 329u64, codemirror_mode : "css",
    codemirror_mime_type : "text/x-scss", filenames : [], interpreters : [], },
    SELinuxPolicy => { name : "SELinux Policy", r#type : "data", color : "#000000",
    extensions : [".te"], aliases : ["SELinux Kernel Policy Language", "sepolicy"],
    tm_scope : "source.sepolicy", ace_mode : "text", language_id : 880010326u64,
    filenames : ["file_contexts", "genfs_contexts", "initial_sids", "port_contexts",
    "security_classes"], interpreters : [], }, SMT => { name : "SMT", r#type :
    "programming", color : "#000000", extensions : [".smt2", ".smt", ".z3"], aliases :
    [], tm_scope : "source.smt", ace_mode : "text", language_id : 330u64, filenames : [],
    interpreters : ["boolector", "cvc4", "mathsat5", "opensmt", "smtinterpol", "smt-rat",
    "stp", "verit", "yices2", "z3"], }, SPARQL => { name : "SPARQL", r#type : "data",
    color : "#0C4597", extensions : [".sparql", ".rq"], aliases : [], tm_scope :
    "source.sparql", ace_mode : "sparql", language_id : 331u64, codemirror_mode :
    "sparql", codemirror_mime_type : "application/sparql-query", filenames : [],
    interpreters : [], }, SQF => { name : "SQF", r#type : "programming", color :
    "#3F3F3F", extensions : [".sqf", ".hqf"], aliases : [], tm_scope : "source.sqf",
    ace_mode : "text", language_id : 332u64, filenames : [], interpreters : [], }, SQL =>
    { name : "SQL", r#type : "data", color : "#e38c00", extensions : [".sql", ".cql",
    ".ddl", ".inc", ".mysql", ".prc", ".tab", ".udf", ".viw"], aliases : [], tm_scope :
    "source.sql", ace_mode : "sql", language_id : 333u64, codemirror_mode : "sql",
    codemirror_mime_type : "text/x-sql", filenames : [], interpreters : [], }, SQLPL => {
    name : "SQLPL", r#type : "programming", color : "#e38c00", extensions : [".sql",
    ".db2"], aliases : [], tm_scope : "source.sql", ace_mode : "sql", language_id :
    334u64, codemirror_mode : "sql", codemirror_mime_type : "text/x-sql", filenames : [],
    interpreters : [], }, SRecodeTemplate => { name : "SRecode Template", r#type :
    "markup", color : "#348a34", extensions : [".srt"], aliases : [], tm_scope :
    "source.lisp", ace_mode : "lisp", language_id : 335u64, codemirror_mode :
    "commonlisp", codemirror_mime_type : "text/x-common-lisp", filenames : [],
    interpreters : [], }, SSHConfig => { name : "SSH Config", r#type : "data", color :
    "#000000", extensions : [], aliases : ["sshconfig", "sshdconfig", "ssh_config",
    "sshd_config"], tm_scope : "source.ssh-config", ace_mode : "text", language_id :
    554920715u64, filenames : ["ssh-config", "ssh_config", "sshconfig", "sshconfig.snip",
    "sshd-config", "sshd_config"], group : "INI", interpreters : [], }, STAR => { name :
    "STAR", r#type : "data", color : "#000000", extensions : [".star"], aliases : [],
    tm_scope : "source.star", ace_mode : "text", language_id : 424510560u64, filenames :
    [], interpreters : [], }, STL => { name : "STL", r#type : "data", color : "#373b5e",
    extensions : [".stl"], aliases : ["ascii stl", "stla"], tm_scope : "source.stl",
    ace_mode : "text", language_id : 455361735u64, filenames : [], interpreters : [], },
    STON => { name : "STON", r#type : "data", color : "#000000", extensions : [".ston"],
    aliases : [], tm_scope : "source.smalltalk", ace_mode : "text", language_id : 336u64,
    filenames : [], group : "Smalltalk", interpreters : [], }, SVG => { name : "SVG",
    r#type : "data", color : "#ff9900", extensions : [".svg"], aliases : [], tm_scope :
    "text.xml.svg", ace_mode : "svg", language_id : 337u64, codemirror_mode : "xml",
    codemirror_mime_type : "text/xml", filenames : [], interpreters : [], }, SWIG => {
    name : "SWIG", r#type : "programming", color : "#000000", extensions : [".i", ".swg",
    ".swig"], aliases : [], tm_scope : "source.c++", ace_mode : "c_cpp", language_id :
    1066250075u64, codemirror_mode : "clike", codemirror_mime_type : "text/x-c++src",
    filenames : [], interpreters : [], }, Sage => { name : "Sage", r#type :
    "programming", color : "#000000", extensions : [".sage", ".sagews"], aliases : [],
    tm_scope : "source.python", ace_mode : "python", language_id : 338u64,
    codemirror_mode : "python", codemirror_mime_type : "text/x-python", filenames : [],
    interpreters : [], }, Sail => { name : "Sail", r#type : "programming", color :
    "#259dd5", extensions : [".sail"], aliases : [], tm_scope : "source.sail", ace_mode :
    "text", language_id : 1029438153u64, filenames : [], interpreters : [], }, SaltStack
    => { name : "SaltStack", r#type : "programming", color : "#646464", extensions :
    [".sls"], aliases : ["saltstate", "salt"], tm_scope : "source.yaml.salt", ace_mode :
    "yaml", language_id : 339u64, codemirror_mode : "yaml", codemirror_mime_type :
    "text/x-yaml", filenames : [], interpreters : [], }, Sass => { name : "Sass", r#type
    : "markup", color : "#a53b70", extensions : [".sass"], aliases : [], tm_scope :
    "source.sass", ace_mode : "sass", language_id : 340u64, codemirror_mode : "sass",
    codemirror_mime_type : "text/x-sass", filenames : [], interpreters : [], }, Scala =>
    { name : "Scala", r#type : "programming", color : "#c22d40", extensions : [".scala",
    ".kojo", ".sbt", ".sc"], aliases : [], tm_scope : "source.scala", ace_mode : "scala",
    language_id : 341u64, codemirror_mode : "clike", codemirror_mime_type :
    "text/x-scala", filenames : [], interpreters : ["scala"], }, Scaml => { name :
    "Scaml", r#type : "markup", color : "#bd181a", extensions : [".scaml"], aliases : [],
    tm_scope : "source.scaml", ace_mode : "text", language_id : 342u64, filenames : [],
    interpreters : [], }, Scenic => { name : "Scenic", r#type : "programming", color :
    "#fdc700", extensions : [".scenic"], aliases : [], tm_scope : "source.scenic",
    ace_mode : "text", language_id : 619814037u64, filenames : [], interpreters :
    ["scenic"], }, Scheme => { name : "Scheme", r#type : "programming", color :
    "#1e4aec", extensions : [".scm", ".sch", ".sld", ".sls", ".sps", ".ss"], aliases :
    [], tm_scope : "source.scheme", ace_mode : "scheme", language_id : 343u64,
    codemirror_mode : "scheme", codemirror_mime_type : "text/x-scheme", filenames : [],
    interpreters : ["scheme", "guile", "bigloo", "chicken", "csi", "gosh", "r6rs"], },
    Scilab => { name : "Scilab", r#type : "programming", color : "#ca0f21", extensions :
    [".sci", ".sce", ".tst"], aliases : [], tm_scope : "source.scilab", ace_mode :
    "text", language_id : 344u64, filenames : [], interpreters : [], }, _Self => { name :
    "Self", r#type : "programming", color : "#0579aa", extensions : [".self"], aliases :
    [], tm_scope : "none", ace_mode : "text", language_id : 345u64, filenames : [],
    interpreters : [], }, ShaderLab => { name : "ShaderLab", r#type : "programming",
    color : "#222c37", extensions : [".shader"], aliases : [], tm_scope :
    "source.shaderlab", ace_mode : "text", language_id : 664257356u64, filenames : [],
    interpreters : [], }, Shell => { name : "Shell", r#type : "programming", color :
    "#89e051", extensions : [".sh", ".bash", ".bats", ".cgi", ".command", ".fcgi",
    ".ksh", ".sh.in", ".tmux", ".tool", ".trigger", ".zsh", ".zsh-theme"], aliases :
    ["sh", "shell-script", "bash", "zsh", "envrc"], tm_scope : "source.shell", ace_mode :
    "sh", language_id : 346u64, codemirror_mode : "shell", codemirror_mime_type :
    "text/x-sh", filenames : [".bash_aliases", ".bash_functions", ".bash_history",
    ".bash_logout", ".bash_profile", ".bashrc", ".cshrc", ".envrc", ".flaskenv",
    ".kshrc", ".login", ".profile", ".tmux.conf", ".xinitrc", ".xsession", ".zlogin",
    ".zlogout", ".zprofile", ".zshenv", ".zshrc", "9fs", "PKGBUILD", "bash_aliases",
    "bash_logout", "bash_profile", "bashrc", "cshrc", "gradlew", "kshrc", "login", "man",
    "mvnw", "profile", "tmux.conf", "xinitrc", "xsession", "zlogin", "zlogout",
    "zprofile", "zshenv", "zshrc"], interpreters : ["ash", "bash", "dash", "ksh", "mksh",
    "pdksh", "rc", "sh", "zsh"], }, ShellCheckConfig => { name : "ShellCheck Config",
    r#type : "data", color : "#cecfcb", extensions : [], aliases : ["shellcheckrc"],
    tm_scope : "source.shellcheckrc", ace_mode : "ini", language_id : 687511714u64,
    codemirror_mode : "properties", codemirror_mime_type : "text/x-properties", filenames
    : [".shellcheckrc"], interpreters : [], }, ShellSession => { name : "ShellSession",
    r#type : "programming", color : "#000000", extensions : [".sh-session"], aliases :
    ["bash session", "console"], tm_scope : "text.shell-session", ace_mode : "sh",
    language_id : 347u64, codemirror_mode : "shell", codemirror_mime_type : "text/x-sh",
    filenames : [], interpreters : [], }, Shen => { name : "Shen", r#type :
    "programming", color : "#120F14", extensions : [".shen"], aliases : [], tm_scope :
    "source.shen", ace_mode : "text", language_id : 348u64, filenames : [], interpreters
    : [], }, Sieve => { name : "Sieve", r#type : "programming", color : "#000000",
    extensions : [".sieve"], aliases : [], tm_scope : "source.sieve", ace_mode : "text",
    language_id : 208976687u64, codemirror_mode : "sieve", codemirror_mime_type :
    "application/sieve", filenames : [], interpreters : [], }, SimpleFileVerification =>
    { name : "Simple File Verification", r#type : "data", color : "#C9BFED", extensions :
    [".sfv"], aliases : ["sfv"], tm_scope : "source.sfv", ace_mode : "ini", language_id :
    735623761u64, codemirror_mode : "properties", codemirror_mime_type :
    "text/x-properties", filenames : [], group : "Checksums", interpreters : [], },
    Singularity => { name : "Singularity", r#type : "programming", color : "#64E6AD",
    extensions : [], aliases : [], tm_scope : "source.singularity", ace_mode : "text",
    language_id : 987024632u64, filenames : ["Singularity"], interpreters : [], }, Slang
    => { name : "Slang", r#type : "programming", color : "#1fbec9", extensions :
    [".slang"], aliases : [], tm_scope : "source.slang", ace_mode : "text", language_id :
    239357863u64, filenames : [], interpreters : [], }, Slash => { name : "Slash", r#type
    : "programming", color : "#007eff", extensions : [".sl"], aliases : [], tm_scope :
    "text.html.slash", ace_mode : "text", language_id : 349u64, filenames : [],
    interpreters : [], }, Slice => { name : "Slice", r#type : "programming", color :
    "#003fa2", extensions : [".ice"], aliases : [], tm_scope : "source.ice", ace_mode :
    "text", language_id : 894641667u64, filenames : [], interpreters : [], }, Slim => {
    name : "Slim", r#type : "markup", color : "#2b2b2b", extensions : [".slim"], aliases
    : [], tm_scope : "text.slim", ace_mode : "slim", language_id : 350u64,
    codemirror_mode : "slim", codemirror_mime_type : "text/x-slim", filenames : [],
    interpreters : [], }, Slint => { name : "Slint", r#type : "markup", color :
    "#2379F4", extensions : [".slint"], aliases : [], tm_scope : "source.slint", ace_mode
    : "text", language_id : 119900149u64, filenames : [], interpreters : [], }, SmPL => {
    name : "SmPL", r#type : "programming", color : "#c94949", extensions : [".cocci"],
    aliases : ["coccinelle"], tm_scope : "source.smpl", ace_mode : "text", language_id :
    164123055u64, filenames : [], interpreters : [], }, Smali => { name : "Smali", r#type
    : "programming", color : "#000000", extensions : [".smali"], aliases : [], tm_scope :
    "source.smali", ace_mode : "text", language_id : 351u64, filenames : [], interpreters
    : [], }, Smalltalk => { name : "Smalltalk", r#type : "programming", color :
    "#596706", extensions : [".st", ".cs"], aliases : ["squeak"], tm_scope :
    "source.smalltalk", ace_mode : "text", language_id : 352u64, codemirror_mode :
    "smalltalk", codemirror_mime_type : "text/x-stsrc", filenames : [], interpreters :
    [], }, Smarty => { name : "Smarty", r#type : "programming", color : "#f0c040",
    extensions : [".tpl"], aliases : [], tm_scope : "text.html.smarty", ace_mode :
    "smarty", language_id : 353u64, codemirror_mode : "smarty", codemirror_mime_type :
    "text/x-smarty", filenames : [], interpreters : [], }, Smithy => { name : "Smithy",
    r#type : "programming", color : "#c44536", extensions : [".smithy"], aliases : [],
    tm_scope : "source.smithy", ace_mode : "smithy", language_id : 1027892786u64,
    codemirror_mode : "clike", codemirror_mime_type : "text/x-csrc", filenames : [],
    interpreters : [], }, Snakemake => { name : "Snakemake", r#type : "programming",
    color : "#419179", extensions : [".smk", ".snakefile"], aliases : ["snakefile"],
    tm_scope : "source.python", ace_mode : "python", language_id : 151241392u64,
    codemirror_mode : "python", codemirror_mime_type : "text/x-python", filenames :
    ["Snakefile"], group : "Python", interpreters : [], }, Solidity => { name :
    "Solidity", r#type : "programming", color : "#AA6746", extensions : [".sol"], aliases
    : [], tm_scope : "source.solidity", ace_mode : "text", language_id : 237469032u64,
    filenames : [], interpreters : [], }, Soong => { name : "Soong", r#type : "data",
    color : "#000000", extensions : [], aliases : [], tm_scope : "source.bp", ace_mode :
    "text", language_id : 222900098u64, filenames : ["Android.bp"], interpreters : [], },
    SourcePawn => { name : "SourcePawn", r#type : "programming", color : "#f69e1d",
    extensions : [".sp", ".inc"], aliases : ["sourcemod"], tm_scope :
    "source.sourcepawn", ace_mode : "text", language_id : 354u64, filenames : [],
    interpreters : [], }, SplineFontDatabase => { name : "Spline Font Database", r#type :
    "data", color : "#000000", extensions : [".sfd"], aliases : [], tm_scope :
    "text.sfd", ace_mode : "yaml", language_id : 767169629u64, filenames : [],
    interpreters : [], }, Squirrel => { name : "Squirrel", r#type : "programming", color
    : "#800000", extensions : [".nut"], aliases : [], tm_scope : "source.nut", ace_mode :
    "c_cpp", language_id : 355u64, codemirror_mode : "clike", codemirror_mime_type :
    "text/x-squirrel", filenames : [], interpreters : [], }, Stan => { name : "Stan",
    r#type : "programming", color : "#b2011d", extensions : [".stan"], aliases : [],
    tm_scope : "source.stan", ace_mode : "text", language_id : 356u64, filenames : [],
    interpreters : [], }, StandardML => { name : "Standard ML", r#type : "programming",
    color : "#dc566d", extensions : [".ml", ".fun", ".sig", ".sml"], aliases : ["sml"],
    tm_scope : "source.ml", ace_mode : "text", language_id : 357u64, codemirror_mode :
    "mllike", codemirror_mime_type : "text/x-sml", filenames : [], interpreters : [], },
    Starlark => { name : "Starlark", r#type : "programming", color : "#76d275",
    extensions : [".bzl", ".star"], aliases : ["bazel", "bzl"], tm_scope :
    "source.python", ace_mode : "python", language_id : 960266174u64, codemirror_mode :
    "python", codemirror_mime_type : "text/x-python", filenames : ["BUCK", "BUILD",
    "BUILD.bazel", "MODULE.bazel", "Tiltfile", "WORKSPACE", "WORKSPACE.bazel",
    "WORKSPACE.bzlmod"], interpreters : [], }, Stata => { name : "Stata", r#type :
    "programming", color : "#1a5f91", extensions : [".do", ".ado", ".doh", ".ihlp",
    ".mata", ".matah", ".sthlp"], aliases : [], tm_scope : "source.stata", ace_mode :
    "text", language_id : 358u64, filenames : [], interpreters : [], }, StringTemplate =>
    { name : "StringTemplate", r#type : "markup", color : "#3fb34f", extensions :
    [".st"], aliases : [], tm_scope : "source.string-template", ace_mode : "html",
    language_id : 89855901u64, codemirror_mode : "htmlmixed", codemirror_mime_type :
    "text/html", filenames : [], interpreters : [], }, Stylus => { name : "Stylus",
    r#type : "markup", color : "#ff6347", extensions : [".styl"], aliases : [], tm_scope
    : "source.stylus", ace_mode : "stylus", language_id : 359u64, codemirror_mode :
    "stylus", codemirror_mime_type : "text/x-styl", filenames : [], interpreters : [], },
    SubRipText => { name : "SubRip Text", r#type : "data", color : "#9e0101", extensions
    : [".srt"], aliases : [], tm_scope : "text.srt", ace_mode : "text", language_id :
    360u64, filenames : [], interpreters : [], }, SugarSS => { name : "SugarSS", r#type :
    "markup", color : "#2fcc9f", extensions : [".sss"], aliases : [], tm_scope :
    "source.css.postcss.sugarss", ace_mode : "text", language_id : 826404698u64,
    filenames : [], interpreters : [], }, SuperCollider => { name : "SuperCollider",
    r#type : "programming", color : "#46390b", extensions : [".sc", ".scd"], aliases :
    [], tm_scope : "source.supercollider", ace_mode : "text", language_id : 361u64,
    filenames : [], interpreters : ["sclang", "scsynth"], }, SurvexData => { name :
    "Survex data", r#type : "data", color : "#ffcc99", extensions : [".svx"], aliases :
    [], tm_scope : "none", ace_mode : "text", language_id : 24470517u64, filenames : [],
    interpreters : [], }, Svelte => { name : "Svelte", r#type : "markup", color :
    "#ff3e00", extensions : [".svelte"], aliases : [], tm_scope : "source.svelte",
    ace_mode : "html", language_id : 928734530u64, codemirror_mode : "htmlmixed",
    codemirror_mime_type : "text/html", filenames : [], interpreters : [], }, Sway => {
    name : "Sway", r#type : "programming", color : "#00F58C", extensions : [".sw"],
    aliases : [], tm_scope : "source.sway", ace_mode : "rust", language_id :
    271471144u64, codemirror_mode : "rust", codemirror_mime_type : "text/x-rustsrc",
    filenames : [], interpreters : [], }, Sweave => { name : "Sweave", r#type : "prose",
    color : "#198ce7", extensions : [".rnw"], aliases : [], tm_scope :
    "text.tex.latex.sweave", ace_mode : "tex", language_id : 558779190u64, filenames :
    [], interpreters : [], }, Swift => { name : "Swift", r#type : "programming", color :
    "#F05138", extensions : [".swift"], aliases : [], tm_scope : "source.swift", ace_mode
    : "swift", language_id : 362u64, codemirror_mode : "swift", codemirror_mime_type :
    "text/x-swift", filenames : [], interpreters : [], }, SystemVerilog => { name :
    "SystemVerilog", r#type : "programming", color : "#DAE1C2", extensions : [".sv",
    ".svh", ".vh"], aliases : [], tm_scope : "source.systemverilog", ace_mode :
    "verilog", language_id : 363u64, codemirror_mode : "verilog", codemirror_mime_type :
    "text/x-systemverilog", filenames : [], interpreters : [], }, TIProgram => { name :
    "TI Program", r#type : "programming", color : "#A0AA87", extensions : [".8xp",
    ".8xp.txt"], aliases : [], tm_scope : "source.8xp", ace_mode : "text", language_id :
    422u64, filenames : [], interpreters : [], }, TLVerilog => { name : "TL-Verilog",
    r#type : "programming", color : "#C40023", extensions : [".tlv"], aliases : [],
    tm_scope : "source.tlverilog", ace_mode : "verilog", language_id : 118656070u64,
    filenames : [], interpreters : [], }, TLA => { name : "TLA", r#type : "programming",
    color : "#4b0079", extensions : [".tla"], aliases : [], tm_scope : "source.tla",
    ace_mode : "text", language_id : 364u64, filenames : [], interpreters : [], }, TOML
    => { name : "TOML", r#type : "data", color : "#9c4221", extensions : [".toml",
    ".toml.example"], aliases : [], tm_scope : "source.toml", ace_mode : "toml",
    language_id : 365u64, codemirror_mode : "toml", codemirror_mime_type : "text/x-toml",
    filenames : ["Cargo.lock", "Cargo.toml.orig", "Gopkg.lock", "Pipfile", "pdm.lock",
    "poetry.lock", "uv.lock"], interpreters : [], }, TSPLIBData => { name :
    "TSPLIB data", r#type : "data", color : "#000000", extensions : [".tsp"], aliases :
    ["travelling salesman problem", "traveling salesman problem"], tm_scope : "none",
    ace_mode : "text", language_id : 89289301u64, filenames : [], interpreters : [], },
    TSQL => { name : "TSQL", r#type : "programming", color : "#e38c00", extensions :
    [".sql"], aliases : [], tm_scope : "source.tsql", ace_mode : "sql", language_id :
    918334941u64, filenames : [], interpreters : [], }, TSV => { name : "TSV", r#type :
    "data", color : "#237346", extensions : [".tsv", ".vcf"], aliases :
    ["tab-seperated values"], tm_scope : "source.generic-db", ace_mode : "tsv",
    language_id : 1035892117u64, filenames : [], interpreters : [], }, TSX => { name :
    "TSX", r#type : "programming", color : "#3178c6", extensions : [".tsx"], aliases :
    [], tm_scope : "source.tsx", ace_mode : "tsx", language_id : 94901924u64,
    codemirror_mode : "jsx", codemirror_mime_type : "text/typescript-jsx", filenames :
    [], group : "TypeScript", interpreters : [], }, TXL => { name : "TXL", r#type :
    "programming", color : "#0178b8", extensions : [".txl"], aliases : [], tm_scope :
    "source.txl", ace_mode : "text", language_id : 366u64, filenames : [], interpreters :
    [], }, Tact => { name : "Tact", r#type : "programming", color : "#48b5ff", extensions
    : [".tact"], aliases : [], tm_scope : "source.tact", ace_mode : "text", language_id :
    606708469u64, filenames : [], interpreters : [], }, Talon => { name : "Talon", r#type
    : "programming", color : "#333333", extensions : [".talon"], aliases : [], tm_scope :
    "source.talon", ace_mode : "text", language_id : 959889508u64, filenames : [],
    interpreters : [], }, Tcl => { name : "Tcl", r#type : "programming", color :
    "#e4cc98", extensions : [".tcl", ".adp", ".sdc", ".tcl.in", ".tm", ".xdc"], aliases :
    ["sdc", "xdc"], tm_scope : "source.tcl", ace_mode : "tcl", language_id : 367u64,
    codemirror_mode : "tcl", codemirror_mime_type : "text/x-tcl", filenames : ["owh",
    "starfield"], interpreters : ["tclsh", "wish"], }, Tcsh => { name : "Tcsh", r#type :
    "programming", color : "#000000", extensions : [".tcsh", ".csh"], aliases : [],
    tm_scope : "source.shell", ace_mode : "sh", language_id : 368u64, codemirror_mode :
    "shell", codemirror_mime_type : "text/x-sh", filenames : [], group : "Shell",
    interpreters : ["tcsh", "csh"], }, TeX => { name : "TeX", r#type : "markup", color :
    "#3D6117", extensions : [".tex", ".aux", ".bbx", ".cbx", ".cls", ".dtx", ".ins",
    ".lbx", ".ltx", ".mkii", ".mkiv", ".mkvi", ".sty", ".toc"], aliases : ["latex"],
    tm_scope : "text.tex.latex", ace_mode : "tex", language_id : 369u64, codemirror_mode
    : "stex", codemirror_mime_type : "text/x-stex", wrap : true, filenames : [],
    interpreters : [], }, Tea => { name : "Tea", r#type : "markup", color : "#000000",
    extensions : [".tea"], aliases : [], tm_scope : "source.tea", ace_mode : "text",
    language_id : 370u64, filenames : [], interpreters : [], }, Teal => { name : "Teal",
    r#type : "programming", color : "#00B1BC", extensions : [".tl"], aliases : [],
    tm_scope : "source.teal", ace_mode : "lua", language_id : 719038619u64,
    codemirror_mode : "lua", codemirror_mime_type : "text/x-lua", filenames : [],
    interpreters : ["tl"], }, Terra => { name : "Terra", r#type : "programming", color :
    "#00004c", extensions : [".t"], aliases : [], tm_scope : "source.terra", ace_mode :
    "lua", language_id : 371u64, codemirror_mode : "lua", codemirror_mime_type :
    "text/x-lua", filenames : [], interpreters : ["lua"], }, TerraformTemplate => { name
    : "Terraform Template", r#type : "markup", color : "#7b42bb", extensions :
    [".tftpl"], aliases : [], tm_scope : "source.hcl.terraform", ace_mode : "ruby",
    language_id : 856832701u64, codemirror_mode : "ruby", codemirror_mime_type :
    "text/x-ruby", filenames : [], group : "HCL", interpreters : [], }, Texinfo => { name
    : "Texinfo", r#type : "prose", color : "#000000", extensions : [".texinfo", ".texi",
    ".txi"], aliases : [], tm_scope : "text.texinfo", ace_mode : "text", language_id :
    988020015u64, wrap : true, filenames : [], interpreters : ["makeinfo"], }, Text => {
    name : "Text", r#type : "prose", color : "#000000", extensions : [".txt", ".fr",
    ".nb", ".ncl", ".no"], aliases : ["fundamental", "plain text"], tm_scope : "none",
    ace_mode : "text", language_id : 372u64, wrap : true, filenames : ["CITATION",
    "CITATIONS", "COPYING", "COPYING.regex", "COPYRIGHT.regex", "FONTLOG", "INSTALL",
    "INSTALL.mysql", "LICENSE", "LICENSE.mysql", "NEWS", "README.me", "README.mysql",
    "README.nss", "click.me", "delete.me", "keep.me", "package.mask", "package.use.mask",
    "package.use.stable.mask", "read.me", "readme.1st", "test.me", "use.mask",
    "use.stable.mask"], interpreters : [], }, TextGrid => { name : "TextGrid", r#type :
    "data", color : "#c8506d", extensions : [".TextGrid"], aliases : [], tm_scope :
    "source.textgrid", ace_mode : "text", language_id : 965696054u64, filenames : [],
    interpreters : [], }, TextMateProperties => { name : "TextMate Properties", r#type :
    "data", color : "#df66e4", extensions : [], aliases : ["tm-properties"], tm_scope :
    "source.tm-properties", ace_mode : "properties", language_id : 981795023u64,
    codemirror_mode : "properties", codemirror_mime_type : "text/x-properties", filenames
    : [".tm_properties"], interpreters : [], }, Textile => { name : "Textile", r#type :
    "prose", color : "#ffe7ac", extensions : [".textile"], aliases : [], tm_scope :
    "none", ace_mode : "textile", language_id : 373u64, codemirror_mode : "textile",
    codemirror_mime_type : "text/x-textile", wrap : true, filenames : [], interpreters :
    [], }, Thrift => { name : "Thrift", r#type : "programming", color : "#D12127",
    extensions : [".thrift"], aliases : [], tm_scope : "source.thrift", ace_mode :
    "text", language_id : 374u64, filenames : [], interpreters : [], }, Toit => { name :
    "Toit", r#type : "programming", color : "#c2c9fb", extensions : [".toit"], aliases :
    [], tm_scope : "source.toit", ace_mode : "text", language_id : 356554395u64,
    filenames : [], interpreters : [], }, TorConfig => { name : "Tor Config", r#type :
    "data", color : "#59316b", extensions : [], aliases : ["torrc"], tm_scope :
    "source.torrc", ace_mode : "apache_conf", language_id : 1016912802u64, filenames :
    ["torrc"], interpreters : [], }, TreeSitterQuery => { name : "Tree-sitter Query",
    r#type : "programming", color : "#8ea64c", extensions : [".scm"], aliases : ["tsq"],
    tm_scope : "source.scm", ace_mode : "text", language_id : 436081647u64, filenames :
    [], interpreters : [], }, Turing => { name : "Turing", r#type : "programming", color
    : "#cf142b", extensions : [".t", ".tu"], aliases : [], tm_scope : "source.turing",
    ace_mode : "text", language_id : 375u64, filenames : [], interpreters : [], }, Turtle
    => { name : "Turtle", r#type : "data", color : "#000000", extensions : [".ttl"],
    aliases : [], tm_scope : "source.turtle", ace_mode : "turtle", language_id : 376u64,
    codemirror_mode : "turtle", codemirror_mime_type : "text/turtle", filenames : [],
    interpreters : [], }, Twig => { name : "Twig", r#type : "markup", color : "#c1d026",
    extensions : [".twig"], aliases : [], tm_scope : "text.html.twig", ace_mode : "twig",
    language_id : 377u64, codemirror_mode : "twig", codemirror_mime_type : "text/x-twig",
    filenames : [], interpreters : [], }, TypeLanguage => { name : "Type Language",
    r#type : "data", color : "#000000", extensions : [".tl"], aliases : ["tl"], tm_scope
    : "source.tl", ace_mode : "text", language_id : 632765617u64, filenames : [],
    interpreters : [], }, TypeScript => { name : "TypeScript", r#type : "programming",
    color : "#3178c6", extensions : [".ts", ".cts", ".mts"], aliases : ["ts"], tm_scope :
    "source.ts", ace_mode : "typescript", language_id : 378u64, codemirror_mode :
    "javascript", codemirror_mime_type : "application/typescript", filenames : [],
    interpreters : ["bun", "deno", "ts-node", "tsx"], }, TypeSpec => { name : "TypeSpec",
    r#type : "programming", color : "#4A3665", extensions : [".tsp"], aliases : ["tsp"],
    tm_scope : "source.tsp", ace_mode : "text", language_id : 952272597u64, filenames :
    [], interpreters : [], }, Typst => { name : "Typst", r#type : "programming", color :
    "#239dad", extensions : [".typ"], aliases : ["typ"], tm_scope : "source.typst",
    ace_mode : "text", language_id : 704730682u64, filenames : [], interpreters : [], },
    UnifiedParallelC => { name : "Unified Parallel C", r#type : "programming", color :
    "#4e3617", extensions : [".upc"], aliases : [], tm_scope : "source.c", ace_mode :
    "c_cpp", language_id : 379u64, codemirror_mode : "clike", codemirror_mime_type :
    "text/x-csrc", filenames : [], group : "C", interpreters : [], }, Unity3DAsset => {
    name : "Unity3D Asset", r#type : "data", color : "#222c37", extensions : [".anim",
    ".asset", ".mask", ".mat", ".meta", ".prefab", ".unity"], aliases : [], tm_scope :
    "source.yaml", ace_mode : "yaml", language_id : 380u64, codemirror_mode : "yaml",
    codemirror_mime_type : "text/x-yaml", filenames : [], interpreters : [], },
    UnixAssembly => { name : "Unix Assembly", r#type : "programming", color : "#000000",
    extensions : [".s", ".ms"], aliases : ["gas", "gnu asm", "unix asm"], tm_scope :
    "source.x86", ace_mode : "assembly_x86", language_id : 120u64, filenames : [], group
    : "Assembly", interpreters : [], }, Uno => { name : "Uno", r#type : "programming",
    color : "#9933cc", extensions : [".uno"], aliases : [], tm_scope : "source.cs",
    ace_mode : "csharp", language_id : 381u64, codemirror_mode : "clike",
    codemirror_mime_type : "text/x-csharp", filenames : [], interpreters : [], },
    UnrealScript => { name : "UnrealScript", r#type : "programming", color : "#a54c4d",
    extensions : [".uc"], aliases : [], tm_scope : "source.java", ace_mode : "java",
    language_id : 382u64, codemirror_mode : "clike", codemirror_mime_type :
    "text/x-java", filenames : [], interpreters : [], }, UntypedPlutusCore => { name :
    "Untyped Plutus Core", r#type : "programming", color : "#36adbd", extensions :
    [".uplc"], aliases : [], tm_scope : "source.uplc", ace_mode : "text", language_id :
    1061635506u64, filenames : [], interpreters : [], }, UrWeb => { name : "UrWeb",
    r#type : "programming", color : "#ccccee", extensions : [".ur", ".urs"], aliases :
    ["Ur/Web", "Ur"], tm_scope : "source.ur", ace_mode : "text", language_id : 383u64,
    filenames : [], interpreters : [], }, V => { name : "V", r#type : "programming",
    color : "#4f87c4", extensions : [".v"], aliases : ["vlang"], tm_scope : "source.v",
    ace_mode : "golang", language_id : 603371597u64, codemirror_mode : "go",
    codemirror_mime_type : "text/x-go", filenames : [], interpreters : [], }, VBA => {
    name : "VBA", r#type : "programming", color : "#867db1", extensions : [".bas",
    ".cls", ".frm", ".vba"], aliases : ["visual basic for applications"], tm_scope :
    "source.vba", ace_mode : "text", language_id : 399230729u64, codemirror_mode : "vb",
    codemirror_mime_type : "text/x-vb", filenames : [], interpreters : [], }, VBScript =>
    { name : "VBScript", r#type : "programming", color : "#15dcdc", extensions :
    [".vbs"], aliases : [], tm_scope : "source.vbnet", ace_mode : "vbscript", language_id
    : 408016005u64, codemirror_mode : "vbscript", codemirror_mime_type : "text/vbscript",
    filenames : [], interpreters : [], }, VCL => { name : "VCL", r#type : "programming",
    color : "#148AA8", extensions : [".vcl"], aliases : [], tm_scope : "source.vcl",
    ace_mode : "text", language_id : 384u64, filenames : [], interpreters : [], }, VHDL
    => { name : "VHDL", r#type : "programming", color : "#adb2cb", extensions : [".vhdl",
    ".vhd", ".vhf", ".vhi", ".vho", ".vhs", ".vht", ".vhw"], aliases : [], tm_scope :
    "source.vhdl", ace_mode : "vhdl", language_id : 385u64, codemirror_mode : "vhdl",
    codemirror_mime_type : "text/x-vhdl", filenames : [], interpreters : [], }, Vala => {
    name : "Vala", r#type : "programming", color : "#a56de2", extensions : [".vala",
    ".vapi"], aliases : [], tm_scope : "source.vala", ace_mode : "vala", language_id :
    386u64, filenames : [], interpreters : [], }, ValveDataFormat => { name :
    "Valve Data Format", r#type : "data", color : "#f26025", extensions : [".vdf"],
    aliases : ["keyvalues", "vdf"], tm_scope : "source.keyvalues", ace_mode : "text",
    language_id : 544060961u64, filenames : [], interpreters : [], },
    VelocityTemplateLanguage => { name : "Velocity Template Language", r#type : "markup",
    color : "#507cff", extensions : [".vtl"], aliases : ["vtl", "velocity"], tm_scope :
    "source.velocity", ace_mode : "velocity", language_id : 292377326u64, codemirror_mode
    : "velocity", codemirror_mime_type : "text/velocity", filenames : [], interpreters :
    [], }, Vento => { name : "Vento", r#type : "markup", color : "#ff0080", extensions :
    [".vto"], aliases : [], tm_scope : "source.vento", ace_mode : "text", language_id :
    757053899u64, filenames : [], interpreters : [], }, Verilog => { name : "Verilog",
    r#type : "programming", color : "#b2b7f8", extensions : [".v", ".veo"], aliases : [],
    tm_scope : "source.verilog", ace_mode : "verilog", language_id : 387u64,
    codemirror_mode : "verilog", codemirror_mime_type : "text/x-verilog", filenames : [],
    interpreters : [], }, VimHelpFile => { name : "Vim Help File", r#type : "prose",
    color : "#199f4b", extensions : [".txt"], aliases : ["help", "vimhelp"], tm_scope :
    "text.vim-help", ace_mode : "text", language_id : 508563686u64, filenames : [],
    interpreters : [], }, VimScript => { name : "Vim Script", r#type : "programming",
    color : "#199f4b", extensions : [".vim", ".vba", ".vimrc", ".vmb"], aliases : ["vim",
    "viml", "nvim", "vimscript"], tm_scope : "source.viml", ace_mode : "text",
    language_id : 388u64, filenames : [".exrc", ".gvimrc", ".nvimrc", ".vimrc", "_vimrc",
    "gvimrc", "nvimrc", "vimrc"], interpreters : [], }, VimSnippet => { name :
    "Vim Snippet", r#type : "markup", color : "#199f4b", extensions : [".snip",
    ".snippet", ".snippets"], aliases : ["SnipMate", "UltiSnip", "UltiSnips",
    "NeoSnippet"], tm_scope : "source.vim-snippet", ace_mode : "text", language_id :
    81265970u64, filenames : [], interpreters : [], }, VisualBasicNET => { name :
    "Visual Basic .NET", r#type : "programming", color : "#945db7", extensions : [".vb",
    ".vbhtml"], aliases : ["visual basic", "vbnet", "vb .net", "vb.net"], tm_scope :
    "source.vbnet", ace_mode : "text", language_id : 389u64, codemirror_mode : "vb",
    codemirror_mime_type : "text/x-vb", filenames : [], interpreters : [], },
    VisualBasic60 => { name : "Visual Basic 6.0", r#type : "programming", color :
    "#2c6353", extensions : [".bas", ".cls", ".ctl", ".Dsr", ".frm"], aliases : ["vb6",
    "vb 6", "visual basic 6", "visual basic classic", "classic visual basic"], tm_scope :
    "source.vba", ace_mode : "text", language_id : 679594952u64, codemirror_mode : "vb",
    codemirror_mime_type : "text/x-vb", filenames : [], interpreters : [], }, Volt => {
    name : "Volt", r#type : "programming", color : "#1F1F1F", extensions : [".volt"],
    aliases : [], tm_scope : "source.d", ace_mode : "d", language_id : 390u64,
    codemirror_mode : "d", codemirror_mime_type : "text/x-d", filenames : [],
    interpreters : [], }, Vue => { name : "Vue", r#type : "markup", color : "#41b883",
    extensions : [".vue"], aliases : [], tm_scope : "source.vue", ace_mode : "vue",
    language_id : 391u64, codemirror_mode : "vue", codemirror_mime_type : "text/x-vue",
    filenames : [], interpreters : [], }, Vyper => { name : "Vyper", r#type :
    "programming", color : "#9F4CF2", extensions : [".vy"], aliases : [], tm_scope :
    "source.vyper", ace_mode : "text", language_id : 1055641948u64, filenames : [],
    interpreters : [], }, WDL => { name : "WDL", r#type : "programming", color :
    "#42f1f4", extensions : [".wdl"], aliases : ["Workflow Description Language"],
    tm_scope : "source.wdl", ace_mode : "text", language_id : 374521672u64, filenames :
    [], interpreters : [], }, WGSL => { name : "WGSL", r#type : "programming", color :
    "#1a5e9a", extensions : [".wgsl"], aliases : [], tm_scope : "source.wgsl", ace_mode :
    "text", language_id : 836605993u64, filenames : [], interpreters : [], },
    WavefrontMaterial => { name : "Wavefront Material", r#type : "data", color :
    "#000000", extensions : [".mtl"], aliases : [], tm_scope : "source.wavefront.mtl",
    ace_mode : "text", language_id : 392u64, filenames : [], interpreters : [], },
    WavefrontObject => { name : "Wavefront Object", r#type : "data", color : "#000000",
    extensions : [".obj"], aliases : [], tm_scope : "source.wavefront.obj", ace_mode :
    "text", language_id : 393u64, filenames : [], interpreters : [], },
    WebOntologyLanguage => { name : "Web Ontology Language", r#type : "data", color :
    "#5b70bd", extensions : [".owl"], aliases : [], tm_scope : "text.xml", ace_mode :
    "xml", language_id : 394u64, filenames : [], interpreters : [], }, WebAssembly => {
    name : "WebAssembly", r#type : "programming", color : "#04133b", extensions :
    [".wast", ".wat"], aliases : ["wast", "wasm"], tm_scope : "source.webassembly",
    ace_mode : "lisp", language_id : 956556503u64, codemirror_mode : "wast",
    codemirror_mime_type : "text/webassembly", filenames : [], interpreters : [], },
    WebAssemblyInterfaceType => { name : "WebAssembly Interface Type", r#type : "data",
    color : "#6250e7", extensions : [".wit"], aliases : ["wit"], tm_scope : "source.wit",
    ace_mode : "text", language_id : 134534086u64, codemirror_mode : "webidl",
    codemirror_mime_type : "text/x-webidl", filenames : [], interpreters : [], }, WebIDL
    => { name : "WebIDL", r#type : "programming", color : "#000000", extensions :
    [".webidl"], aliases : [], tm_scope : "source.webidl", ace_mode : "text", language_id
    : 395u64, codemirror_mode : "webidl", codemirror_mime_type : "text/x-webidl",
    filenames : [], interpreters : [], }, WebVTT => { name : "WebVTT", r#type : "data",
    color : "#000000", extensions : [".vtt"], aliases : ["vtt"], tm_scope : "text.vtt",
    ace_mode : "text", language_id : 658679714u64, wrap : true, filenames : [],
    interpreters : [], }, WgetConfig => { name : "Wget Config", r#type : "data", color :
    "#000000", extensions : [], aliases : ["wgetrc"], tm_scope : "source.wgetrc",
    ace_mode : "text", language_id : 668457123u64, filenames : [".wgetrc"], group :
    "INI", interpreters : [], }, Whiley => { name : "Whiley", r#type : "programming",
    color : "#d5c397", extensions : [".whiley"], aliases : [], tm_scope :
    "source.whiley", ace_mode : "text", language_id : 888779559u64, filenames : [],
    interpreters : [], }, Wikitext => { name : "Wikitext", r#type : "prose", color :
    "#fc5757", extensions : [".mediawiki", ".wiki", ".wikitext"], aliases : ["mediawiki",
    "wiki"], tm_scope : "text.html.mediawiki", ace_mode : "mediawiki", language_id :
    228u64, wrap : true, filenames : [], interpreters : [], }, Win32MessageFile => { name
    : "Win32 Message File", r#type : "data", color : "#000000", extensions : [".mc"],
    aliases : [], tm_scope : "source.win32-messages", ace_mode : "ini", language_id :
    950967261u64, codemirror_mode : "properties", codemirror_mime_type :
    "text/x-properties", filenames : [], interpreters : [], }, WindowsRegistryEntries =>
    { name : "Windows Registry Entries", r#type : "data", color : "#52d5ff", extensions :
    [".reg"], aliases : [], tm_scope : "source.reg", ace_mode : "ini", language_id :
    969674868u64, codemirror_mode : "properties", codemirror_mime_type :
    "text/x-properties", filenames : [], interpreters : [], }, WitcherScript => { name :
    "Witcher Script", r#type : "programming", color : "#ff0000", extensions : [".ws"],
    aliases : [], tm_scope : "source.witcherscript", ace_mode : "text", language_id :
    686821385u64, filenames : [], interpreters : [], }, WolframLanguage => { name :
    "Wolfram Language", r#type : "programming", color : "#dd1100", extensions :
    [".mathematica", ".cdf", ".m", ".ma", ".mt", ".nb", ".nbp", ".wl", ".wls", ".wlt"],
    aliases : ["mathematica", "mma", "wolfram", "wolfram lang", "wl"], tm_scope :
    "source.mathematica", ace_mode : "text", language_id : 224u64, codemirror_mode :
    "mathematica", codemirror_mime_type : "text/x-mathematica", filenames : [],
    interpreters : ["wolfram", "WolframKernel", "wolframscript", "math", "MathKernel",
    "MathematicaScript", "WolframNB", "Mathematica"], }, Wollok => { name : "Wollok",
    r#type : "programming", color : "#a23738", extensions : [".wlk"], aliases : [],
    tm_scope : "source.wollok", ace_mode : "wollok", language_id : 632745969u64,
    filenames : [], interpreters : [], }, WorldOfWarcraftAddonData => { name :
    "World of Warcraft Addon Data", r#type : "data", color : "#f7e43f", extensions :
    [".toc"], aliases : [], tm_scope : "source.toc", ace_mode : "text", language_id :
    396u64, filenames : [], interpreters : [], }, Wren => { name : "Wren", r#type :
    "programming", color : "#383838", extensions : [".wren"], aliases : ["wrenlang"],
    tm_scope : "source.wren", ace_mode : "text", language_id : 713580619u64, filenames :
    [], interpreters : [], }, XBitMap => { name : "X BitMap", r#type : "data", color :
    "#000000", extensions : [".xbm"], aliases : ["xbm"], tm_scope : "source.c", ace_mode
    : "c_cpp", language_id : 782911107u64, codemirror_mode : "clike",
    codemirror_mime_type : "text/x-csrc", filenames : [], group : "C", interpreters : [],
    }, XFontDirectoryIndex => { name : "X Font Directory Index", r#type : "data", color :
    "#000000", extensions : [], aliases : [], tm_scope : "source.fontdir", ace_mode :
    "text", language_id : 208700028u64, filenames : ["encodings.dir", "fonts.alias",
    "fonts.dir", "fonts.scale"], interpreters : [], }, XPixMap => { name : "X PixMap",
    r#type : "data", color : "#000000", extensions : [".xpm", ".pm"], aliases : ["xpm"],
    tm_scope : "source.c", ace_mode : "c_cpp", language_id : 781846279u64,
    codemirror_mode : "clike", codemirror_mime_type : "text/x-csrc", filenames : [],
    group : "C", interpreters : [], }, X10 => { name : "X10", r#type : "programming",
    color : "#4B6BEF", extensions : [".x10"], aliases : ["xten"], tm_scope :
    "source.x10", ace_mode : "text", language_id : 397u64, filenames : [], interpreters :
    [], }, XC => { name : "XC", r#type : "programming", color : "#99DA07", extensions :
    [".xc"], aliases : [], tm_scope : "source.xc", ace_mode : "c_cpp", language_id :
    398u64, codemirror_mode : "clike", codemirror_mime_type : "text/x-csrc", filenames :
    [], interpreters : [], }, XCompose => { name : "XCompose", r#type : "data", color :
    "#000000", extensions : [], aliases : [], tm_scope : "config.xcompose", ace_mode :
    "text", language_id : 225167241u64, filenames : [".XCompose", "XCompose",
    "xcompose"], interpreters : [], }, XML => { name : "XML", r#type : "data", color :
    "#0060ac", extensions : [".xml", ".adml", ".admx", ".ant", ".axaml", ".axml",
    ".builds", ".ccproj", ".ccxml", ".clixml", ".cproject", ".cscfg", ".csdef", ".csl",
    ".csproj", ".ct", ".depproj", ".dita", ".ditamap", ".ditaval", ".dll.config",
    ".dotsettings", ".filters", ".fsproj", ".fxml", ".glade", ".gml", ".gmx", ".gpx",
    ".grxml", ".gst", ".hzp", ".iml", ".ivy", ".jelly", ".jsproj", ".kml", ".launch",
    ".mdpolicy", ".mjml", ".mm", ".mod", ".mojo", ".mxml", ".natvis", ".ncl", ".ndproj",
    ".nproj", ".nuspec", ".odd", ".osm", ".pkgproj", ".pluginspec", ".proj", ".props",
    ".ps1xml", ".psc1", ".pt", ".qhelp", ".rdf", ".res", ".resx", ".rs", ".rss", ".sch",
    ".scxml", ".sfproj", ".shproj", ".slnx", ".srdf", ".storyboard", ".sublime-snippet",
    ".sw", ".targets", ".tml", ".ts", ".tsx", ".typ", ".ui", ".urdf", ".ux", ".vbproj",
    ".vcxproj", ".vsixmanifest", ".vssettings", ".vstemplate", ".vxml", ".wixproj",
    ".workflow", ".wsdl", ".wsf", ".wxi", ".wxl", ".wxs", ".x3d", ".xacro", ".xaml",
    ".xib", ".xlf", ".xliff", ".xmi", ".xml.dist", ".xmp", ".xproj", ".xsd", ".xspec",
    ".xul", ".zcml"], aliases : ["rss", "xsd", "wsdl"], tm_scope : "text.xml", ace_mode :
    "xml", language_id : 399u64, codemirror_mode : "xml", codemirror_mime_type :
    "text/xml", filenames : [".classpath", ".cproject", ".project", "App.config",
    "NuGet.config", "Settings.StyleCop", "Web.Debug.config", "Web.Release.config",
    "Web.config", "packages.config"], interpreters : [], }, XMLPropertyList => { name :
    "XML Property List", r#type : "data", color : "#0060ac", extensions : [".plist",
    ".stTheme", ".tmCommand", ".tmLanguage", ".tmPreferences", ".tmSnippet", ".tmTheme"],
    aliases : [], tm_scope : "text.xml.plist", ace_mode : "xml", language_id :
    75622871u64, codemirror_mode : "xml", codemirror_mime_type : "text/xml", filenames :
    [], group : "XML", interpreters : [], }, XPages => { name : "XPages", r#type :
    "data", color : "#000000", extensions : [".xsp-config", ".xsp.metadata"], aliases :
    [], tm_scope : "text.xml", ace_mode : "xml", language_id : 400u64, codemirror_mode :
    "xml", codemirror_mime_type : "text/xml", filenames : [], interpreters : [], }, XProc
    => { name : "XProc", r#type : "programming", color : "#000000", extensions : [".xpl",
    ".xproc"], aliases : [], tm_scope : "text.xml", ace_mode : "xml", language_id :
    401u64, codemirror_mode : "xml", codemirror_mime_type : "text/xml", filenames : [],
    interpreters : [], }, XQuery => { name : "XQuery", r#type : "programming", color :
    "#5232e7", extensions : [".xquery", ".xq", ".xql", ".xqm", ".xqy"], aliases : [],
    tm_scope : "source.xq", ace_mode : "xquery", language_id : 402u64, codemirror_mode :
    "xquery", codemirror_mime_type : "application/xquery", filenames : [], interpreters :
    [], }, XS => { name : "XS", r#type : "programming", color : "#000000", extensions :
    [".xs"], aliases : [], tm_scope : "source.c", ace_mode : "c_cpp", language_id :
    403u64, codemirror_mode : "clike", codemirror_mime_type : "text/x-csrc", filenames :
    [], interpreters : [], }, XSLT => { name : "XSLT", r#type : "programming", color :
    "#EB8CEB", extensions : [".xslt", ".xsl"], aliases : ["xsl"], tm_scope :
    "text.xml.xsl", ace_mode : "xml", language_id : 404u64, codemirror_mode : "xml",
    codemirror_mime_type : "text/xml", filenames : [], interpreters : [], }, Xmake => {
    name : "Xmake", r#type : "programming", color : "#22a079", extensions : [], aliases :
    [], tm_scope : "source.xmake", ace_mode : "text", language_id : 225223071u64,
    filenames : ["xmake.lua"], interpreters : [], }, Xojo => { name : "Xojo", r#type :
    "programming", color : "#81bd41", extensions : [".xojo_code", ".xojo_menu",
    ".xojo_report", ".xojo_script", ".xojo_toolbar", ".xojo_window"], aliases : [],
    tm_scope : "source.xojo", ace_mode : "text", language_id : 405u64, filenames : [],
    interpreters : [], }, Xonsh => { name : "Xonsh", r#type : "programming", color :
    "#285EEF", extensions : [".xsh"], aliases : [], tm_scope : "source.python", ace_mode
    : "text", language_id : 614078284u64, codemirror_mode : "python",
    codemirror_mime_type : "text/x-python", filenames : [], interpreters : [], }, Xtend
    => { name : "Xtend", r#type : "programming", color : "#24255d", extensions :
    [".xtend"], aliases : [], tm_scope : "source.xtend", ace_mode : "text", language_id :
    406u64, filenames : [], interpreters : [], }, YAML => { name : "YAML", r#type :
    "data", color : "#cb171e", extensions : [".yml", ".mir", ".reek", ".rviz",
    ".sublime-syntax", ".syntax", ".yaml", ".yaml-tmlanguage", ".yaml.sed",
    ".yml.mysql"], aliases : ["yml"], tm_scope : "source.yaml", ace_mode : "yaml",
    language_id : 407u64, codemirror_mode : "yaml", codemirror_mime_type : "text/x-yaml",
    filenames : [".clang-format", ".clang-tidy", ".clangd", ".gemrc", "CITATION.cff",
    "glide.lock", "pixi.lock", "yarn.lock"], interpreters : [], }, YANG => { name :
    "YANG", r#type : "data", color : "#000000", extensions : [".yang"], aliases : [],
    tm_scope : "source.yang", ace_mode : "text", language_id : 408u64, filenames : [],
    interpreters : [], }, YARA => { name : "YARA", r#type : "programming", color :
    "#220000", extensions : [".yar", ".yara"], aliases : [], tm_scope : "source.yara",
    ace_mode : "text", language_id : 805122868u64, filenames : [], interpreters : [], },
    YASnippet => { name : "YASnippet", r#type : "markup", color : "#32AB90", extensions :
    [".yasnippet"], aliases : ["snippet", "yas"], tm_scope : "source.yasnippet", ace_mode
    : "text", language_id : 378760102u64, filenames : [], interpreters : [], }, Yacc => {
    name : "Yacc", r#type : "programming", color : "#4B6C4B", extensions : [".y",
    ".yacc", ".yy"], aliases : [], tm_scope : "source.yacc", ace_mode : "text",
    language_id : 409u64, filenames : [], interpreters : [], }, Yul => { name : "Yul",
    r#type : "programming", color : "#794932", extensions : [".yul"], aliases : [],
    tm_scope : "source.yul", ace_mode : "text", language_id : 237469033u64, filenames :
    [], interpreters : [], }, ZAP => { name : "ZAP", r#type : "programming", color :
    "#0d665e", extensions : [".zap", ".xzap"], aliases : [], tm_scope : "source.zap",
    ace_mode : "text", language_id : 952972794u64, filenames : [], interpreters : [], },
    ZIL => { name : "ZIL", r#type : "programming", color : "#dc75e5", extensions :
    [".zil", ".mud"], aliases : [], tm_scope : "source.zil", ace_mode : "text",
    language_id : 973483626u64, filenames : [], interpreters : [], }, Zeek => { name :
    "Zeek", r#type : "programming", color : "#000000", extensions : [".zeek", ".bro"],
    aliases : ["bro"], tm_scope : "source.zeek", ace_mode : "zeek", language_id : 40u64,
    filenames : [], interpreters : [], }, ZenScript => { name : "ZenScript", r#type :
    "programming", color : "#00BCD1", extensions : [".zs"], aliases : [], tm_scope :
    "source.zenscript", ace_mode : "text", language_id : 494938890u64, filenames : [],
    interpreters : [], }, Zephir => { name : "Zephir", r#type : "programming", color :
    "#118f9e", extensions : [".zep"], aliases : [], tm_scope : "source.php.zephir",
    ace_mode : "php", language_id : 410u64, filenames : [], interpreters : [], }, Zig =>
    { name : "Zig", r#type : "programming", color : "#ec915c", extensions : [".zig",
    ".zig.zon"], aliases : [], tm_scope : "source.zig", ace_mode : "zig", language_id :
    646424281u64, filenames : [], interpreters : [], }, Zimpl => { name : "Zimpl", r#type
    : "programming", color : "#d67711", extensions : [".zimpl", ".zmpl", ".zpl"], aliases
    : [], tm_scope : "none", ace_mode : "text", language_id : 411u64, filenames : [],
    interpreters : [], }, Zmodel => { name : "Zmodel", r#type : "data", color :
    "#ff7100", extensions : [".zmodel"], aliases : [], tm_scope : "source.zmodel",
    ace_mode : "text", language_id : 803760908u64, filenames : [], interpreters : [], },
    CURLConfig => { name : "cURL Config", r#type : "data", color : "#000000", extensions
    : [], aliases : ["curlrc"], tm_scope : "source.curlrc", ace_mode : "text",
    language_id : 992375436u64, filenames : [".curlrc", "_curlrc"], group : "INI",
    interpreters : [], }, Crontab => { name : "crontab", r#type : "data", color :
    "#ead7ac", extensions : [], aliases : ["cron", "cron table"], tm_scope :
    "text.crontab", ace_mode : "tcl", language_id : 705203557u64, filenames :
    ["crontab"], interpreters : [], }, Desktop => { name : "desktop", r#type : "data",
    color : "#000000", extensions : [".desktop", ".desktop.in", ".service"], aliases :
    [], tm_scope : "source.desktop", ace_mode : "text", language_id : 412u64, filenames :
    [], interpreters : [], }, Dircolors => { name : "dircolors", r#type : "data", color :
    "#000000", extensions : [".dircolors"], aliases : [], tm_scope : "source.dircolors",
    ace_mode : "text", language_id : 691605112u64, filenames : [".dir_colors",
    ".dircolors", "DIR_COLORS", "_dir_colors", "_dircolors", "dir_colors"], interpreters
    : [], }, EC => { name : "eC", r#type : "programming", color : "#913960", extensions :
    [".ec", ".eh"], aliases : [], tm_scope : "source.c.ec", ace_mode : "text",
    language_id : 413u64, filenames : [], interpreters : [], }, Edn => { name : "edn",
    r#type : "data", color : "#000000", extensions : [".edn"], aliases : [], tm_scope :
    "source.clojure", ace_mode : "clojure", language_id : 414u64, codemirror_mode :
    "clojure", codemirror_mime_type : "application/edn", filenames : [], interpreters :
    [], }, Fish => { name : "fish", r#type : "programming", color : "#4aae47", extensions
    : [".fish"], aliases : [], tm_scope : "source.fish", ace_mode : "text", language_id :
    415u64, filenames : [], group : "Shell", interpreters : ["fish"], }, Hoon => { name :
    "hoon", r#type : "programming", color : "#00b171", extensions : [".hoon"], aliases :
    [], tm_scope : "source.hoon", ace_mode : "text", language_id : 560883276u64,
    filenames : [], interpreters : [], }, ICalendar => { name : "iCalendar", r#type :
    "data", color : "#ec564c", extensions : [".ics", ".ical"], aliases : ["iCal"],
    tm_scope : "source.iCalendar", ace_mode : "properties", language_id : 98384424u64,
    codemirror_mode : "properties", codemirror_mime_type : "text/x-properties", filenames
    : [], interpreters : [], }, Jq => { name : "jq", r#type : "programming", color :
    "#c7254e", extensions : [".jq"], aliases : [], tm_scope : "source.jq", ace_mode :
    "text", language_id : 905371884u64, filenames : [], interpreters : ["gojq", "jaq",
    "jq", "jqjq", "jqq", "query-json"], }, Kvlang => { name : "kvlang", r#type :
    "markup", color : "#1da6e0", extensions : [".kv"], aliases : [], tm_scope :
    "source.python.kivy", ace_mode : "text", language_id : 970675279u64, filenames : [],
    interpreters : [], }, MIRCScript => { name : "mIRC Script", r#type : "programming",
    color : "#3d57c3", extensions : [".mrc"], aliases : [], tm_scope : "source.msl",
    ace_mode : "text", language_id : 517654727u64, filenames : [], interpreters : [], },
    Mcfunction => { name : "mcfunction", r#type : "programming", color : "#E22837",
    extensions : [".mcfunction"], aliases : [], tm_scope : "source.mcfunction", ace_mode
    : "text", language_id : 462488745u64, filenames : [], interpreters : [], }, Mdsvex =>
    { name : "mdsvex", r#type : "markup", color : "#5f9ea0", extensions : [".svx"],
    aliases : [], tm_scope : "none", ace_mode : "markdown", language_id : 566198445u64,
    codemirror_mode : "gfm", codemirror_mime_type : "text/x-gfm", wrap : true, filenames
    : [], interpreters : [], }, Mupad => { name : "mupad", r#type : "programming", color
    : "#244963", extensions : [".mu"], aliases : [], tm_scope : "source.mupad", ace_mode
    : "text", language_id : 416u64, filenames : [], interpreters : [], }, Nanorc => {
    name : "nanorc", r#type : "data", color : "#2d004d", extensions : [".nanorc"],
    aliases : [], tm_scope : "source.nanorc", ace_mode : "text", language_id :
    775996197u64, filenames : [".nanorc", "nanorc"], group : "INI", interpreters : [], },
    NesC => { name : "nesC", r#type : "programming", color : "#94B0C7", extensions :
    [".nc"], aliases : [], tm_scope : "source.nesc", ace_mode : "text", language_id :
    417u64, filenames : [], interpreters : [], }, Ooc => { name : "ooc", r#type :
    "programming", color : "#b0b77e", extensions : [".ooc"], aliases : [], tm_scope :
    "source.ooc", ace_mode : "text", language_id : 418u64, filenames : [], interpreters :
    [], }, Q => { name : "q", r#type : "programming", color : "#0040cd", extensions :
    [".q"], aliases : [], tm_scope : "source.q", ace_mode : "text", language_id :
    970539067u64, codemirror_mode : "q", codemirror_mime_type : "text/x-q", filenames :
    [], interpreters : [], }, ReStructuredText => { name : "reStructuredText", r#type :
    "prose", color : "#141414", extensions : [".rst", ".rest", ".rest.txt", ".rst.txt"],
    aliases : ["rst"], tm_scope : "text.restructuredtext", ace_mode : "rst", language_id
    : 419u64, codemirror_mode : "rst", codemirror_mime_type : "text/x-rst", wrap : true,
    filenames : [], interpreters : [], }, Robotstxt => { name : "robots.txt", r#type :
    "data", color : "#000000", extensions : [], aliases : ["robots", "robots txt"],
    tm_scope : "text.robots-txt", ace_mode : "text", language_id : 674736065u64,
    filenames : ["robots.txt"], interpreters : [], }, Sed => { name : "sed", r#type :
    "programming", color : "#64b970", extensions : [".sed"], aliases : [], tm_scope :
    "source.sed", ace_mode : "text", language_id : 847830017u64, filenames : [],
    interpreters : ["gsed", "minised", "sed", "ssed"], }, Templ => { name : "templ",
    r#type : "markup", color : "#66D0DD", extensions : [".templ"], aliases : [], tm_scope
    : "source.templ", ace_mode : "text", language_id : 795579337u64, filenames : [],
    interpreters : [], }, VCard => { name : "vCard", r#type : "data", color : "#ee2647",
    extensions : [".vcf"], aliases : ["virtual contact file",
    "electronic business card"], tm_scope : "source.vcard", ace_mode : "properties",
    language_id : 851476558u64, codemirror_mode : "properties", codemirror_mime_type :
    "text/x-properties", filenames : [], interpreters : [], }, Wisp => { name : "wisp",
    r#type : "programming", color : "#7582D1", extensions : [".wisp"], aliases : [],
    tm_scope : "source.clojure", ace_mode : "clojure", language_id : 420u64,
    codemirror_mode : "clojure", codemirror_mime_type : "text/x-clojure", filenames : [],
    interpreters : [], }, XBase => { name : "xBase", r#type : "programming", color :
    "#403a40", extensions : [".prg", ".ch", ".prw"], aliases : ["advpl", "clipper",
    "foxpro"], tm_scope : "source.harbour", ace_mode : "text", language_id : 421u64,
    filenames : [], interpreters : [], },
}
static BY_EXTENSION: phf::Map<&'static str, &'static [fn() -> LanguageInfo]> = phf_map! {
    ".1" => & [Roff::info, RoffManpage::info], ".1in" => & [Roff::info,
    RoffManpage::info], ".1m" => & [Roff::info, RoffManpage::info], ".1x" => &
    [Roff::info, RoffManpage::info], ".2" => & [Roff::info, RoffManpage::info], ".2da" =>
    & [_2DimensionalArray::info], ".3" => & [Roff::info, RoffManpage::info], ".3in" => &
    [Roff::info, RoffManpage::info], ".3m" => & [Roff::info, RoffManpage::info], ".3p" =>
    & [Roff::info, RoffManpage::info], ".3pm" => & [Roff::info, RoffManpage::info],
    ".3qt" => & [Roff::info, RoffManpage::info], ".3x" => & [Roff::info,
    RoffManpage::info], ".4" => & [Roff::info, RoffManpage::info], ".4DForm" => &
    [JSON::info], ".4DProject" => & [JSON::info], ".4dm" => & [_4D::info], ".4gl" => &
    [Genero4gl::info], ".4th" => & [Forth::info], ".5" => & [Roff::info,
    RoffManpage::info], ".6" => & [Roff::info, RoffManpage::info], ".6pl" => &
    [Raku::info], ".6pm" => & [Raku::info], ".7" => & [Roff::info, RoffManpage::info],
    ".8" => & [Roff::info, RoffManpage::info], ".8xp" => & [TIProgram::info], ".8xp.txt"
    => & [TIProgram::info], ".9" => & [Roff::info, RoffManpage::info], ".Dsr" => &
    [VisualBasic60::info], ".JSON-tmLanguage" => & [JSON::info], ".OutJob" => &
    [AltiumDesigner::info], ".PcbDoc" => & [AltiumDesigner::info], ".PrjPCB" => &
    [AltiumDesigner::info], ".SchDoc" => & [AltiumDesigner::info], ".TextGrid" => &
    [TextGrid::info], "._coffee" => & [CoffeeScript::info], "._js" => &
    [JavaScript::info], "._ls" => & [LiveScript::info], ".a51" => & [Assembly::info],
    ".abap" => & [ABAP::info], ".abnf" => & [ABNF::info], ".action" => &
    [ROSInterface::info], ".ada" => & [Ada::info], ".adb" => & [Ada::info], ".adml" => &
    [XML::info], ".admx" => & [XML::info], ".ado" => & [Stata::info], ".adoc" => &
    [AsciiDoc::info], ".adp" => & [Tcl::info], ".ads" => & [Ada::info], ".afm" => &
    [AdobeFontMetrics::info], ".agc" => & [ApolloGuidanceComputer::info], ".agda" => &
    [Agda::info], ".ahk" => & [AutoHotkey::info], ".ahkl" => & [AutoHotkey::info],
    ".aidl" => & [AIDL::info], ".aj" => & [AspectJ::info], ".ak" => & [Aiken::info],
    ".al" => & [AL::info, Perl::info], ".als" => & [Alloy::info], ".ampl" => &
    [AMPL::info], ".angelscript" => & [AngelScript::info], ".anim" => &
    [Unity3DAsset::info], ".ant" => & [XML::info], ".antlers.html" => & [Antlers::info],
    ".antlers.php" => & [Antlers::info], ".antlers.xml" => & [Antlers::info],
    ".apacheconf" => & [ApacheConf::info], ".apex" => & [Apex::info], ".apib" => &
    [APIBlueprint::info], ".apl" => & [APL::info], ".app" => & [Erlang::info], ".app.src"
    => & [Erlang::info], ".applescript" => & [AppleScript::info], ".arc" => &
    [Arc::info], ".arpa" => & [DNSZone::info], ".arr" => & [Pyret::info], ".as" => &
    [ActionScript::info, AngelScript::info], ".asax" => & [ASPNET::info], ".asc" => &
    [AGSScript::info, AsciiDoc::info, PublicKey::info], ".asciidoc" => &
    [AsciiDoc::info], ".ascx" => & [ASPNET::info], ".asd" => & [CommonLisp::info],
    ".asddls" => & [ABAPCDS::info], ".ash" => & [AGSScript::info, KoLmafiaASH::info],
    ".ashx" => & [ASPNET::info], ".asl" => & [ASL::info], ".asm" => & [Assembly::info,
    Motorola68KAssembly::info], ".asmx" => & [ASPNET::info], ".asn" => & [ASN1::info],
    ".asn1" => & [ASN1::info], ".asp" => & [ClassicASP::info], ".aspx" => &
    [ASPNET::info], ".asset" => & [Unity3DAsset::info], ".astro" => & [Astro::info],
    ".asy" => & [Asymptote::info, LTspiceSymbol::info], ".au3" => & [AutoIt::info],
    ".aug" => & [Augeas::info], ".auk" => & [Awk::info], ".aux" => & [TeX::info], ".avdl"
    => & [AvroIDL::info], ".avsc" => & [JSON::info], ".aw" => & [PHP::info], ".awk" => &
    [Awk::info], ".axaml" => & [XML::info], ".axd" => & [ASPNET::info], ".axi" => &
    [NetLinx::info], ".axi.erb" => & [NetLinxpERB::info], ".axml" => & [XML::info],
    ".axs" => & [NetLinx::info], ".axs.erb" => & [NetLinxpERB::info], ".b" => &
    [Brainfuck::info, Limbo::info], ".bal" => & [Ballerina::info], ".bas" => &
    [B4X::info, BASIC::info, FreeBASIC::info, QuickBASIC::info, VBA::info,
    VisualBasic60::info], ".bash" => & [Shell::info], ".bat" => & [Batchfile::info],
    ".bats" => & [Shell::info], ".bb" => & [BitBake::info, BlitzBasic::info,
    Clojure::info], ".bbappend" => & [BitBake::info], ".bbclass" => & [BitBake::info],
    ".bbx" => & [TeX::info], ".bdf" => & [GlyphBitmapDistributionFormat::info], ".bdy" =>
    & [PLSQL::info], ".be" => & [Berry::info], ".befunge" => & [Befunge::info], ".bf" =>
    & [Beef::info, Befunge::info, Brainfuck::info, HyPhy::info], ".bi" => &
    [FreeBASIC::info, QuickBASIC::info], ".bib" => & [BibTeX::info], ".bibtex" => &
    [BibTeX::info], ".bicep" => & [Bicep::info], ".bicepparam" => & [Bicep::info],
    ".bison" => & [Bison::info], ".blade" => & [Blade::info], ".blade.php" => &
    [Blade::info], ".bmx" => & [BlitzMax::info], ".bones" => & [JavaScript::info], ".boo"
    => & [Boo::info], ".boot" => & [Clojure::info], ".bpl" => & [Boogie::info], ".bqn" =>
    & [BQN::info], ".brd" => & [Eagle::info, KiCadLegacyLayout::info], ".bro" => &
    [Zeek::info], ".brs" => & [Brightscript::info], ".bru" => & [Bru::info], ".bs" => &
    [Bikeshed::info, BluespecBH::info, BrighterScript::info], ".bsl" => &
    [_1CEnterprise::info], ".bst" => & [BibTeXStyle::info, BuildStream::info], ".bsv" =>
    & [Bluespec::info], ".builder" => & [Ruby::info], ".builds" => & [XML::info], ".bzl"
    => & [Starlark::info], ".c" => & [C::info], ".c++" => & [Cpp::info], ".c++-objdump"
    => & [CppObjDump::info], ".c++objdump" => & [CppObjDump::info], ".c-objdump" => &
    [CObjDump::info], ".c3" => & [C3::info], ".cabal" => & [CabalConfig::info],
    ".caddyfile" => & [Caddyfile::info], ".cairo" => & [Cairo::info, CairoZero::info],
    ".cake" => & [Csharp::info, CoffeeScript::info], ".capnp" => & [CapnProto::info],
    ".carbon" => & [Carbon::info], ".cats" => & [C::info], ".cbl" => & [COBOL::info],
    ".cbx" => & [TeX::info], ".cc" => & [Cpp::info], ".ccp" => & [COBOL::info], ".ccproj"
    => & [XML::info], ".ccxml" => & [XML::info], ".cdc" => & [Cadence::info], ".cdf" => &
    [WolframLanguage::info], ".cds" => & [CAPCDS::info], ".ceylon" => & [Ceylon::info],
    ".cfc" => & [ColdFusionCFC::info], ".cfg" => & [HAProxy::info, INI::info], ".cfm" =>
    & [ColdFusion::info], ".cfml" => & [ColdFusion::info], ".cgi" => & [Perl::info,
    Python::info, Shell::info], ".cginc" => & [HLSL::info], ".ch" => & [Charity::info,
    XBase::info], ".chem" => & [Pic::info], ".chpl" => & [Chapel::info], ".chs" => &
    [C2hsHaskell::info], ".cil" => & [CIL::info], ".circom" => & [Circom::info], ".cirru"
    => & [Cirru::info], ".cj" => & [Cangjie::info], ".cjs" => & [JavaScript::info],
    ".cjsx" => & [CoffeeScript::info], ".ck" => & [ChucK::info], ".cl" => &
    [CommonLisp::info, Cool::info, OpenCL::info], ".cl2" => & [Clojure::info], ".clar" =>
    & [Clarity::info], ".click" => & [Click::info], ".clixml" => & [XML::info], ".clj" =>
    & [Clojure::info], ".cljc" => & [Clojure::info], ".cljs" => & [Clojure::info],
    ".cljs.hl" => & [Clojure::info], ".cljscm" => & [Clojure::info], ".cljx" => &
    [Clojure::info], ".clp" => & [CLIPS::info], ".cls" => & [Apex::info,
    ObjectScript::info, OpenEdgeABL::info, TeX::info, VBA::info, VisualBasic60::info],
    ".clue" => & [Clue::info], ".clw" => & [Clarion::info], ".cmake" => & [CMake::info],
    ".cmake.in" => & [CMake::info], ".cmd" => & [Batchfile::info], ".cmp" => &
    [GerberImage::info], ".cnc" => & [GCode::info], ".cnf" => & [INI::info], ".cob" => &
    [COBOL::info], ".cobol" => & [COBOL::info], ".cocci" => & [SmPL::info],
    ".code-snippets" => & [JSONWithComments::info], ".code-workspace" => &
    [JSONWithComments::info], ".coffee" => & [CoffeeScript::info], ".coffee.md" => &
    [LiterateCoffeeScript::info], ".com" => & [DIGITALCommandLanguage::info], ".command"
    => & [Shell::info], ".conll" => & [CoNLLU::info], ".conllu" => & [CoNLLU::info],
    ".containerfile" => & [Dockerfile::info], ".cook" => & [Cooklang::info], ".coq" => &
    [RocqProver::info], ".cp" => & [Cpp::info, ComponentPascal::info], ".cpp" => &
    [Cpp::info], ".cpp-objdump" => & [CppObjDump::info], ".cppm" => & [Cpp::info],
    ".cppobjdump" => & [CppObjDump::info], ".cproject" => & [XML::info], ".cps" => &
    [ComponentPascal::info], ".cpy" => & [COBOL::info], ".cql" => & [SQL::info], ".cr" =>
    & [Crystal::info], ".crc32" => & [Checksums::info], ".creole" => & [Creole::info],
    ".cs" => & [Csharp::info, Smalltalk::info], ".cs.pp" => & [Csharp::info], ".csc" => &
    [GSC::info], ".cscfg" => & [XML::info], ".csd" => & [CsoundDocument::info], ".csdef"
    => & [XML::info], ".csh" => & [Tcsh::info], ".cshtml" => & [HTMLpRazor::info], ".csl"
    => & [Kusto::info, XML::info], ".cson" => & [CSON::info], ".csproj" => & [XML::info],
    ".css" => & [CSS::info], ".csv" => & [CSV::info], ".csx" => & [Csharp::info], ".ct"
    => & [XML::info], ".ctl" => & [VisualBasic60::info], ".ctp" => & [PHP::info], ".cts"
    => & [TypeScript::info], ".cu" => & [Cuda::info], ".cue" => & [CUE::info,
    CueSheet::info], ".cuh" => & [Cuda::info], ".curry" => & [Curry::info], ".cw" => &
    [Redcode::info], ".cwl" => & [CommonWorkflowLanguage::info], ".cxx" => & [Cpp::info],
    ".cxx-objdump" => & [CppObjDump::info], ".cy" => & [Cycript::info], ".cylc" => &
    [Cylc::info], ".cyp" => & [Cypher::info], ".cypher" => & [Cypher::info], ".d" => &
    [D::info, DTrace::info, Makefile::info], ".d-objdump" => & [DObjDump::info], ".d2" =>
    & [D2::info], ".dae" => & [COLLADA::info], ".darcspatch" => & [DarcsPatch::info],
    ".dart" => & [Dart::info], ".das" => & [Daslang::info], ".dats" => & [ATS::info],
    ".db2" => & [SQLPL::info], ".dcl" => & [Clean::info], ".ddl" => & [PLSQL::info,
    SQL::info], ".decls" => & [BlitzBasic::info], ".depproj" => & [XML::info], ".desktop"
    => & [Desktop::info], ".desktop.in" => & [Desktop::info], ".dfm" => & [Pascal::info],
    ".dfy" => & [Dafny::info], ".dhall" => & [Dhall::info], ".di" => & [D::info], ".diff"
    => & [Diff::info], ".dircolors" => & [Dircolors::info], ".dita" => & [XML::info],
    ".ditamap" => & [XML::info], ".ditaval" => & [XML::info], ".djs" => &
    [Dogescript::info], ".dll.config" => & [XML::info], ".dlm" => & [IDL::info], ".dm" =>
    & [DM::info], ".do" => & [Stata::info], ".dockerfile" => & [Dockerfile::info], ".dof"
    => & [INI::info], ".doh" => & [Stata::info], ".dot" => & [GraphvizDOT::info],
    ".dotsettings" => & [XML::info], ".dpatch" => & [DarcsPatch::info], ".dpr" => &
    [Pascal::info], ".druby" => & [Mirah::info], ".dsc" => &
    [DebianPackageControlFile::info, DenizenScript::info], ".dsl" => & [ASL::info],
    ".dsp" => & [Faust::info, MicrosoftDeveloperStudioProject::info], ".dtx" => &
    [TeX::info], ".duby" => & [Mirah::info], ".dwl" => & [DataWeave::info], ".dyalog" =>
    & [APL::info], ".dyl" => & [Dylan::info], ".dylan" => & [Dylan::info], ".dzn" => &
    [MiniZincData::info], ".e" => & [E::info, Eiffel::info, Euphoria::info], ".eam.fs" =>
    & [Formatted::info], ".eb" => & [Easybuild::info], ".ebnf" => & [EBNF::info],
    ".ebuild" => & [GentooEbuild::info], ".ec" => & [EC::info], ".ecl" => & [ECL::info,
    ECLiPSe::info], ".eclass" => & [GentooEclass::info], ".eclxml" => & [ECL::info],
    ".ecr" => & [HTMLpECR::info], ".ect" => & [EJS::info], ".edc" => &
    [EdjeDataCollection::info], ".edge" => & [Edge::info], ".edgeql" => & [EdgeQL::info],
    ".editorconfig" => & [EditorConfig::info], ".edn" => & [Edn::info], ".eh" => &
    [EC::info], ".ejs" => & [EJS::info], ".ejs.t" => & [EJS::info], ".el" => &
    [EmacsLisp::info], ".eliom" => & [OCaml::info], ".eliomi" => & [OCaml::info], ".elm"
    => & [Elm::info], ".elv" => & [Elvish::info], ".em" => & [EmberScript::info],
    ".emacs" => & [EmacsLisp::info], ".emacs.desktop" => & [EmacsLisp::info],
    ".emberscript" => & [EmberScript::info], ".eml" => & [EMail::info], ".env" => &
    [Dotenv::info], ".epj" => & [EcereProjects::info], ".eps" => & [PostScript::info],
    ".epsi" => & [PostScript::info], ".eq" => & [EQ::info], ".erb" => & [HTMLpERB::info],
    ".erb.deface" => & [HTMLpERB::info], ".erl" => & [Erlang::info], ".es" => &
    [Erlang::info, JavaScript::info], ".es6" => & [JavaScript::info], ".escript" => &
    [Erlang::info], ".esdl" => & [EdgeQL::info], ".ets" => & [ArkTS::info], ".ex" => &
    [Elixir::info, Euphoria::info], ".exs" => & [Elixir::info], ".eye" => & [Ruby::info],
    ".f" => & [FilebenchWML::info, Forth::info, Fortran::info], ".f03" => &
    [FortranFreeForm::info], ".f08" => & [FortranFreeForm::info], ".f77" => &
    [Fortran::info], ".f90" => & [FortranFreeForm::info], ".f95" => &
    [FortranFreeForm::info], ".factor" => & [Factor::info], ".fan" => & [Fantom::info],
    ".fancypack" => & [Fancy::info], ".fcgi" => & [Lua::info, PHP::info, Perl::info,
    Python::info, Ruby::info, Shell::info], ".fea" => & [OpenTypeFeatureFile::info],
    ".feature" => & [Gherkin::info], ".filters" => & [XML::info], ".fir" => &
    [FIRRTL::info], ".fish" => & [Fish::info], ".flex" => & [JFlex::info], ".flf" => &
    [FIGletFont::info], ".flix" => & [Flix::info], ".flux" => & [FLUX::info], ".fnc" => &
    [PLSQL::info], ".fnl" => & [Fennel::info], ".for" => & [Formatted::info, Forth::info,
    Fortran::info], ".forth" => & [Forth::info], ".fp" => & [GLSL::info], ".fpp" => &
    [Fortran::info], ".fr" => & [Forth::info, Frege::info, Text::info], ".frag" => &
    [GLSL::info, JavaScript::info], ".frg" => & [GLSL::info], ".frm" => & [INI::info,
    VBA::info, VisualBasic60::info], ".frt" => & [Forth::info], ".fs" => & [Fsharp::info,
    Filterscript::info, Forth::info, GLSL::info], ".fsh" => & [GLSL::info], ".fshader" =>
    & [GLSL::info], ".fsi" => & [Fsharp::info], ".fsproj" => & [XML::info], ".fst" => &
    [Fstar::info], ".fsti" => & [Fstar::info], ".fsx" => & [Fsharp::info], ".fth" => &
    [Forth::info], ".ftl" => & [Fluent::info, FreeMarker::info], ".ftlh" => &
    [FreeMarker::info], ".fun" => & [StandardML::info], ".fut" => & [Futhark::info],
    ".fx" => & [FLUX::info, HLSL::info], ".fxh" => & [HLSL::info], ".fxml" => &
    [XML::info], ".fy" => & [Fancy::info], ".g" => & [GCode::info, GAP::info], ".g4" => &
    [ANTLR::info], ".gaml" => & [GAML::info], ".gap" => & [GAP::info], ".gawk" => &
    [Awk::info], ".gbl" => & [GerberImage::info], ".gbo" => & [GerberImage::info], ".gbp"
    => & [GerberImage::info], ".gbr" => & [GerberImage::info], ".gbs" => &
    [GerberImage::info], ".gco" => & [GCode::info], ".gcode" => & [GCode::info], ".gd" =>
    & [GAP::info, GDScript::info], ".gdb" => & [GDB::info], ".gdbinit" => & [GDB::info],
    ".gdnlib" => & [GodotResource::info], ".gdns" => & [GodotResource::info], ".gdshader"
    => & [GDShader::info], ".gdshaderinc" => & [GDShader::info], ".ged" => &
    [GEDCOM::info], ".gemspec" => & [Ruby::info], ".geo" => & [GLSL::info], ".geojson" =>
    & [JSON::info], ".geom" => & [GLSL::info], ".gf" => & [GrammaticalFramework::info],
    ".gi" => & [GAP::info], ".gitconfig" => & [GitConfig::info], ".gitignore" => &
    [IgnoreList::info], ".gjs" => & [GlimmerJS::info], ".gko" => & [GerberImage::info],
    ".glade" => & [XML::info], ".gleam" => & [Gleam::info], ".glf" => & [Glyph::info],
    ".glsl" => & [GLSL::info], ".glslf" => & [GLSL::info], ".glslv" => & [GLSL::info],
    ".gltf" => & [JSON::info], ".glyphs" => & [OpenStepPropertyList::info], ".gmi" => &
    [Gemini::info], ".gml" => & [GameMakerLanguage::info, GerberImage::info,
    GraphModelingLanguage::info, XML::info], ".gms" => & [GAMS::info], ".gmx" => &
    [XML::info], ".gn" => & [GN::info], ".gni" => & [GN::info], ".gnu" => &
    [Gnuplot::info], ".gnuplot" => & [Gnuplot::info], ".go" => & [Go::info], ".god" => &
    [Ruby::info], ".gohtml" => & [GoTemplate::info], ".golo" => & [Golo::info], ".gotmpl"
    => & [GoTemplate::info], ".gp" => & [Gnuplot::info], ".gpb" => & [GerberImage::info],
    ".gpt" => & [GerberImage::info], ".gpx" => & [XML::info], ".gql" => &
    [GraphQL::info], ".grace" => & [Grace::info], ".gradle" => & [Gradle::info],
    ".gradle.kts" => & [GradleKotlinDSL::info], ".graphql" => & [GraphQL::info],
    ".graphqls" => & [GraphQL::info], ".groovy" => & [Groovy::info], ".grt" => &
    [Groovy::info], ".grxml" => & [XML::info], ".gs" => & [GLSL::info, Genie::info,
    Gosu::info, JavaScript::info], ".gsc" => & [GSC::info], ".gsh" => & [GSC::info],
    ".gshader" => & [GLSL::info], ".gsp" => & [GroovyServerPages::info], ".gst" => &
    [Gosu::info, XML::info], ".gsx" => & [Gosu::info], ".gtl" => & [GerberImage::info],
    ".gto" => & [GerberImage::info], ".gtp" => & [GerberImage::info], ".gtpl" => &
    [Groovy::info], ".gts" => & [GerberImage::info, GlimmerTS::info], ".gv" => &
    [GraphvizDOT::info], ".gvy" => & [Groovy::info], ".gyp" => & [Python::info], ".gypi"
    => & [Python::info], ".h" => & [C::info, Cpp::info, ObjectiveC::info], ".h++" => &
    [Cpp::info], ".h.in" => & [C::info], ".ha" => & [Hare::info], ".hack" => &
    [Hack::info], ".haml" => & [Haml::info], ".haml.deface" => & [Haml::info],
    ".handlebars" => & [Handlebars::info], ".har" => & [JSON::info], ".hats" => &
    [ATS::info], ".hb" => & [Harbour::info], ".hbs" => & [Handlebars::info], ".hc" => &
    [HolyC::info], ".hcl" => & [HCL::info], ".heex" => & [HTMLpEEX::info], ".hh" => &
    [Cpp::info, Hack::info], ".hhi" => & [Hack::info], ".hic" => & [Clojure::info],
    ".hip" => & [HIP::info], ".hlean" => & [Lean::info], ".hlsl" => & [HLSL::info],
    ".hlsli" => & [HLSL::info], ".hocon" => & [HOCON::info], ".hoon" => & [Hoon::info],
    ".hpp" => & [Cpp::info], ".hqf" => & [SQF::info], ".hql" => & [HiveQL::info], ".hrl"
    => & [Erlang::info], ".hs" => & [Haskell::info], ".hs-boot" => & [Haskell::info],
    ".hsc" => & [Haskell::info], ".hta" => & [HTML::info], ".htm" => & [HTML::info],
    ".html" => & [Ecmarkup::info, HTML::info], ".html.eex" => & [HTMLpEEX::info],
    ".html.hl" => & [HTML::info], ".html.tmpl" => & [GoTemplate::info], ".http" => &
    [HTTP::info], ".hurl" => & [Hurl::info], ".hx" => & [Haxe::info], ".hxml" => &
    [HXML::info], ".hxsl" => & [Haxe::info], ".hxx" => & [Cpp::info], ".hy" => &
    [Hy::info], ".hzp" => & [XML::info], ".i" => & [Assembly::info,
    Motorola68KAssembly::info, SWIG::info], ".i3" => & [Modula3::info], ".i7x" => &
    [Inform7::info], ".ical" => & [ICalendar::info], ".ice" => & [JSON::info,
    Slice::info], ".iced" => & [CoffeeScript::info], ".icl" => & [Clean::info], ".ics" =>
    & [ICalendar::info], ".idc" => & [C::info], ".idr" => & [Idris::info], ".ig" => &
    [Modula3::info], ".ihlp" => & [Stata::info], ".ijm" => & [ImageJMacro::info], ".ijs"
    => & [J::info], ".ik" => & [Ioke::info], ".ily" => & [LilyPond::info], ".imba" => &
    [Imba::info], ".iml" => & [XML::info], ".inc" => & [Assembly::info, BitBake::info,
    Cpp::info, HTML::info, Motorola68KAssembly::info, NASL::info, PHP::info,
    POVRaySDL::info, Pascal::info, Pawn::info, SQL::info, SourcePawn::info], ".ini" => &
    [INI::info], ".ink" => & [Ink::info], ".inl" => & [Cpp::info], ".ino" => &
    [Cpp::info], ".ins" => & [TeX::info], ".intr" => & [Dylan::info], ".io" => &
    [Io::info], ".iol" => & [Jolie::info], ".ipf" => & [IGORPro::info], ".ipp" => &
    [Cpp::info], ".ipynb" => & [JupyterNotebook::info], ".irclog" => & [IRCLog::info],
    ".isl" => & [InnoSetup::info], ".ispc" => & [ISPC::info], ".iss" => &
    [InnoSetup::info], ".iuml" => & [PlantUML::info], ".ivy" => & [XML::info], ".ixx" =>
    & [Cpp::info], ".j" => & [Jasmin::info, ObjectiveJ::info], ".j2" => & [Jinja::info],
    ".jade" => & [Pug::info], ".jai" => & [Jai::info], ".jake" => & [JavaScript::info],
    ".janet" => & [Janet::info], ".jav" => & [Java::info], ".java" => & [Java::info],
    ".javascript" => & [JavaScript::info], ".jbuilder" => & [Ruby::info], ".jcl" => &
    [JCL::info], ".jelly" => & [XML::info], ".jflex" => & [JFlex::info], ".jinja" => &
    [Jinja::info], ".jinja2" => & [Jinja::info], ".jison" => & [Jison::info], ".jisonlex"
    => & [JisonLex::info], ".jl" => & [Julia::info], ".jq" => & [JSONiq::info, Jq::info],
    ".js" => & [JavaScript::info], ".js.erb" => & [JavaScriptpERB::info], ".jsb" => &
    [JavaScript::info], ".jscad" => & [JavaScript::info], ".jsfl" => &
    [JavaScript::info], ".jsh" => & [Java::info], ".jslib" => & [JavaScript::info],
    ".jsm" => & [JavaScript::info], ".json" => & [JSON::info, OASv2Json::info,
    OASv3Json::info], ".json.example" => & [JSON::info], ".json5" => & [JSON5::info],
    ".jsonc" => & [JSONWithComments::info], ".jsonl" => & [JSON::info], ".jsonld" => &
    [JSONLD::info], ".jsonnet" => & [Jsonnet::info], ".jsp" => & [JavaServerPages::info],
    ".jspre" => & [JavaScript::info], ".jsproj" => & [XML::info], ".jss" => &
    [JavaScript::info], ".jst" => & [EJS::info], ".jsx" => & [JavaScript::info], ".jte"
    => & [JavaTemplateEngine::info], ".just" => & [Just::info], ".k" => & [KCL::info,
    KFramework::info], ".kak" => & [KakouneScript::info], ".kdl" => & [KDL::info],
    ".kicad_mod" => & [KiCadLayout::info], ".kicad_pcb" => & [KiCadLayout::info],
    ".kicad_sch" => & [KiCadSchematic::info], ".kicad_sym" => & [KiCadSchematic::info],
    ".kicad_wks" => & [KiCadLayout::info], ".kid" => & [Genshi::info], ".kit" => &
    [Kit::info], ".kk" => & [Koka::info], ".kml" => & [XML::info], ".kojo" => &
    [Scala::info], ".kql" => & [Kusto::info], ".krl" => & [KRL::info], ".ks" => &
    [KerboScript::info, Kickstart::info], ".ksh" => & [Shell::info], ".ksy" => &
    [KaitaiStruct::info], ".kt" => & [Kotlin::info], ".ktm" => & [Kotlin::info], ".kts"
    => & [Kotlin::info], ".kv" => & [Kvlang::info], ".l" => & [CommonLisp::info,
    Lex::info, PicoLisp::info, Roff::info], ".lagda" => & [LiterateAgda::info],
    ".langium" => & [Langium::info], ".lark" => & [Lark::info], ".las" => &
    [Lasso::info], ".lasso" => & [Lasso::info], ".lasso8" => & [Lasso::info], ".lasso9"
    => & [Lasso::info], ".latte" => & [Latte::info], ".launch" => & [XML::info], ".lbx"
    => & [TeX::info], ".ld" => & [LinkerScript::info], ".lds" => & [LinkerScript::info],
    ".lean" => & [Lean::info, Lean4::info], ".leex" => & [HTMLpEEX::info],
    ".lektorproject" => & [INI::info], ".leo" => & [Leo::info], ".less" => &
    [Less::info], ".lex" => & [Lex::info], ".lfe" => & [LFE::info], ".lgt" => &
    [Logtalk::info], ".lhs" => & [LiterateHaskell::info], ".libsonnet" => &
    [Jsonnet::info], ".lid" => & [Dylan::info], ".lidr" => & [Idris::info], ".ligo" => &
    [LigoLANG::info], ".linq" => & [Csharp::info], ".liquid" => & [Liquid::info], ".lisp"
    => & [CommonLisp::info, NewLisp::info], ".litcoffee" => &
    [LiterateCoffeeScript::info], ".livecodescript" => & [LiveCodeScript::info],
    ".livemd" => & [Markdown::info], ".lkml" => & [LookML::info], ".ll" => &
    [LLVM::info], ".lmi" => & [Python::info], ".logtalk" => & [Logtalk::info], ".lol" =>
    & [LOLCODE::info], ".lookml" => & [LookML::info], ".lp" => &
    [AnswerSetProgramming::info, LinearProgramming::info], ".lpr" => & [Pascal::info],
    ".ls" => & [LiveScript::info, LoomScript::info], ".lsl" => & [LSL::info], ".lslp" =>
    & [LSL::info], ".lsp" => & [CommonLisp::info, NewLisp::info], ".ltx" => &
    [TeX::info], ".lua" => & [Lua::info], ".luau" => & [Luau::info], ".lvclass" => &
    [LabVIEW::info], ".lvlib" => & [LabVIEW::info], ".lvproj" => & [LabVIEW::info], ".ly"
    => & [LilyPond::info], ".m" => & [Limbo::info, M::info, MATLAB::info, MUF::info,
    Mercury::info, ObjectiveC::info, WolframLanguage::info], ".m2" => &
    [Macaulay2::info], ".m3" => & [Modula3::info], ".m3u" => & [M3U::info], ".m3u8" => &
    [M3U::info], ".m4" => & [M4::info, M4Sugar::info], ".ma" => &
    [WolframLanguage::info], ".mak" => & [Makefile::info], ".make" => & [Makefile::info],
    ".makefile" => & [Makefile::info], ".mako" => & [Mako::info], ".man" => &
    [Roff::info, RoffManpage::info], ".mao" => & [Mako::info], ".markdown" => &
    [Markdown::info], ".marko" => & [Marko::info], ".mask" => & [Mask::info,
    Unity3DAsset::info], ".mat" => & [Unity3DAsset::info], ".mata" => & [Stata::info],
    ".matah" => & [Stata::info], ".mathematica" => & [WolframLanguage::info], ".matlab"
    => & [MATLAB::info], ".mawk" => & [Awk::info], ".maxhelp" => & [Max::info], ".maxpat"
    => & [Max::info], ".maxproj" => & [Max::info], ".mbox" => & [EMail::info], ".mbt" =>
    & [MoonBit::info], ".mc" => & [M4::info, MonkeyC::info, Win32MessageFile::info],
    ".mcfunction" => & [Mcfunction::info], ".mcmeta" => & [JSON::info], ".mcr" => &
    [MAXScript::info], ".md" => & [GCCMachineDescription::info, Markdown::info], ".md2"
    => & [Checksums::info], ".md4" => & [Checksums::info], ".md5" => & [Checksums::info],
    ".mdoc" => & [Roff::info, RoffManpage::info], ".mdown" => & [Markdown::info],
    ".mdpolicy" => & [XML::info], ".mdwn" => & [Markdown::info], ".mdx" => & [MDX::info],
    ".me" => & [Roff::info], ".mediawiki" => & [Wikitext::info], ".mermaid" => &
    [Mermaid::info], ".meta" => & [Unity3DAsset::info], ".metal" => & [Metal::info],
    ".mg" => & [Modula3::info], ".minid" => & [MiniD::info], ".mint" => & [Mint::info],
    ".mir" => & [YAML::info], ".mirah" => & [Mirah::info], ".mjml" => & [XML::info],
    ".mjs" => & [JavaScript::info], ".mk" => & [Makefile::info], ".mkd" => &
    [Markdown::info], ".mkdn" => & [Markdown::info], ".mkdown" => & [Markdown::info],
    ".mkfile" => & [Makefile::info], ".mkii" => & [TeX::info], ".mkiv" => & [TeX::info],
    ".mkvi" => & [TeX::info], ".ml" => & [OCaml::info, StandardML::info], ".ml4" => &
    [OCaml::info], ".mli" => & [OCaml::info], ".mligo" => & [CameLIGO::info], ".mlir" =>
    & [MLIR::info], ".mll" => & [OCaml::info], ".mly" => & [OCaml::info], ".mm" => &
    [ObjectiveCpp::info, XML::info], ".mmd" => & [Mermaid::info], ".mmk" => &
    [ModuleManagementSystem::info], ".mms" => & [ModuleManagementSystem::info], ".mo" =>
    & [Modelica::info, Motoko::info], ".mod" => & [AMPL::info, LinuxKernelModule::info,
    Modula2::info, NMODL::info, XML::info], ".mojo" => & [Mojo::info, XML::info],
    ".monkey" => & [Monkey::info], ".monkey2" => & [Monkey::info], ".moo" => &
    [Mercury::info, Moocode::info], ".moon" => & [MoonScript::info], ".move" => &
    [MoveLang::info], ".mpl" => & [JetBrainsMPS::info], ".mps" => & [JetBrainsMPS::info],
    ".mq4" => & [MQL4::info], ".mq5" => & [MQL5::info], ".mqh" => & [MQL4::info,
    MQL5::info], ".mrc" => & [MIRCScript::info], ".ms" => & [MAXScript::info, Roff::info,
    UnixAssembly::info], ".msd" => & [JetBrainsMPS::info], ".msg" => & [OMNeTppMSG::info,
    ROSInterface::info], ".mspec" => & [Ruby::info], ".mss" => & [CartoCSS::info], ".mt"
    => & [WolframLanguage::info], ".mtl" => & [WavefrontMaterial::info], ".mtml" => &
    [MTML::info], ".mts" => & [TypeScript::info], ".mu" => & [Mupad::info], ".mud" => &
    [ZIL::info], ".muf" => & [MUF::info], ".mumps" => & [M::info], ".muse" => &
    [Muse::info], ".mustache" => & [Mustache::info], ".mxml" => & [XML::info], ".mxt" =>
    & [Max::info], ".mysql" => & [SQL::info], ".myt" => & [Myghty::info], ".mzn" => &
    [MiniZinc::info], ".n" => & [Nemerle::info, Roff::info], ".nanorc" => &
    [Nanorc::info], ".nas" => & [Assembly::info, Nasal::info], ".nasl" => & [NASL::info],
    ".nasm" => & [Assembly::info], ".natvis" => & [XML::info], ".nawk" => & [Awk::info],
    ".nb" => & [Text::info, WolframLanguage::info], ".nbp" => & [WolframLanguage::info],
    ".nc" => & [NesC::info], ".ncl" => & [GerberImage::info, NCL::info, Nickel::info,
    Text::info, XML::info], ".ndproj" => & [XML::info], ".ne" => & [Nearley::info],
    ".nearley" => & [Nearley::info], ".ned" => & [OMNeTppNED::info], ".neon" => &
    [NEON::info], ".nf" => & [Nextflow::info], ".nginx" => & [Nginx::info], ".nginxconf"
    => & [Nginx::info], ".ni" => & [Inform7::info], ".nim" => & [Nim::info], ".nim.cfg"
    => & [Nim::info], ".nimble" => & [Nim::info], ".nimrod" => & [Nim::info], ".nims" =>
    & [Nim::info], ".ninja" => & [Ninja::info], ".nit" => & [Nit::info], ".nix" => &
    [Nix::info], ".njk" => & [Nunjucks::info], ".njs" => & [JavaScript::info], ".nl" => &
    [NL::info, NewLisp::info], ".nlogo" => & [NetLogo::info], ".no" => & [Text::info],
    ".nomad" => & [HCL::info], ".nproj" => & [XML::info], ".nqp" => & [Raku::info], ".nr"
    => & [Noir::info, Roff::info], ".nse" => & [Lua::info], ".nsh" => & [NSIS::info],
    ".nsi" => & [NSIS::info], ".nss" => & [NWScript::info], ".nu" => & [Nu::info,
    Nushell::info], ".numpy" => & [NumPy::info], ".numpyw" => & [NumPy::info], ".numsc"
    => & [NumPy::info], ".nuspec" => & [XML::info], ".nut" => & [Squirrel::info], ".ny"
    => & [CommonLisp::info], ".ob2" => & [Oberon::info], ".obj" => &
    [WavefrontObject::info], ".objdump" => & [ObjDump::info], ".odd" => & [XML::info],
    ".odin" => & [ObjectDataInstanceNotation::info, Odin::info], ".ol" => &
    [Jolie::info], ".omgrofl" => & [Omgrofl::info], ".ooc" => & [Ooc::info], ".opa" => &
    [Opa::info], ".opal" => & [Opal::info], ".opencl" => & [OpenCL::info], ".orc" => &
    [Csound::info], ".org" => & [Org::info], ".os" => & [_1CEnterprise::info], ".osm" =>
    & [XML::info], ".overpassql" => & [OverpassQL::info], ".owl" => &
    [WebOntologyLanguage::info], ".ox" => & [Ox::info], ".oxh" => & [Ox::info], ".oxo" =>
    & [Ox::info], ".oxygene" => & [Oxygene::info], ".oz" => & [Oz::info], ".p" => &
    [Gnuplot::info, OpenEdgeABL::info], ".p4" => & [P4::info], ".p6" => & [Raku::info],
    ".p6l" => & [Raku::info], ".p6m" => & [Raku::info], ".p8" => & [Lua::info], ".pac" =>
    & [JavaScript::info], ".pact" => & [Pact::info], ".pan" => & [Pan::info], ".parrot"
    => & [Parrot::info], ".pas" => & [Pascal::info], ".pascal" => & [Pascal::info],
    ".pasm" => & [ParrotAssembly::info], ".pat" => & [Max::info], ".patch" => &
    [Diff::info], ".pb" => & [PureBasic::info], ".pbi" => & [PureBasic::info], ".pbt" =>
    & [PowerBuilder::info, ProtocolBufferTextFormat::info], ".pbtxt" => &
    [ProtocolBufferTextFormat::info], ".pck" => & [PLSQL::info], ".pcss" => &
    [PostCSS::info], ".pd" => & [PureData::info], ".pd_lua" => & [Lua::info], ".pddl" =>
    & [PDDL::info], ".pde" => & [Processing::info], ".peggy" => & [PEGjs::info], ".pegjs"
    => & [PEGjs::info], ".pep" => & [Pep8::info], ".per" => & [GeneroPer::info], ".perl"
    => & [Perl::info], ".pfa" => & [PostScript::info], ".pgsql" => & [PLpgSQL::info],
    ".ph" => & [Perl::info], ".php" => & [Hack::info, PHP::info], ".php3" => &
    [PHP::info], ".php4" => & [PHP::info], ".php5" => & [PHP::info], ".phps" => &
    [PHP::info], ".phpt" => & [PHP::info], ".phtml" => & [HTMLpPHP::info], ".pic" => &
    [Pic::info], ".pig" => & [PigLatin::info], ".pike" => & [Pike::info], ".pir" => &
    [ParrotInternalRepresentation::info], ".pkb" => & [PLSQL::info], ".pkgproj" => &
    [XML::info], ".pkl" => & [Pickle::info, Pkl::info], ".pks" => & [PLSQL::info], ".pl"
    => & [Perl::info, Prolog::info, Raku::info], ".pl6" => & [Raku::info], ".plantuml" =>
    & [PlantUML::info], ".plb" => & [PLSQL::info], ".plist" => &
    [OpenStepPropertyList::info, XMLPropertyList::info], ".plot" => & [Gnuplot::info],
    ".pls" => & [PLSQL::info], ".plsql" => & [PLSQL::info], ".plt" => & [Gnuplot::info,
    Prolog::info], ".pluginspec" => & [Ruby::info, XML::info], ".plx" => & [Perl::info],
    ".pm" => & [Perl::info, Raku::info, XPixMap::info], ".pm6" => & [Raku::info], ".pml"
    => & [Promela::info], ".pmod" => & [Pike::info], ".po" => & [GettextCatalog::info],
    ".pod" => & [Pod::info, Pod6::info], ".pod6" => & [Pod6::info], ".podsl" => &
    [CommonLisp::info], ".podspec" => & [Ruby::info], ".pogo" => & [PogoScript::info],
    ".polar" => & [Polar::info], ".pony" => & [Pony::info], ".por" => & [Portugol::info],
    ".postcss" => & [PostCSS::info], ".pot" => & [GettextCatalog::info], ".pov" => &
    [POVRaySDL::info], ".pp" => & [Pascal::info, Puppet::info], ".pprx" => &
    [REXX::info], ".praat" => & [Praat::info], ".prawn" => & [Ruby::info], ".prc" => &
    [PLSQL::info, SQL::info], ".prefab" => & [Unity3DAsset::info], ".prefs" => &
    [INI::info], ".prg" => & [XBase::info], ".pri" => & [QMake::info], ".prisma" => &
    [Prisma::info], ".pro" => & [IDL::info, INI::info, Proguard::info, Prolog::info,
    QMake::info], ".proj" => & [XML::info], ".prolog" => & [Prolog::info], ".properties"
    => & [INI::info, JavaProperties::info], ".props" => & [XML::info], ".proto" => &
    [ProtocolBuffer::info], ".prw" => & [XBase::info], ".ps" => & [PostScript::info],
    ".ps1" => & [PowerShell::info], ".ps1xml" => & [XML::info], ".psc" => &
    [Papyrus::info], ".psc1" => & [XML::info], ".psd1" => & [PowerShell::info], ".psgi"
    => & [Perl::info], ".psm1" => & [PowerShell::info], ".pt" => & [XML::info], ".pub" =>
    & [PublicKey::info], ".pug" => & [Pug::info], ".puml" => & [PlantUML::info], ".purs"
    => & [PureScript::info], ".pwn" => & [Pawn::info], ".pxd" => & [Cython::info], ".pxi"
    => & [Cython::info], ".py" => & [Python::info], ".py3" => & [Python::info], ".pyde"
    => & [Python::info], ".pyi" => & [Python::info], ".pyp" => & [Python::info], ".pyt"
    => & [Python::info], ".pytb" => & [PythonTraceback::info], ".pyw" => &
    [Python::info], ".pyx" => & [Cython::info], ".q" => & [HiveQL::info, Q::info],
    ".qasm" => & [OpenQASM::info], ".qbs" => & [QML::info], ".qc" => & [QuakeC::info],
    ".qhelp" => & [XML::info], ".ql" => & [CodeQL::info], ".qll" => & [CodeQL::info],
    ".qmd" => & [RMarkdown::info], ".qml" => & [QML::info], ".qs" => & [Qsharp::info,
    QtScript::info], ".r" => & [R::info, Rebol::info, Rez::info], ".r2" => &
    [Rebol::info], ".r3" => & [Rebol::info], ".rabl" => & [Ruby::info], ".rake" => &
    [Ruby::info], ".raku" => & [Raku::info], ".rakumod" => & [Raku::info], ".raml" => &
    [RAML::info], ".raw" => & [RawTokenData::info], ".razor" => & [HTMLpRazor::info],
    ".rb" => & [Ruby::info], ".rbbas" => & [REALbasic::info], ".rbfrm" => &
    [REALbasic::info], ".rbi" => & [Ruby::info], ".rbmnu" => & [REALbasic::info],
    ".rbres" => & [REALbasic::info], ".rbs" => & [RBS::info], ".rbtbar" => &
    [REALbasic::info], ".rbuild" => & [Ruby::info], ".rbuistate" => & [REALbasic::info],
    ".rbw" => & [Ruby::info], ".rbx" => & [Ruby::info], ".rbxs" => & [Lua::info],
    ".rchit" => & [GLSL::info], ".rd" => & [R::info], ".rdf" => & [XML::info], ".rdoc" =>
    & [RDoc::info], ".re" => & [Cpp::info, Reason::info], ".reb" => & [Rebol::info],
    ".rebol" => & [Rebol::info], ".red" => & [Red::info], ".reds" => & [Red::info],
    ".reek" => & [YAML::info], ".reg" => & [WindowsRegistryEntries::info], ".regex" => &
    [RegularExpression::info], ".regexp" => & [RegularExpression::info], ".rego" => &
    [OpenPolicyAgent::info], ".rei" => & [Reason::info], ".religo" => &
    [ReasonLIGO::info], ".res" => & [ReScript::info, XML::info], ".resi" => &
    [ReScript::info], ".resource" => & [RobotFramework::info], ".rest" => &
    [ReStructuredText::info], ".rest.txt" => & [ReStructuredText::info], ".resx" => &
    [XML::info], ".rex" => & [REXX::info], ".rexx" => & [REXX::info], ".rg" => &
    [Rouge::info], ".rhtml" => & [HTMLpERB::info], ".ring" => & [Ring::info], ".riot" =>
    & [Riot::info], ".rkt" => & [Racket::info], ".rktd" => & [Racket::info], ".rktl" => &
    [Racket::info], ".rl" => & [Ragel::info], ".rmd" => & [RMarkdown::info], ".rmiss" =>
    & [GLSL::info], ".rnh" => & [RUNOFF::info], ".rno" => & [RUNOFF::info, Roff::info],
    ".rnw" => & [Sweave::info], ".robot" => & [RobotFramework::info], ".roc" => &
    [Roc::info], ".rockspec" => & [Lua::info], ".roff" => & [Roff::info], ".ron" => &
    [RON::info], ".ronn" => & [Markdown::info], ".rpgle" => & [RPGLE::info], ".rpy" => &
    [Python::info, RenPy::info], ".rq" => & [SPARQL::info], ".rs" => &
    [RenderScript::info, Rust::info, XML::info], ".rs.in" => & [Rust::info], ".rsc" => &
    [Rascal::info, RouterOSScript::info], ".rsh" => & [RenderScript::info], ".rss" => &
    [XML::info], ".rst" => & [ReStructuredText::info], ".rst.txt" => &
    [ReStructuredText::info], ".rsx" => & [R::info], ".rtf" => & [RichTextFormat::info],
    ".ru" => & [Ruby::info], ".ruby" => & [Ruby::info], ".rviz" => & [YAML::info], ".s"
    => & [Assembly::info, Motorola68KAssembly::info, UnixAssembly::info], ".sage" => &
    [Sage::info], ".sagews" => & [Sage::info], ".sail" => & [Sail::info], ".sarif" => &
    [JSON::info], ".sas" => & [SAS::info], ".sass" => & [Sass::info], ".sats" => &
    [ATS::info], ".sbt" => & [Scala::info], ".sc" => & [Scala::info,
    SuperCollider::info], ".scad" => & [OpenSCAD::info], ".scala" => & [Scala::info],
    ".scaml" => & [Scaml::info], ".scd" => & [Markdown::info, SuperCollider::info],
    ".sce" => & [Scilab::info], ".scenic" => & [Scenic::info], ".sch" => & [Eagle::info,
    KiCadSchematic::info, Scheme::info, XML::info], ".sci" => & [Scilab::info], ".scm" =>
    & [Scheme::info, TreeSitterQuery::info], ".sco" => & [CsoundScore::info], ".scpt" =>
    & [AppleScript::info], ".scrbl" => & [Racket::info], ".scss" => & [SCSS::info],
    ".scxml" => & [XML::info], ".sdc" => & [Tcl::info], ".sed" => & [Sed::info], ".self"
    => & [_Self::info], ".service" => & [Desktop::info], ".sexp" => & [CommonLisp::info],
    ".sfd" => & [SplineFontDatabase::info], ".sfproj" => & [XML::info], ".sfv" => &
    [SimpleFileVerification::info], ".sh" => & [Shell::info], ".sh-session" => &
    [ShellSession::info], ".sh.in" => & [Shell::info], ".sha1" => & [Checksums::info],
    ".sha2" => & [Checksums::info], ".sha224" => & [Checksums::info], ".sha256" => &
    [Checksums::info], ".sha256sum" => & [Checksums::info], ".sha3" => &
    [Checksums::info], ".sha384" => & [Checksums::info], ".sha512" => &
    [Checksums::info], ".shader" => & [GLSL::info, ShaderLab::info], ".shen" => &
    [Shen::info], ".shproj" => & [XML::info], ".sieve" => & [Sieve::info], ".sig" => &
    [StandardML::info], ".sj" => & [ObjectiveJ::info], ".sjs" => & [JavaScript::info],
    ".sl" => & [Slash::info], ".slang" => & [Slang::info], ".sld" => & [Scheme::info],
    ".slim" => & [Slim::info], ".slint" => & [Slint::info], ".sln" => &
    [MicrosoftVisualStudioSolution::info], ".slnx" => & [XML::info], ".sls" => &
    [SaltStack::info, Scheme::info], ".sma" => & [Pawn::info], ".smali" => &
    [Smali::info], ".smithy" => & [Smithy::info], ".smk" => & [Snakemake::info], ".sml"
    => & [StandardML::info], ".smt" => & [SMT::info], ".smt2" => & [SMT::info],
    ".snakefile" => & [Snakemake::info], ".snap" => & [JestSnapshot::info], ".snip" => &
    [VimSnippet::info], ".snippet" => & [VimSnippet::info], ".snippets" => &
    [VimSnippet::info], ".sol" => & [GerberImage::info, Solidity::info], ".soy" => &
    [ClosureTemplates::info], ".sp" => & [SourcePawn::info], ".sparql" => &
    [SPARQL::info], ".spc" => & [PLSQL::info], ".spec" => & [Python::info, RPMSpec::info,
    Ruby::info], ".spin" => & [PropellerSpin::info], ".sps" => & [Scheme::info], ".sqf"
    => & [SQF::info], ".sql" => & [PLSQL::info, PLpgSQL::info, SQL::info, SQLPL::info,
    TSQL::info], ".sqlrpgle" => & [RPGLE::info], ".sra" => & [PowerBuilder::info],
    ".srdf" => & [XML::info], ".srt" => & [SRecodeTemplate::info, SubRipText::info],
    ".sru" => & [PowerBuilder::info], ".srv" => & [ROSInterface::info], ".srw" => &
    [PowerBuilder::info], ".ss" => & [Scheme::info], ".ssjs" => & [JavaScript::info],
    ".sss" => & [SugarSS::info], ".st" => & [Smalltalk::info, StringTemplate::info],
    ".stTheme" => & [XMLPropertyList::info], ".stan" => & [Stan::info], ".star" => &
    [STAR::info, Starlark::info], ".sthlp" => & [Stata::info], ".stl" => & [STL::info],
    ".ston" => & [STON::info], ".story" => & [Gherkin::info], ".storyboard" => &
    [XML::info], ".sty" => & [TeX::info], ".styl" => & [Stylus::info], ".sublime-build"
    => & [JSONWithComments::info], ".sublime-color-scheme" => & [JSONWithComments::info],
    ".sublime-commands" => & [JSONWithComments::info], ".sublime-completions" => &
    [JSONWithComments::info], ".sublime-keymap" => & [JSONWithComments::info],
    ".sublime-macro" => & [JSONWithComments::info], ".sublime-menu" => &
    [JSONWithComments::info], ".sublime-mousemap" => & [JSONWithComments::info],
    ".sublime-project" => & [JSONWithComments::info], ".sublime-settings" => &
    [JSONWithComments::info], ".sublime-snippet" => & [XML::info], ".sublime-syntax" => &
    [YAML::info], ".sublime-theme" => & [JSONWithComments::info], ".sublime-workspace" =>
    & [JSONWithComments::info], ".sublime_metrics" => & [JSONWithComments::info],
    ".sublime_session" => & [JSONWithComments::info], ".sv" => & [SystemVerilog::info],
    ".svelte" => & [Svelte::info], ".svg" => & [SVG::info], ".svh" => &
    [SystemVerilog::info], ".svx" => & [SurvexData::info, Mdsvex::info], ".sw" => &
    [Sway::info, XML::info], ".swg" => & [SWIG::info], ".swift" => & [Swift::info],
    ".swig" => & [SWIG::info], ".syntax" => & [YAML::info], ".t" => & [Perl::info,
    Raku::info, Terra::info, Turing::info], ".tab" => & [SQL::info], ".tac" => &
    [Python::info], ".tact" => & [JSON::info, Tact::info], ".tag" => &
    [JavaServerPages::info], ".talon" => & [Talon::info], ".targets" => & [XML::info],
    ".tcc" => & [Cpp::info], ".tcl" => & [Tcl::info], ".tcl.in" => & [Tcl::info], ".tcsh"
    => & [Tcsh::info], ".te" => & [SELinuxPolicy::info], ".tea" => & [Tea::info],
    ".templ" => & [Templ::info], ".tesc" => & [GLSL::info], ".tese" => & [GLSL::info],
    ".tex" => & [TeX::info], ".texi" => & [Texinfo::info], ".texinfo" => &
    [Texinfo::info], ".textile" => & [Textile::info], ".textproto" => &
    [ProtocolBufferTextFormat::info], ".tf" => & [HCL::info], ".tfstate" => &
    [JSON::info], ".tfstate.backup" => & [JSON::info], ".tftpl" => &
    [TerraformTemplate::info], ".tfvars" => & [HCL::info], ".thor" => & [Ruby::info],
    ".thrift" => & [Thrift::info], ".thy" => & [Isabelle::info], ".tl" => & [Teal::info,
    TypeLanguage::info], ".tla" => & [TLA::info], ".tlv" => & [TLVerilog::info], ".tm" =>
    & [Tcl::info], ".tmCommand" => & [XMLPropertyList::info], ".tmLanguage" => &
    [XMLPropertyList::info], ".tmPreferences" => & [XMLPropertyList::info], ".tmSnippet"
    => & [XMLPropertyList::info], ".tmTheme" => & [XMLPropertyList::info], ".tmac" => &
    [Roff::info], ".tml" => & [XML::info], ".tmpl" => & [GoTemplate::info], ".tmux" => &
    [Shell::info], ".toc" => & [TeX::info, WorldOfWarcraftAddonData::info], ".toit" => &
    [Toit::info], ".toml" => & [TOML::info], ".toml.example" => & [TOML::info], ".tool"
    => & [Shell::info], ".topojson" => & [JSON::info], ".tpb" => & [PLSQL::info], ".tpl"
    => & [GoTemplate::info, Smarty::info], ".tpp" => & [Cpp::info], ".tps" => &
    [PLSQL::info], ".tres" => & [GodotResource::info], ".trg" => & [PLSQL::info],
    ".trigger" => & [Apex::info, Shell::info], ".ts" => & [TypeScript::info, XML::info],
    ".tscn" => & [GodotResource::info], ".tsconfig.json" => & [JSONWithComments::info],
    ".tsp" => & [TSPLIBData::info, TypeSpec::info], ".tst" => & [GAP::info,
    Scilab::info], ".tsv" => & [TSV::info], ".tsx" => & [TSX::info, XML::info], ".ttl" =>
    & [Turtle::info], ".tu" => & [Turing::info], ".twig" => & [Twig::info], ".txi" => &
    [Texinfo::info], ".txl" => & [TXL::info], ".txt" => & [AdblockFilterList::info,
    Text::info, VimHelpFile::info], ".txx" => & [Cpp::info], ".typ" => & [Typst::info,
    XML::info], ".uc" => & [UnrealScript::info], ".udf" => & [SQL::info], ".udo" => &
    [Csound::info], ".ui" => & [XML::info], ".unity" => & [Unity3DAsset::info], ".uno" =>
    & [Uno::info], ".upc" => & [UnifiedParallelC::info], ".uplc" => &
    [UntypedPlutusCore::info], ".ur" => & [UrWeb::info], ".urdf" => & [XML::info], ".url"
    => & [INI::info], ".urs" => & [UrWeb::info], ".ux" => & [XML::info], ".v" => &
    [RocqProver::info, V::info, Verilog::info], ".vala" => & [Vala::info], ".vapi" => &
    [Vala::info], ".vark" => & [Gosu::info], ".vb" => & [VisualBasicNET::info], ".vba" =>
    & [VBA::info, VimScript::info], ".vbhtml" => & [VisualBasicNET::info], ".vbproj" => &
    [XML::info], ".vbs" => & [VBScript::info], ".vcf" => & [TSV::info, VCard::info],
    ".vcl" => & [VCL::info], ".vcxproj" => & [XML::info], ".vdf" => &
    [ValveDataFormat::info], ".veo" => & [Verilog::info], ".vert" => & [GLSL::info],
    ".vh" => & [SystemVerilog::info], ".vhd" => & [VHDL::info], ".vhdl" => &
    [VHDL::info], ".vhf" => & [VHDL::info], ".vhi" => & [VHDL::info], ".vho" => &
    [VHDL::info], ".vhost" => & [ApacheConf::info, Nginx::info], ".vhs" => &
    [VHDL::info], ".vht" => & [VHDL::info], ".vhw" => & [VHDL::info], ".vim" => &
    [VimScript::info], ".vimrc" => & [VimScript::info], ".viw" => & [SQL::info], ".vmb"
    => & [VimScript::info], ".volt" => & [Volt::info], ".vrx" => & [GLSL::info], ".vs" =>
    & [GLSL::info], ".vsh" => & [GLSL::info], ".vshader" => & [GLSL::info],
    ".vsixmanifest" => & [XML::info], ".vssettings" => & [XML::info], ".vstemplate" => &
    [XML::info], ".vtl" => & [VelocityTemplateLanguage::info], ".vto" => & [Vento::info],
    ".vtt" => & [WebVTT::info], ".vue" => & [Vue::info], ".vw" => & [PLSQL::info],
    ".vxml" => & [XML::info], ".vy" => & [Vyper::info], ".w" => & [CWeb::info,
    OpenEdgeABL::info], ".wast" => & [WebAssembly::info], ".wat" => &
    [WebAssembly::info], ".watchr" => & [Ruby::info], ".wdl" => & [WDL::info], ".webapp"
    => & [JSON::info], ".webidl" => & [WebIDL::info], ".webmanifest" => & [JSON::info],
    ".weechatlog" => & [IRCLog::info], ".wgsl" => & [WGSL::info], ".whiley" => &
    [Whiley::info], ".wiki" => & [Wikitext::info], ".wikitext" => & [Wikitext::info],
    ".wisp" => & [Wisp::info], ".wit" => & [WebAssemblyInterfaceType::info], ".wixproj"
    => & [XML::info], ".wl" => & [WolframLanguage::info], ".wlk" => & [Wollok::info],
    ".wls" => & [WolframLanguage::info], ".wlt" => & [WolframLanguage::info], ".wlua" =>
    & [Lua::info], ".workbook" => & [Markdown::info], ".workflow" => & [HCL::info,
    XML::info], ".wren" => & [Wren::info], ".ws" => & [WitcherScript::info], ".wsdl" => &
    [XML::info], ".wsf" => & [XML::info], ".wsgi" => & [Python::info], ".wxi" => &
    [XML::info], ".wxl" => & [XML::info], ".wxs" => & [XML::info], ".x" => &
    [DirectX3DFile::info, LinkerScript::info, Logos::info, RPC::info], ".x10" => &
    [X10::info], ".x3d" => & [XML::info], ".x68" => & [Motorola68KAssembly::info],
    ".xacro" => & [XML::info], ".xaml" => & [XML::info], ".xbm" => & [XBitMap::info],
    ".xc" => & [XC::info], ".xdc" => & [Tcl::info], ".xht" => & [HTML::info], ".xhtml" =>
    & [HTML::info], ".xi" => & [Logos::info], ".xib" => & [XML::info], ".xlf" => &
    [XML::info], ".xliff" => & [XML::info], ".xm" => & [Logos::info], ".xmi" => &
    [XML::info], ".xml" => & [XML::info], ".xml.dist" => & [XML::info], ".xmp" => &
    [XML::info], ".xojo_code" => & [Xojo::info], ".xojo_menu" => & [Xojo::info],
    ".xojo_report" => & [Xojo::info], ".xojo_script" => & [Xojo::info], ".xojo_toolbar"
    => & [Xojo::info], ".xojo_window" => & [Xojo::info], ".xpl" => & [XProc::info],
    ".xpm" => & [XPixMap::info], ".xproc" => & [XProc::info], ".xproj" => & [XML::info],
    ".xpy" => & [Python::info], ".xq" => & [XQuery::info], ".xql" => & [XQuery::info],
    ".xqm" => & [XQuery::info], ".xquery" => & [XQuery::info], ".xqy" => &
    [XQuery::info], ".xrl" => & [Erlang::info], ".xs" => & [XS::info], ".xsd" => &
    [XML::info], ".xsh" => & [Xonsh::info], ".xsjs" => & [JavaScript::info], ".xsjslib"
    => & [JavaScript::info], ".xsl" => & [XSLT::info], ".xslt" => & [XSLT::info],
    ".xsp-config" => & [XPages::info], ".xsp.metadata" => & [XPages::info], ".xspec" => &
    [XML::info], ".xtend" => & [Xtend::info], ".xul" => & [XML::info], ".xzap" => &
    [ZAP::info], ".y" => & [Yacc::info], ".yacc" => & [Yacc::info], ".yaml" => &
    [MiniYAML::info, OASv2Yaml::info, OASv3Yaml::info, YAML::info], ".yaml-tmlanguage" =>
    & [YAML::info], ".yaml.sed" => & [YAML::info], ".yang" => & [YANG::info], ".yap" => &
    [Prolog::info], ".yar" => & [YARA::info], ".yara" => & [YARA::info], ".yasnippet" =>
    & [YASnippet::info], ".yml" => & [MiniYAML::info, OASv2Yaml::info, OASv3Yaml::info,
    YAML::info], ".yml.mysql" => & [YAML::info], ".yrl" => & [Erlang::info], ".yul" => &
    [Yul::info], ".yy" => & [JSON::info, Yacc::info], ".yyp" => & [JSON::info], ".z3" =>
    & [SMT::info], ".zap" => & [ZAP::info], ".zcml" => & [XML::info], ".zeek" => &
    [Zeek::info], ".zep" => & [Zephir::info], ".zig" => & [Zig::info], ".zig.zon" => &
    [Zig::info], ".zil" => & [ZIL::info], ".zimpl" => & [Zimpl::info], ".zmodel" => &
    [Zmodel::info], ".zmpl" => & [Zimpl::info], ".zone" => & [DNSZone::info], ".zpl" => &
    [Zimpl::info], ".zs" => & [ZenScript::info], ".zsh" => & [Shell::info], ".zsh-theme"
    => & [Shell::info]
};
