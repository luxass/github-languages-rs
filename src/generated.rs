use std::collections::HashMap;
use once_cell::sync::Lazy;
#[derive(Debug, Clone, PartialEq)]
pub struct Language {
    pub name: &'static str,
    pub r#type: &'static str,
    pub color: &'static str,
    pub extensions: &'static [&'static str],
    pub aliases: &'static [&'static str],
    pub tm_scope: &'static str,
    pub ace_mode: &'static str,
    pub language_id: u64,
    pub codemirror_mode: Option<&'static str>,
    pub codemirror_mime_type: Option<&'static str>,
    pub wrap: Option<bool>,
    pub filenames: &'static [&'static str],
    pub group: Option<&'static str>,
    pub interpreters: &'static [&'static str],
    pub fs_name: Option<&'static str>,
    pub searchable: Option<bool>,
}
pub struct _1CEnterprise;
pub struct _2DimensionalArray;
pub struct _4D;
pub struct ABAP;
pub struct ABAPCDS;
pub struct ABNF;
pub struct AGSScript;
pub struct AIDL;
pub struct AL;
pub struct AMPL;
pub struct ANTLR;
pub struct APIBlueprint;
pub struct APL;
pub struct ASL;
pub struct ASN1;
pub struct ASPNET;
pub struct ATS;
pub struct ActionScript;
pub struct Ada;
pub struct AdblockFilterList;
pub struct AdobeFontMetrics;
pub struct Agda;
pub struct Alloy;
pub struct AlpineAbuild;
pub struct AltiumDesigner;
pub struct AngelScript;
pub struct AnswerSetProgramming;
pub struct AntBuildSystem;
pub struct Antlers;
pub struct ApacheConf;
pub struct Apex;
pub struct ApolloGuidanceComputer;
pub struct AppleScript;
pub struct Arc;
pub struct AsciiDoc;
pub struct AspectJ;
pub struct Assembly;
pub struct Astro;
pub struct Asymptote;
pub struct Augeas;
pub struct AutoHotkey;
pub struct AutoIt;
pub struct AvroIDL;
pub struct Awk;
pub struct B4X;
pub struct BASIC;
pub struct BQN;
pub struct Ballerina;
pub struct Batchfile;
pub struct Beef;
pub struct Befunge;
pub struct Berry;
pub struct BibTeX;
pub struct Bicep;
pub struct Bikeshed;
pub struct Bison;
pub struct BitBake;
pub struct Blade;
pub struct BlitzBasic;
pub struct BlitzMax;
pub struct Bluespec;
pub struct BluespecBH;
pub struct Boo;
pub struct Boogie;
pub struct Brainfuck;
pub struct BrighterScript;
pub struct Brightscript;
pub struct Browserslist;
pub struct C;
pub struct Csharp;
pub struct Cpp;
pub struct CObjDump;
pub struct C2hsHaskell;
pub struct CAPCDS;
pub struct CIL;
pub struct CLIPS;
pub struct CMake;
pub struct COBOL;
pub struct CODEOWNERS;
pub struct COLLADA;
pub struct CSON;
pub struct CSS;
pub struct CSV;
pub struct CUE;
pub struct CWeb;
pub struct CabalConfig;
pub struct Caddyfile;
pub struct Cadence;
pub struct Cairo;
pub struct CairoZero;
pub struct CameLIGO;
pub struct CapnProto;
pub struct Carbon;
pub struct CartoCSS;
pub struct Ceylon;
pub struct Chapel;
pub struct Charity;
pub struct Checksums;
pub struct ChucK;
pub struct Circom;
pub struct Cirru;
pub struct Clarion;
pub struct Clarity;
pub struct ClassicASP;
pub struct Clean;
pub struct Click;
pub struct Clojure;
pub struct ClosureTemplates;
pub struct CloudFirestoreSecurityRules;
pub struct CoNLLU;
pub struct CodeQL;
pub struct CoffeeScript;
pub struct ColdFusion;
pub struct ColdFusionCFC;
pub struct CommonLisp;
pub struct CommonWorkflowLanguage;
pub struct ComponentPascal;
pub struct Cool;
pub struct Coq;
pub struct CppObjDump;
pub struct Creole;
pub struct Crystal;
pub struct Csound;
pub struct CsoundDocument;
pub struct CsoundScore;
pub struct Cuda;
pub struct CueSheet;
pub struct Curry;
pub struct Cycript;
pub struct Cylc;
pub struct Cypher;
pub struct Cython;
pub struct D;
pub struct DObjDump;
pub struct D2;
pub struct DIGITALCommandLanguage;
pub struct DM;
pub struct DNSZone;
pub struct DTrace;
pub struct Dafny;
pub struct DarcsPatch;
pub struct Dart;
pub struct DataWeave;
pub struct DebianPackageControlFile;
pub struct DenizenScript;
pub struct Dhall;
pub struct Diff;
pub struct DirectX3DFile;
pub struct Dockerfile;
pub struct Dogescript;
pub struct Dotenv;
pub struct Dune;
pub struct Dylan;
pub struct E;
pub struct EMail;
pub struct EBNF;
pub struct ECL;
pub struct ECLiPSe;
pub struct EJS;
pub struct EQ;
pub struct Eagle;
pub struct Earthly;
pub struct Easybuild;
pub struct EcereProjects;
pub struct Ecmarkup;
pub struct Edge;
pub struct EdgeQL;
pub struct EditorConfig;
pub struct EdjeDataCollection;
pub struct Eiffel;
pub struct Elixir;
pub struct Elm;
pub struct Elvish;
pub struct ElvishTranscript;
pub struct EmacsLisp;
pub struct EmberScript;
pub struct Erlang;
pub struct Euphoria;
pub struct Fsharp;
pub struct Fstar;
pub struct FIGletFont;
pub struct FIRRTL;
pub struct FLUX;
pub struct Factor;
pub struct Fancy;
pub struct Fantom;
pub struct Faust;
pub struct Fennel;
pub struct FilebenchWML;
pub struct Filterscript;
pub struct Fluent;
pub struct Formatted;
pub struct Forth;
pub struct Fortran;
pub struct FortranFreeForm;
pub struct FreeBASIC;
pub struct FreeMarker;
pub struct Frege;
pub struct Futhark;
pub struct GCode;
pub struct GAML;
pub struct GAMS;
pub struct GAP;
pub struct GCCMachineDescription;
pub struct GDB;
pub struct GDScript;
pub struct GEDCOM;
pub struct GLSL;
pub struct GN;
pub struct GSC;
pub struct GameMakerLanguage;
pub struct Gemfilelock;
pub struct Gemini;
pub struct Genero4gl;
pub struct GeneroPer;
pub struct Genie;
pub struct Genshi;
pub struct GentooEbuild;
pub struct GentooEclass;
pub struct GerberImage;
pub struct GettextCatalog;
pub struct Gherkin;
pub struct GitAttributes;
pub struct GitConfig;
pub struct GitRevisionList;
pub struct Gleam;
pub struct GlimmerJS;
pub struct GlimmerTS;
pub struct Glyph;
pub struct GlyphBitmapDistributionFormat;
pub struct Gnuplot;
pub struct Go;
pub struct GoChecksums;
pub struct GoModule;
pub struct GoWorkspace;
pub struct GodotResource;
pub struct Golo;
pub struct Gosu;
pub struct Grace;
pub struct Gradle;
pub struct GradleKotlinDSL;
pub struct GrammaticalFramework;
pub struct GraphModelingLanguage;
pub struct GraphQL;
pub struct GraphvizDOT;
pub struct Groovy;
pub struct GroovyServerPages;
pub struct HAProxy;
pub struct HCL;
pub struct HLSL;
pub struct HOCON;
pub struct HTML;
pub struct HTMLpECR;
pub struct HTMLpEEX;
pub struct HTMLpERB;
pub struct HTMLpPHP;
pub struct HTMLpRazor;
pub struct HTTP;
pub struct HXML;
pub struct Hack;
pub struct Haml;
pub struct Handlebars;
pub struct Harbour;
pub struct Hare;
pub struct Haskell;
pub struct Haxe;
pub struct HiveQL;
pub struct HolyC;
pub struct HostsFile;
pub struct Hy;
pub struct HyPhy;
pub struct IDL;
pub struct IGORPro;
pub struct INI;
pub struct IRCLog;
pub struct Idris;
pub struct IgnoreList;
pub struct ImageJMacro;
pub struct Imba;
pub struct Inform7;
pub struct Ink;
pub struct InnoSetup;
pub struct Io;
pub struct Ioke;
pub struct Isabelle;
pub struct IsabelleROOT;
pub struct J;
pub struct JARManifest;
pub struct JCL;
pub struct JFlex;
pub struct JSON;
pub struct JSONWithComments;
pub struct JSON5;
pub struct JSONLD;
pub struct JSONiq;
pub struct Jai;
pub struct Janet;
pub struct Jasmin;
pub struct Java;
pub struct JavaProperties;
pub struct JavaServerPages;
pub struct JavaTemplateEngine;
pub struct JavaScript;
pub struct JavaScriptpERB;
pub struct JestSnapshot;
pub struct JetBrainsMPS;
pub struct Jinja;
pub struct Jison;
pub struct JisonLex;
pub struct Jolie;
pub struct Jsonnet;
pub struct Julia;
pub struct JuliaREPL;
pub struct JupyterNotebook;
pub struct Just;
pub struct KDL;
pub struct KRL;
pub struct KaitaiStruct;
pub struct KakouneScript;
pub struct KerboScript;
pub struct KiCadLayout;
pub struct KiCadLegacyLayout;
pub struct KiCadSchematic;
pub struct Kickstart;
pub struct Kit;
pub struct Kotlin;
pub struct Kusto;
pub struct LFE;
pub struct LLVM;
pub struct LOLCODE;
pub struct LSL;
pub struct LTspiceSymbol;
pub struct LabVIEW;
pub struct Lark;
pub struct Lasso;
pub struct Latte;
pub struct Lean;
pub struct Lean4;
pub struct Less;
pub struct Lex;
pub struct LigoLANG;
pub struct LilyPond;
pub struct Limbo;
pub struct LinearProgramming;
pub struct LinkerScript;
pub struct LinuxKernelModule;
pub struct Liquid;
pub struct LiterateAgda;
pub struct LiterateCoffeeScript;
pub struct LiterateHaskell;
pub struct LiveCodeScript;
pub struct LiveScript;
pub struct Logos;
pub struct Logtalk;
pub struct LookML;
pub struct LoomScript;
pub struct Lua;
pub struct Luau;
pub struct M;
pub struct M4;
pub struct M4Sugar;
pub struct MATLAB;
pub struct MAXScript;
pub struct MDX;
pub struct MLIR;
pub struct MQL4;
pub struct MQL5;
pub struct MTML;
pub struct MUF;
pub struct Macaulay2;
pub struct Makefile;
pub struct Mako;
pub struct Markdown;
pub struct Marko;
pub struct Mask;
pub struct Mathematica;
pub struct MavenPOM;
pub struct Max;
pub struct Mercury;
pub struct Mermaid;
pub struct Meson;
pub struct Metal;
pub struct MicrosoftDeveloperStudioProject;
pub struct MicrosoftVisualStudioSolution;
pub struct MiniD;
pub struct MiniYAML;
pub struct MiniZinc;
pub struct MiniZincData;
pub struct Mint;
pub struct Mirah;
pub struct Modelica;
pub struct Modula2;
pub struct Modula3;
pub struct ModuleManagementSystem;
pub struct Mojo;
pub struct Monkey;
pub struct MonkeyC;
pub struct Moocode;
pub struct MoonBit;
pub struct MoonScript;
pub struct Motoko;
pub struct Motorola68KAssembly;
pub struct Move;
pub struct Muse;
pub struct Mustache;
pub struct Myghty;
pub struct NASL;
pub struct NCL;
pub struct NEON;
pub struct NL;
pub struct NMODL;
pub struct NPMConfig;
pub struct NSIS;
pub struct NWScript;
pub struct Nasal;
pub struct Nearley;
pub struct Nemerle;
pub struct NetLinx;
pub struct NetLinxpERB;
pub struct NetLogo;
pub struct NewLisp;
pub struct Nextflow;
pub struct Nginx;
pub struct Nim;
pub struct Ninja;
pub struct Nit;
pub struct Nix;
pub struct Noir;
pub struct Nu;
pub struct NumPy;
pub struct Nunjucks;
pub struct Nushell;
pub struct OASv2Json;
pub struct OASv2Yaml;
pub struct OASv3Json;
pub struct OASv3Yaml;
pub struct OCaml;
pub struct OMNeTppMSG;
pub struct OMNeTppNED;
pub struct Oberon;
pub struct ObjDump;
pub struct ObjectDataInstanceNotation;
pub struct ObjectScript;
pub struct ObjectiveC;
pub struct ObjectiveCpp;
pub struct ObjectiveJ;
pub struct Odin;
pub struct Omgrofl;
pub struct Opa;
pub struct Opal;
pub struct OpenPolicyAgent;
pub struct OpenAPISpecificationV2;
pub struct OpenAPISpecificationV3;
pub struct OpenCL;
pub struct OpenEdgeABL;
pub struct OpenQASM;
pub struct OpenRCRunscript;
pub struct OpenSCAD;
pub struct OpenStepPropertyList;
pub struct OpenTypeFeatureFile;
pub struct OptionList;
pub struct Org;
pub struct OverpassQL;
pub struct Ox;
pub struct Oxygene;
pub struct Oz;
pub struct P4;
pub struct PDDL;
pub struct PEGjs;
pub struct PHP;
pub struct PLSQL;
pub struct PLpgSQL;
pub struct POVRaySDL;
pub struct Pact;
pub struct Pan;
pub struct Papyrus;
pub struct Parrot;
pub struct ParrotAssembly;
pub struct ParrotInternalRepresentation;
pub struct Pascal;
pub struct Pawn;
pub struct Pep8;
pub struct Perl;
pub struct Pic;
pub struct Pickle;
pub struct PicoLisp;
pub struct PigLatin;
pub struct Pike;
pub struct PipRequirements;
pub struct Pkl;
pub struct PlantUML;
pub struct Pod;
pub struct Pod6;
pub struct PogoScript;
pub struct Polar;
pub struct Pony;
pub struct Portugol;
pub struct PostCSS;
pub struct PostScript;
pub struct PowerBuilder;
pub struct PowerShell;
pub struct Praat;
pub struct Prisma;
pub struct Processing;
pub struct Procfile;
pub struct Proguard;
pub struct Prolog;
pub struct Promela;
pub struct PropellerSpin;
pub struct ProtocolBuffer;
pub struct ProtocolBufferTextFormat;
pub struct PublicKey;
pub struct Pug;
pub struct Puppet;
pub struct PureData;
pub struct PureBasic;
pub struct PureScript;
pub struct Pyret;
pub struct Python;
pub struct PythonConsole;
pub struct PythonTraceback;
pub struct Qsharp;
pub struct QML;
pub struct QMake;
pub struct QtScript;
pub struct Quake;
pub struct QuickBASIC;
pub struct R;
pub struct RAML;
pub struct RBS;
pub struct RDoc;
pub struct REALbasic;
pub struct REXX;
pub struct RMarkdown;
pub struct RON;
pub struct RPC;
pub struct RPGLE;
pub struct RPMSpec;
pub struct RUNOFF;
pub struct Racket;
pub struct Ragel;
pub struct Raku;
pub struct Rascal;
pub struct RawTokenData;
pub struct ReScript;
pub struct ReadlineConfig;
pub struct Reason;
pub struct ReasonLIGO;
pub struct Rebol;
pub struct RecordJar;
pub struct Red;
pub struct Redcode;
pub struct RedirectRules;
pub struct RegularExpression;
pub struct RenPy;
pub struct RenderScript;
pub struct Rez;
pub struct RichTextFormat;
pub struct Ring;
pub struct Riot;
pub struct RobotFramework;
pub struct Roc;
pub struct Roff;
pub struct RoffManpage;
pub struct Rouge;
pub struct RouterOSScript;
pub struct Ruby;
pub struct Rust;
pub struct SAS;
pub struct SCSS;
pub struct SELinuxPolicy;
pub struct SMT;
pub struct SPARQL;
pub struct SQF;
pub struct SQL;
pub struct SQLPL;
pub struct SRecodeTemplate;
pub struct SSHConfig;
pub struct STAR;
pub struct STL;
pub struct STON;
pub struct SVG;
pub struct SWIG;
pub struct Sage;
pub struct SaltStack;
pub struct Sass;
pub struct Scala;
pub struct Scaml;
pub struct Scenic;
pub struct Scheme;
pub struct Scilab;
pub struct _Self;
pub struct ShaderLab;
pub struct Shell;
pub struct ShellCheckConfig;
pub struct ShellSession;
pub struct Shen;
pub struct Sieve;
pub struct SimpleFileVerification;
pub struct Singularity;
pub struct Slash;
pub struct Slice;
pub struct Slim;
pub struct Slint;
pub struct SmPL;
pub struct Smali;
pub struct Smalltalk;
pub struct Smarty;
pub struct Smithy;
pub struct Snakemake;
pub struct Solidity;
pub struct Soong;
pub struct SourcePawn;
pub struct SplineFontDatabase;
pub struct Squirrel;
pub struct Stan;
pub struct StandardML;
pub struct Starlark;
pub struct Stata;
pub struct StringTemplate;
pub struct Stylus;
pub struct SubRipText;
pub struct SugarSS;
pub struct SuperCollider;
pub struct SurvexData;
pub struct Svelte;
pub struct Sway;
pub struct Sweave;
pub struct Swift;
pub struct SystemVerilog;
pub struct TIProgram;
pub struct TLVerilog;
pub struct TLA;
pub struct TOML;
pub struct TSPLIBData;
pub struct TSQL;
pub struct TSV;
pub struct TSX;
pub struct TXL;
pub struct Tact;
pub struct Talon;
pub struct Tcl;
pub struct Tcsh;
pub struct TeX;
pub struct Tea;
pub struct Terra;
pub struct TerraformTemplate;
pub struct Texinfo;
pub struct Text;
pub struct TextGrid;
pub struct TextMateProperties;
pub struct Textile;
pub struct Thrift;
pub struct Toit;
pub struct Turing;
pub struct Turtle;
pub struct Twig;
pub struct TypeLanguage;
pub struct TypeScript;
pub struct TypeSpec;
pub struct Typst;
pub struct UnifiedParallelC;
pub struct Unity3DAsset;
pub struct UnixAssembly;
pub struct Uno;
pub struct UnrealScript;
pub struct UrWeb;
pub struct V;
pub struct VBA;
pub struct VBScript;
pub struct VCL;
pub struct VHDL;
pub struct Vala;
pub struct ValveDataFormat;
pub struct VelocityTemplateLanguage;
pub struct Verilog;
pub struct VimHelpFile;
pub struct VimScript;
pub struct VimSnippet;
pub struct VisualBasicNET;
pub struct VisualBasic60;
pub struct Volt;
pub struct Vue;
pub struct Vyper;
pub struct WDL;
pub struct WGSL;
pub struct WavefrontMaterial;
pub struct WavefrontObject;
pub struct WebOntologyLanguage;
pub struct WebAssembly;
pub struct WebAssemblyInterfaceType;
pub struct WebIDL;
pub struct WebVTT;
pub struct WgetConfig;
pub struct Whiley;
pub struct Wikitext;
pub struct Win32MessageFile;
pub struct WindowsRegistryEntries;
pub struct WitcherScript;
pub struct Wollok;
pub struct WorldOfWarcraftAddonData;
pub struct Wren;
pub struct XBitMap;
pub struct XFontDirectoryIndex;
pub struct XPixMap;
pub struct X10;
pub struct XC;
pub struct XCompose;
pub struct XML;
pub struct XMLPropertyList;
pub struct XPages;
pub struct XProc;
pub struct XQuery;
pub struct XS;
pub struct XSLT;
pub struct Xmake;
pub struct Xojo;
pub struct Xonsh;
pub struct Xtend;
pub struct YAML;
pub struct YANG;
pub struct YARA;
pub struct YASnippet;
pub struct Yacc;
pub struct Yul;
pub struct ZAP;
pub struct ZIL;
pub struct Zeek;
pub struct ZenScript;
pub struct Zephir;
pub struct Zig;
pub struct Zimpl;
pub struct CURLConfig;
pub struct Crontab;
pub struct Desktop;
pub struct Dircolors;
pub struct EC;
pub struct Edn;
pub struct Fish;
pub struct Hoon;
pub struct ICalendar;
pub struct Jq;
pub struct Kvlang;
pub struct MIRCScript;
pub struct Mcfunction;
pub struct Mdsvex;
pub struct Mupad;
pub struct Nanorc;
pub struct NesC;
pub struct Ooc;
pub struct Q;
pub struct ReStructuredText;
pub struct Robotstxt;
pub struct Sed;
pub struct Templ;
pub struct VCard;
pub struct Wisp;
pub struct XBase;
impl _1CEnterprise {
    pub fn info() -> Language {
        Language {
            name: "1C Enterprise",
            r#type: "programming",
            color: "#814CCC",
            extensions: &[".bsl", ".os"],
            tm_scope: "source.bsl",
            ace_mode: "text",
            language_id: 0u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl _2DimensionalArray {
    pub fn info() -> Language {
        Language {
            name: "2-Dimensional Array",
            r#type: "data",
            color: "#38761D",
            extensions: &[".2da"],
            tm_scope: "source.2da",
            ace_mode: "text",
            language_id: 387204628u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl _4D {
    pub fn info() -> Language {
        Language {
            name: "4D",
            r#type: "programming",
            color: "#004289",
            extensions: &[".4dm"],
            tm_scope: "source.4dm",
            ace_mode: "text",
            language_id: 577529595u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ABAP {
    pub fn info() -> Language {
        Language {
            name: "ABAP",
            r#type: "programming",
            color: "#E8274B",
            extensions: &[".abap"],
            tm_scope: "source.abap",
            ace_mode: "abap",
            language_id: 1u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ABAPCDS {
    pub fn info() -> Language {
        Language {
            name: "ABAP CDS",
            r#type: "programming",
            color: "#555e25",
            extensions: &[".asddls"],
            tm_scope: "source.abapcds",
            ace_mode: "text",
            language_id: 452681853u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ABNF {
    pub fn info() -> Language {
        Language {
            name: "ABNF",
            r#type: "data",
            color: "",
            extensions: &[".abnf"],
            tm_scope: "source.abnf",
            ace_mode: "text",
            language_id: 429u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl AGSScript {
    pub fn info() -> Language {
        Language {
            name: "AGS Script",
            r#type: "programming",
            color: "#B9D9FF",
            extensions: &[".asc", ".ash"],
            tm_scope: "source.c++",
            ace_mode: "c_cpp",
            language_id: 2u64,
            aliases: &["ags"],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-c++src"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl AIDL {
    pub fn info() -> Language {
        Language {
            name: "AIDL",
            r#type: "programming",
            color: "#34EB6B",
            extensions: &[".aidl"],
            tm_scope: "source.aidl",
            ace_mode: "text",
            language_id: 451700185u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["aidl"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl AL {
    pub fn info() -> Language {
        Language {
            name: "AL",
            r#type: "programming",
            color: "#3AA2B5",
            extensions: &[".al"],
            tm_scope: "source.al",
            ace_mode: "text",
            language_id: 658971832u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl AMPL {
    pub fn info() -> Language {
        Language {
            name: "AMPL",
            r#type: "programming",
            color: "#E6EFBB",
            extensions: &[".ampl", ".mod"],
            tm_scope: "source.ampl",
            ace_mode: "text",
            language_id: 3u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ANTLR {
    pub fn info() -> Language {
        Language {
            name: "ANTLR",
            r#type: "programming",
            color: "#9DC3FF",
            extensions: &[".g4"],
            tm_scope: "source.antlr",
            ace_mode: "text",
            language_id: 4u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl APIBlueprint {
    pub fn info() -> Language {
        Language {
            name: "API Blueprint",
            r#type: "markup",
            color: "#2ACCA8",
            extensions: &[".apib"],
            tm_scope: "text.html.markdown.source.gfm.apib",
            ace_mode: "markdown",
            language_id: 5u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl APL {
    pub fn info() -> Language {
        Language {
            name: "APL",
            r#type: "programming",
            color: "#5A8164",
            extensions: &[".apl", ".dyalog"],
            tm_scope: "source.apl",
            ace_mode: "text",
            language_id: 6u64,
            aliases: &[],
            codemirror_mode: Some("apl"),
            codemirror_mime_type: Some("text/apl"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["apl", "aplx", "dyalog"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ASL {
    pub fn info() -> Language {
        Language {
            name: "ASL",
            r#type: "programming",
            color: "",
            extensions: &[".asl", ".dsl"],
            tm_scope: "source.asl",
            ace_mode: "text",
            language_id: 124996147u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ASN1 {
    pub fn info() -> Language {
        Language {
            name: "ASN.1",
            r#type: "data",
            color: "",
            extensions: &[".asn", ".asn1"],
            tm_scope: "source.asn",
            ace_mode: "text",
            language_id: 7u64,
            aliases: &[],
            codemirror_mode: Some("asn.1"),
            codemirror_mime_type: Some("text/x-ttcn-asn"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ASPNET {
    pub fn info() -> Language {
        Language {
            name: "ASP.NET",
            r#type: "programming",
            color: "#9400ff",
            extensions: &[".asax", ".ascx", ".ashx", ".asmx", ".aspx", ".axd"],
            tm_scope: "text.html.asp",
            ace_mode: "text",
            language_id: 564186416u64,
            aliases: &["aspx", "aspx-vb"],
            codemirror_mode: Some("htmlembedded"),
            codemirror_mime_type: Some("application/x-aspx"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ATS {
    pub fn info() -> Language {
        Language {
            name: "ATS",
            r#type: "programming",
            color: "#1ac620",
            extensions: &[".dats", ".hats", ".sats"],
            tm_scope: "source.ats",
            ace_mode: "ocaml",
            language_id: 9u64,
            aliases: &["ats2"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ActionScript {
    pub fn info() -> Language {
        Language {
            name: "ActionScript",
            r#type: "programming",
            color: "#882B0F",
            extensions: &[".as"],
            tm_scope: "source.actionscript.3",
            ace_mode: "actionscript",
            language_id: 10u64,
            aliases: &["actionscript 3", "actionscript3", "as3"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Ada {
    pub fn info() -> Language {
        Language {
            name: "Ada",
            r#type: "programming",
            color: "#02f88c",
            extensions: &[".adb", ".ada", ".ads"],
            tm_scope: "source.ada",
            ace_mode: "ada",
            language_id: 11u64,
            aliases: &["ada95", "ada2005"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl AdblockFilterList {
    pub fn info() -> Language {
        Language {
            name: "Adblock Filter List",
            r#type: "data",
            color: "#800000",
            extensions: &[".txt"],
            tm_scope: "text.adblock",
            ace_mode: "text",
            language_id: 884614762u64,
            aliases: &["ad block filters", "ad block", "adb", "adblock"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl AdobeFontMetrics {
    pub fn info() -> Language {
        Language {
            name: "Adobe Font Metrics",
            r#type: "data",
            color: "#fa0f00",
            extensions: &[".afm"],
            tm_scope: "source.afm",
            ace_mode: "text",
            language_id: 147198098u64,
            aliases: &[
                "acfm",
                "adobe composite font metrics",
                "adobe multiple font metrics",
                "amfm",
            ],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Agda {
    pub fn info() -> Language {
        Language {
            name: "Agda",
            r#type: "programming",
            color: "#315665",
            extensions: &[".agda"],
            tm_scope: "source.agda",
            ace_mode: "text",
            language_id: 12u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Alloy {
    pub fn info() -> Language {
        Language {
            name: "Alloy",
            r#type: "programming",
            color: "#64C800",
            extensions: &[".als"],
            tm_scope: "source.alloy",
            ace_mode: "text",
            language_id: 13u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl AlpineAbuild {
    pub fn info() -> Language {
        Language {
            name: "Alpine Abuild",
            r#type: "programming",
            color: "#0D597F",
            extensions: &[],
            tm_scope: "source.shell",
            ace_mode: "sh",
            language_id: 14u64,
            aliases: &["abuild", "apkbuild"],
            codemirror_mode: Some("shell"),
            codemirror_mime_type: Some("text/x-sh"),
            wrap: None,
            filenames: &["APKBUILD"],
            group: Some("Shell"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl AltiumDesigner {
    pub fn info() -> Language {
        Language {
            name: "Altium Designer",
            r#type: "data",
            color: "#A89663",
            extensions: &[".OutJob", ".PcbDoc", ".PrjPCB", ".SchDoc"],
            tm_scope: "source.ini",
            ace_mode: "ini",
            language_id: 187772328u64,
            aliases: &["altium"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl AngelScript {
    pub fn info() -> Language {
        Language {
            name: "AngelScript",
            r#type: "programming",
            color: "#C7D7DC",
            extensions: &[".as", ".angelscript"],
            tm_scope: "source.angelscript",
            ace_mode: "text",
            language_id: 389477596u64,
            aliases: &[],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-c++src"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl AnswerSetProgramming {
    pub fn info() -> Language {
        Language {
            name: "Answer Set Programming",
            r#type: "programming",
            color: "#A9CC29",
            extensions: &[".lp"],
            tm_scope: "source.answersetprogramming",
            ace_mode: "prolog",
            language_id: 433009171u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["clingo"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl AntBuildSystem {
    pub fn info() -> Language {
        Language {
            name: "Ant Build System",
            r#type: "data",
            color: "#A9157E",
            extensions: &[],
            tm_scope: "text.xml.ant",
            ace_mode: "xml",
            language_id: 15u64,
            aliases: &[],
            codemirror_mode: Some("xml"),
            codemirror_mime_type: Some("application/xml"),
            wrap: None,
            filenames: &["ant.xml", "build.xml"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Antlers {
    pub fn info() -> Language {
        Language {
            name: "Antlers",
            r#type: "markup",
            color: "#ff269e",
            extensions: &[".antlers.html", ".antlers.php", ".antlers.xml"],
            tm_scope: "text.html.statamic",
            ace_mode: "text",
            language_id: 1067292663u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ApacheConf {
    pub fn info() -> Language {
        Language {
            name: "ApacheConf",
            r#type: "data",
            color: "#d12127",
            extensions: &[".apacheconf", ".vhost"],
            tm_scope: "source.apacheconf",
            ace_mode: "apache_conf",
            language_id: 16u64,
            aliases: &["aconf", "apache"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[".htaccess", "apache2.conf", "httpd.conf"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Apex {
    pub fn info() -> Language {
        Language {
            name: "Apex",
            r#type: "programming",
            color: "#1797c0",
            extensions: &[".cls", ".apex", ".trigger"],
            tm_scope: "source.apex",
            ace_mode: "java",
            language_id: 17u64,
            aliases: &[],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-java"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ApolloGuidanceComputer {
    pub fn info() -> Language {
        Language {
            name: "Apollo Guidance Computer",
            r#type: "programming",
            color: "#0B3D91",
            extensions: &[".agc"],
            tm_scope: "source.agc",
            ace_mode: "assembly_x86",
            language_id: 18u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: Some("Assembly"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl AppleScript {
    pub fn info() -> Language {
        Language {
            name: "AppleScript",
            r#type: "programming",
            color: "#101F1F",
            extensions: &[".applescript", ".scpt"],
            tm_scope: "source.applescript",
            ace_mode: "applescript",
            language_id: 19u64,
            aliases: &["osascript"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["osascript"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Arc {
    pub fn info() -> Language {
        Language {
            name: "Arc",
            r#type: "programming",
            color: "#aa2afe",
            extensions: &[".arc"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 20u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl AsciiDoc {
    pub fn info() -> Language {
        Language {
            name: "AsciiDoc",
            r#type: "prose",
            color: "#73a0c5",
            extensions: &[".asciidoc", ".adoc", ".asc"],
            tm_scope: "text.html.asciidoc",
            ace_mode: "asciidoc",
            language_id: 22u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: Some(true),
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl AspectJ {
    pub fn info() -> Language {
        Language {
            name: "AspectJ",
            r#type: "programming",
            color: "#a957b0",
            extensions: &[".aj"],
            tm_scope: "source.aspectj",
            ace_mode: "text",
            language_id: 23u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Assembly {
    pub fn info() -> Language {
        Language {
            name: "Assembly",
            r#type: "programming",
            color: "#6E4C13",
            extensions: &[".asm", ".a51", ".i", ".inc", ".nas", ".nasm"],
            tm_scope: "source.assembly",
            ace_mode: "assembly_x86",
            language_id: 24u64,
            aliases: &["asm", "nasm"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Astro {
    pub fn info() -> Language {
        Language {
            name: "Astro",
            r#type: "markup",
            color: "#ff5a03",
            extensions: &[".astro"],
            tm_scope: "source.astro",
            ace_mode: "html",
            language_id: 578209015u64,
            aliases: &[],
            codemirror_mode: Some("jsx"),
            codemirror_mime_type: Some("text/jsx"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Asymptote {
    pub fn info() -> Language {
        Language {
            name: "Asymptote",
            r#type: "programming",
            color: "#ff0000",
            extensions: &[".asy"],
            tm_scope: "source.c++",
            ace_mode: "c_cpp",
            language_id: 591605007u64,
            aliases: &[],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-kotlin"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["asy"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Augeas {
    pub fn info() -> Language {
        Language {
            name: "Augeas",
            r#type: "programming",
            color: "#9CC134",
            extensions: &[".aug"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 25u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl AutoHotkey {
    pub fn info() -> Language {
        Language {
            name: "AutoHotkey",
            r#type: "programming",
            color: "#6594b9",
            extensions: &[".ahk", ".ahkl"],
            tm_scope: "source.ahk",
            ace_mode: "autohotkey",
            language_id: 26u64,
            aliases: &["ahk"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl AutoIt {
    pub fn info() -> Language {
        Language {
            name: "AutoIt",
            r#type: "programming",
            color: "#1C3552",
            extensions: &[".au3"],
            tm_scope: "source.autoit",
            ace_mode: "autohotkey",
            language_id: 27u64,
            aliases: &["au3", "AutoIt3", "AutoItScript"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl AvroIDL {
    pub fn info() -> Language {
        Language {
            name: "Avro IDL",
            r#type: "data",
            color: "#0040FF",
            extensions: &[".avdl"],
            tm_scope: "source.avro",
            ace_mode: "text",
            language_id: 785497837u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Awk {
    pub fn info() -> Language {
        Language {
            name: "Awk",
            r#type: "programming",
            color: "#c30e9b",
            extensions: &[".awk", ".auk", ".gawk", ".mawk", ".nawk"],
            tm_scope: "source.awk",
            ace_mode: "text",
            language_id: 28u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["awk", "gawk", "mawk", "nawk"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl B4X {
    pub fn info() -> Language {
        Language {
            name: "B4X",
            r#type: "programming",
            color: "#00e4ff",
            extensions: &[".bas"],
            tm_scope: "source.vba",
            ace_mode: "text",
            language_id: 96642275u64,
            aliases: &["basic for android"],
            codemirror_mode: Some("vb"),
            codemirror_mime_type: Some("text/x-vb"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl BASIC {
    pub fn info() -> Language {
        Language {
            name: "BASIC",
            r#type: "programming",
            color: "#ff0000",
            extensions: &[".bas"],
            tm_scope: "source.basic",
            ace_mode: "text",
            language_id: 28923963u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl BQN {
    pub fn info() -> Language {
        Language {
            name: "BQN",
            r#type: "programming",
            color: "#2b7067",
            extensions: &[".bqn"],
            tm_scope: "source.bqn",
            ace_mode: "text",
            language_id: 330386870u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Ballerina {
    pub fn info() -> Language {
        Language {
            name: "Ballerina",
            r#type: "programming",
            color: "#FF5000",
            extensions: &[".bal"],
            tm_scope: "source.ballerina",
            ace_mode: "text",
            language_id: 720859680u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Batchfile {
    pub fn info() -> Language {
        Language {
            name: "Batchfile",
            r#type: "programming",
            color: "#C1F12E",
            extensions: &[".bat", ".cmd"],
            tm_scope: "source.batchfile",
            ace_mode: "batchfile",
            language_id: 29u64,
            aliases: &["bat", "batch", "dosbatch", "winbatch"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Beef {
    pub fn info() -> Language {
        Language {
            name: "Beef",
            r#type: "programming",
            color: "#a52f4e",
            extensions: &[".bf"],
            tm_scope: "source.cs",
            ace_mode: "csharp",
            language_id: 545626333u64,
            aliases: &[],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-csharp"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Befunge {
    pub fn info() -> Language {
        Language {
            name: "Befunge",
            r#type: "programming",
            color: "",
            extensions: &[".befunge", ".bf"],
            tm_scope: "source.befunge",
            ace_mode: "text",
            language_id: 30u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Berry {
    pub fn info() -> Language {
        Language {
            name: "Berry",
            r#type: "programming",
            color: "#15A13C",
            extensions: &[".be"],
            tm_scope: "source.berry",
            ace_mode: "text",
            language_id: 121855308u64,
            aliases: &["be"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl BibTeX {
    pub fn info() -> Language {
        Language {
            name: "BibTeX",
            r#type: "markup",
            color: "#778899",
            extensions: &[".bib", ".bibtex"],
            tm_scope: "text.bibtex",
            ace_mode: "tex",
            language_id: 982188347u64,
            aliases: &[],
            codemirror_mode: Some("stex"),
            codemirror_mime_type: Some("text/x-stex"),
            wrap: None,
            filenames: &[],
            group: Some("TeX"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Bicep {
    pub fn info() -> Language {
        Language {
            name: "Bicep",
            r#type: "programming",
            color: "#519aba",
            extensions: &[".bicep", ".bicepparam"],
            tm_scope: "source.bicep",
            ace_mode: "text",
            language_id: 321200902u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Bikeshed {
    pub fn info() -> Language {
        Language {
            name: "Bikeshed",
            r#type: "markup",
            color: "#5562ac",
            extensions: &[".bs"],
            tm_scope: "source.csswg",
            ace_mode: "html",
            language_id: 1055528081u64,
            aliases: &[],
            codemirror_mode: Some("htmlmixed"),
            codemirror_mime_type: Some("text/html"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Bison {
    pub fn info() -> Language {
        Language {
            name: "Bison",
            r#type: "programming",
            color: "#6A463F",
            extensions: &[".bison"],
            tm_scope: "source.yacc",
            ace_mode: "text",
            language_id: 31u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: Some("Yacc"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl BitBake {
    pub fn info() -> Language {
        Language {
            name: "BitBake",
            r#type: "programming",
            color: "#00bce4",
            extensions: &[".bb", ".bbappend", ".bbclass", ".inc"],
            tm_scope: "source.bb",
            ace_mode: "text",
            language_id: 32u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Blade {
    pub fn info() -> Language {
        Language {
            name: "Blade",
            r#type: "markup",
            color: "#f7523f",
            extensions: &[".blade", ".blade.php"],
            tm_scope: "text.html.php.blade",
            ace_mode: "text",
            language_id: 33u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl BlitzBasic {
    pub fn info() -> Language {
        Language {
            name: "BlitzBasic",
            r#type: "programming",
            color: "#00FFAE",
            extensions: &[".bb", ".decls"],
            tm_scope: "source.blitzmax",
            ace_mode: "text",
            language_id: 34u64,
            aliases: &["b3d", "blitz3d", "blitzplus", "bplus"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl BlitzMax {
    pub fn info() -> Language {
        Language {
            name: "BlitzMax",
            r#type: "programming",
            color: "#cd6400",
            extensions: &[".bmx"],
            tm_scope: "source.blitzmax",
            ace_mode: "text",
            language_id: 35u64,
            aliases: &["bmax"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Bluespec {
    pub fn info() -> Language {
        Language {
            name: "Bluespec",
            r#type: "programming",
            color: "#12223c",
            extensions: &[".bsv"],
            tm_scope: "source.bsv",
            ace_mode: "verilog",
            language_id: 36u64,
            aliases: &["bluespec bsv", "bsv"],
            codemirror_mode: Some("verilog"),
            codemirror_mime_type: Some("text/x-systemverilog"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl BluespecBH {
    pub fn info() -> Language {
        Language {
            name: "Bluespec BH",
            r#type: "programming",
            color: "#12223c",
            extensions: &[".bs"],
            tm_scope: "source.bh",
            ace_mode: "haskell",
            language_id: 641580358u64,
            aliases: &["bh", "bluespec classic"],
            codemirror_mode: Some("haskell"),
            codemirror_mime_type: Some("text/x-haskell"),
            wrap: None,
            filenames: &[],
            group: Some("Bluespec"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Boo {
    pub fn info() -> Language {
        Language {
            name: "Boo",
            r#type: "programming",
            color: "#d4bec1",
            extensions: &[".boo"],
            tm_scope: "source.boo",
            ace_mode: "text",
            language_id: 37u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Boogie {
    pub fn info() -> Language {
        Language {
            name: "Boogie",
            r#type: "programming",
            color: "#c80fa0",
            extensions: &[".bpl"],
            tm_scope: "source.boogie",
            ace_mode: "text",
            language_id: 955017407u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["boogie"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Brainfuck {
    pub fn info() -> Language {
        Language {
            name: "Brainfuck",
            r#type: "programming",
            color: "#2F2530",
            extensions: &[".b", ".bf"],
            tm_scope: "source.bf",
            ace_mode: "text",
            language_id: 38u64,
            aliases: &[],
            codemirror_mode: Some("brainfuck"),
            codemirror_mime_type: Some("text/x-brainfuck"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl BrighterScript {
    pub fn info() -> Language {
        Language {
            name: "BrighterScript",
            r#type: "programming",
            color: "#66AABB",
            extensions: &[".bs"],
            tm_scope: "source.brs",
            ace_mode: "text",
            language_id: 943571030u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Brightscript {
    pub fn info() -> Language {
        Language {
            name: "Brightscript",
            r#type: "programming",
            color: "#662D91",
            extensions: &[".brs"],
            tm_scope: "source.brs",
            ace_mode: "text",
            language_id: 39u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Browserslist {
    pub fn info() -> Language {
        Language {
            name: "Browserslist",
            r#type: "data",
            color: "#ffd539",
            extensions: &[],
            tm_scope: "text.browserslist",
            ace_mode: "text",
            language_id: 153503348u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[".browserslistrc", "browserslist"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl C {
    pub fn info() -> Language {
        Language {
            name: "C",
            r#type: "programming",
            color: "#555555",
            extensions: &[".c", ".cats", ".h", ".idc"],
            tm_scope: "source.c",
            ace_mode: "c_cpp",
            language_id: 41u64,
            aliases: &[],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-csrc"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["tcc"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Csharp {
    pub fn info() -> Language {
        Language {
            name: "C#",
            r#type: "programming",
            color: "#178600",
            extensions: &[".cs", ".cake", ".cs.pp", ".csx", ".linq"],
            tm_scope: "source.cs",
            ace_mode: "csharp",
            language_id: 42u64,
            aliases: &["csharp", "cake", "cakescript"],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-csharp"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Cpp {
    pub fn info() -> Language {
        Language {
            name: "C++",
            r#type: "programming",
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
            language_id: 43u64,
            aliases: &["cpp"],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-c++src"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl CObjDump {
    pub fn info() -> Language {
        Language {
            name: "C-ObjDump",
            r#type: "data",
            color: "",
            extensions: &[".c-objdump"],
            tm_scope: "objdump.x86asm",
            ace_mode: "assembly_x86",
            language_id: 44u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl C2hsHaskell {
    pub fn info() -> Language {
        Language {
            name: "C2hs Haskell",
            r#type: "programming",
            color: "",
            extensions: &[".chs"],
            tm_scope: "source.haskell",
            ace_mode: "haskell",
            language_id: 45u64,
            aliases: &["c2hs"],
            codemirror_mode: Some("haskell"),
            codemirror_mime_type: Some("text/x-haskell"),
            wrap: None,
            filenames: &[],
            group: Some("Haskell"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl CAPCDS {
    pub fn info() -> Language {
        Language {
            name: "CAP CDS",
            r#type: "programming",
            color: "#0092d1",
            extensions: &[".cds"],
            tm_scope: "source.cds",
            ace_mode: "text",
            language_id: 390788699u64,
            aliases: &["cds"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl CIL {
    pub fn info() -> Language {
        Language {
            name: "CIL",
            r#type: "data",
            color: "",
            extensions: &[".cil"],
            tm_scope: "source.cil",
            ace_mode: "text",
            language_id: 29176339u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl CLIPS {
    pub fn info() -> Language {
        Language {
            name: "CLIPS",
            r#type: "programming",
            color: "#00A300",
            extensions: &[".clp"],
            tm_scope: "source.clips",
            ace_mode: "text",
            language_id: 46u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl CMake {
    pub fn info() -> Language {
        Language {
            name: "CMake",
            r#type: "programming",
            color: "#DA3434",
            extensions: &[".cmake", ".cmake.in"],
            tm_scope: "source.cmake",
            ace_mode: "text",
            language_id: 47u64,
            aliases: &[],
            codemirror_mode: Some("cmake"),
            codemirror_mime_type: Some("text/x-cmake"),
            wrap: None,
            filenames: &["CMakeLists.txt"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl COBOL {
    pub fn info() -> Language {
        Language {
            name: "COBOL",
            r#type: "programming",
            color: "",
            extensions: &[".cob", ".cbl", ".ccp", ".cobol", ".cpy"],
            tm_scope: "source.cobol",
            ace_mode: "cobol",
            language_id: 48u64,
            aliases: &[],
            codemirror_mode: Some("cobol"),
            codemirror_mime_type: Some("text/x-cobol"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl CODEOWNERS {
    pub fn info() -> Language {
        Language {
            name: "CODEOWNERS",
            r#type: "data",
            color: "",
            extensions: &[],
            tm_scope: "text.codeowners",
            ace_mode: "gitignore",
            language_id: 321684729u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["CODEOWNERS"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl COLLADA {
    pub fn info() -> Language {
        Language {
            name: "COLLADA",
            r#type: "data",
            color: "#F1A42B",
            extensions: &[".dae"],
            tm_scope: "text.xml",
            ace_mode: "xml",
            language_id: 49u64,
            aliases: &[],
            codemirror_mode: Some("xml"),
            codemirror_mime_type: Some("text/xml"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl CSON {
    pub fn info() -> Language {
        Language {
            name: "CSON",
            r#type: "data",
            color: "#244776",
            extensions: &[".cson"],
            tm_scope: "source.coffee",
            ace_mode: "coffee",
            language_id: 424u64,
            aliases: &[],
            codemirror_mode: Some("coffeescript"),
            codemirror_mime_type: Some("text/x-coffeescript"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl CSS {
    pub fn info() -> Language {
        Language {
            name: "CSS",
            r#type: "markup",
            color: "#663399",
            extensions: &[".css"],
            tm_scope: "source.css",
            ace_mode: "css",
            language_id: 50u64,
            aliases: &[],
            codemirror_mode: Some("css"),
            codemirror_mime_type: Some("text/css"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl CSV {
    pub fn info() -> Language {
        Language {
            name: "CSV",
            r#type: "data",
            color: "#237346",
            extensions: &[".csv"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 51u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl CUE {
    pub fn info() -> Language {
        Language {
            name: "CUE",
            r#type: "programming",
            color: "#5886E1",
            extensions: &[".cue"],
            tm_scope: "source.cue",
            ace_mode: "text",
            language_id: 356063509u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl CWeb {
    pub fn info() -> Language {
        Language {
            name: "CWeb",
            r#type: "programming",
            color: "#00007a",
            extensions: &[".w"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 657332628u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl CabalConfig {
    pub fn info() -> Language {
        Language {
            name: "Cabal Config",
            r#type: "data",
            color: "#483465",
            extensions: &[".cabal"],
            tm_scope: "source.cabal",
            ace_mode: "haskell",
            language_id: 677095381u64,
            aliases: &["Cabal"],
            codemirror_mode: Some("haskell"),
            codemirror_mime_type: Some("text/x-haskell"),
            wrap: None,
            filenames: &["cabal.config", "cabal.project"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Caddyfile {
    pub fn info() -> Language {
        Language {
            name: "Caddyfile",
            r#type: "data",
            color: "#22b638",
            extensions: &[".caddyfile"],
            tm_scope: "source.Caddyfile",
            ace_mode: "text",
            language_id: 615465151u64,
            aliases: &["Caddy"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["Caddyfile"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Cadence {
    pub fn info() -> Language {
        Language {
            name: "Cadence",
            r#type: "programming",
            color: "#00ef8b",
            extensions: &[".cdc"],
            tm_scope: "source.cadence",
            ace_mode: "text",
            language_id: 270184138u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Cairo {
    pub fn info() -> Language {
        Language {
            name: "Cairo",
            r#type: "programming",
            color: "#ff4a48",
            extensions: &[".cairo"],
            tm_scope: "source.cairo",
            ace_mode: "text",
            language_id: 620599567u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: Some("Cairo"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl CairoZero {
    pub fn info() -> Language {
        Language {
            name: "Cairo Zero",
            r#type: "programming",
            color: "#ff4a48",
            extensions: &[".cairo"],
            tm_scope: "source.cairo0",
            ace_mode: "text",
            language_id: 891399890u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: Some("Cairo"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl CameLIGO {
    pub fn info() -> Language {
        Language {
            name: "CameLIGO",
            r#type: "programming",
            color: "#3be133",
            extensions: &[".mligo"],
            tm_scope: "source.mligo",
            ace_mode: "ocaml",
            language_id: 829207807u64,
            aliases: &[],
            codemirror_mode: Some("mllike"),
            codemirror_mime_type: Some("text/x-ocaml"),
            wrap: None,
            filenames: &[],
            group: Some("LigoLANG"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl CapnProto {
    pub fn info() -> Language {
        Language {
            name: "Cap'n Proto",
            r#type: "programming",
            color: "#c42727",
            extensions: &[".capnp"],
            tm_scope: "source.capnp",
            ace_mode: "text",
            language_id: 52u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Carbon {
    pub fn info() -> Language {
        Language {
            name: "Carbon",
            r#type: "programming",
            color: "#222222",
            extensions: &[".carbon"],
            tm_scope: "source.v",
            ace_mode: "golang",
            language_id: 55627273u64,
            aliases: &[],
            codemirror_mode: Some("go"),
            codemirror_mime_type: Some("text/x-go"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl CartoCSS {
    pub fn info() -> Language {
        Language {
            name: "CartoCSS",
            r#type: "programming",
            color: "",
            extensions: &[".mss"],
            tm_scope: "source.css.mss",
            ace_mode: "text",
            language_id: 53u64,
            aliases: &["Carto"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Ceylon {
    pub fn info() -> Language {
        Language {
            name: "Ceylon",
            r#type: "programming",
            color: "#dfa535",
            extensions: &[".ceylon"],
            tm_scope: "source.ceylon",
            ace_mode: "text",
            language_id: 54u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Chapel {
    pub fn info() -> Language {
        Language {
            name: "Chapel",
            r#type: "programming",
            color: "#8dc63f",
            extensions: &[".chpl"],
            tm_scope: "source.chapel",
            ace_mode: "text",
            language_id: 55u64,
            aliases: &["chpl"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Charity {
    pub fn info() -> Language {
        Language {
            name: "Charity",
            r#type: "programming",
            color: "",
            extensions: &[".ch"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 56u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Checksums {
    pub fn info() -> Language {
        Language {
            name: "Checksums",
            r#type: "data",
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
            language_id: 372063053u64,
            aliases: &["checksum", "hash", "hashes", "sum", "sums"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[
                "MD5SUMS",
                "SHA1SUMS",
                "SHA256SUMS",
                "SHA256SUMS.txt",
                "SHA512SUMS",
                "checksums.txt",
                "cksums",
                "md5sum.txt",
            ],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ChucK {
    pub fn info() -> Language {
        Language {
            name: "ChucK",
            r#type: "programming",
            color: "#3f8000",
            extensions: &[".ck"],
            tm_scope: "source.java",
            ace_mode: "java",
            language_id: 57u64,
            aliases: &[],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-java"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Circom {
    pub fn info() -> Language {
        Language {
            name: "Circom",
            r#type: "programming",
            color: "#707575",
            extensions: &[".circom"],
            tm_scope: "source.circom",
            ace_mode: "text",
            language_id: 1042332086u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Cirru {
    pub fn info() -> Language {
        Language {
            name: "Cirru",
            r#type: "programming",
            color: "#ccccff",
            extensions: &[".cirru"],
            tm_scope: "source.cirru",
            ace_mode: "cirru",
            language_id: 58u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Clarion {
    pub fn info() -> Language {
        Language {
            name: "Clarion",
            r#type: "programming",
            color: "#db901e",
            extensions: &[".clw"],
            tm_scope: "source.clarion",
            ace_mode: "text",
            language_id: 59u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Clarity {
    pub fn info() -> Language {
        Language {
            name: "Clarity",
            r#type: "programming",
            color: "#5546ff",
            extensions: &[".clar"],
            tm_scope: "source.clar",
            ace_mode: "lisp",
            language_id: 91493841u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ClassicASP {
    pub fn info() -> Language {
        Language {
            name: "Classic ASP",
            r#type: "programming",
            color: "#6a40fd",
            extensions: &[".asp"],
            tm_scope: "text.html.asp",
            ace_mode: "text",
            language_id: 8u64,
            aliases: &["asp"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Clean {
    pub fn info() -> Language {
        Language {
            name: "Clean",
            r#type: "programming",
            color: "#3F85AF",
            extensions: &[".icl", ".dcl"],
            tm_scope: "source.clean",
            ace_mode: "text",
            language_id: 60u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Click {
    pub fn info() -> Language {
        Language {
            name: "Click",
            r#type: "programming",
            color: "#E4E6F3",
            extensions: &[".click"],
            tm_scope: "source.click",
            ace_mode: "text",
            language_id: 61u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Clojure {
    pub fn info() -> Language {
        Language {
            name: "Clojure",
            r#type: "programming",
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
            language_id: 62u64,
            aliases: &[],
            codemirror_mode: Some("clojure"),
            codemirror_mime_type: Some("text/x-clojure"),
            wrap: None,
            filenames: &["riemann.config"],
            group: None,
            interpreters: &["bb"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ClosureTemplates {
    pub fn info() -> Language {
        Language {
            name: "Closure Templates",
            r#type: "markup",
            color: "#0d948f",
            extensions: &[".soy"],
            tm_scope: "text.html.soy",
            ace_mode: "soy_template",
            language_id: 357046146u64,
            aliases: &["soy"],
            codemirror_mode: Some("soy"),
            codemirror_mime_type: Some("text/x-soy"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl CloudFirestoreSecurityRules {
    pub fn info() -> Language {
        Language {
            name: "Cloud Firestore Security Rules",
            r#type: "data",
            color: "#FFA000",
            extensions: &[],
            tm_scope: "source.firestore",
            ace_mode: "less",
            language_id: 407996372u64,
            aliases: &[],
            codemirror_mode: Some("css"),
            codemirror_mime_type: Some("text/css"),
            wrap: None,
            filenames: &["firestore.rules"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl CoNLLU {
    pub fn info() -> Language {
        Language {
            name: "CoNLL-U",
            r#type: "data",
            color: "",
            extensions: &[".conllu", ".conll"],
            tm_scope: "text.conllu",
            ace_mode: "text",
            language_id: 421026389u64,
            aliases: &["CoNLL", "CoNLL-X"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl CodeQL {
    pub fn info() -> Language {
        Language {
            name: "CodeQL",
            r#type: "programming",
            color: "#140f46",
            extensions: &[".ql", ".qll"],
            tm_scope: "source.ql",
            ace_mode: "text",
            language_id: 424259634u64,
            aliases: &["ql"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl CoffeeScript {
    pub fn info() -> Language {
        Language {
            name: "CoffeeScript",
            r#type: "programming",
            color: "#244776",
            extensions: &[".coffee", "._coffee", ".cake", ".cjsx", ".iced"],
            tm_scope: "source.coffee",
            ace_mode: "coffee",
            language_id: 63u64,
            aliases: &["coffee", "coffee-script"],
            codemirror_mode: Some("coffeescript"),
            codemirror_mime_type: Some("text/x-coffeescript"),
            wrap: None,
            filenames: &["Cakefile"],
            group: None,
            interpreters: &["coffee"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ColdFusion {
    pub fn info() -> Language {
        Language {
            name: "ColdFusion",
            r#type: "programming",
            color: "#ed2cd6",
            extensions: &[".cfm", ".cfml"],
            tm_scope: "text.html.cfm",
            ace_mode: "coldfusion",
            language_id: 64u64,
            aliases: &["cfm", "cfml", "coldfusion html"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ColdFusionCFC {
    pub fn info() -> Language {
        Language {
            name: "ColdFusion CFC",
            r#type: "programming",
            color: "#ed2cd6",
            extensions: &[".cfc"],
            tm_scope: "source.cfscript",
            ace_mode: "coldfusion",
            language_id: 65u64,
            aliases: &["cfc"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: Some("ColdFusion"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl CommonLisp {
    pub fn info() -> Language {
        Language {
            name: "Common Lisp",
            r#type: "programming",
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
            language_id: 66u64,
            aliases: &["lisp"],
            codemirror_mode: Some("commonlisp"),
            codemirror_mime_type: Some("text/x-common-lisp"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["lisp", "sbcl", "ccl", "clisp", "ecl"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl CommonWorkflowLanguage {
    pub fn info() -> Language {
        Language {
            name: "Common Workflow Language",
            r#type: "programming",
            color: "#B5314C",
            extensions: &[".cwl"],
            tm_scope: "source.cwl",
            ace_mode: "yaml",
            language_id: 988547172u64,
            aliases: &["cwl"],
            codemirror_mode: Some("yaml"),
            codemirror_mime_type: Some("text/x-yaml"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["cwl-runner"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ComponentPascal {
    pub fn info() -> Language {
        Language {
            name: "Component Pascal",
            r#type: "programming",
            color: "#B0CE4E",
            extensions: &[".cp", ".cps"],
            tm_scope: "source.pascal",
            ace_mode: "pascal",
            language_id: 67u64,
            aliases: &[],
            codemirror_mode: Some("pascal"),
            codemirror_mime_type: Some("text/x-pascal"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Cool {
    pub fn info() -> Language {
        Language {
            name: "Cool",
            r#type: "programming",
            color: "",
            extensions: &[".cl"],
            tm_scope: "source.cool",
            ace_mode: "text",
            language_id: 68u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Coq {
    pub fn info() -> Language {
        Language {
            name: "Coq",
            r#type: "programming",
            color: "#d0b68c",
            extensions: &[".coq", ".v"],
            tm_scope: "source.coq",
            ace_mode: "text",
            language_id: 69u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl CppObjDump {
    pub fn info() -> Language {
        Language {
            name: "Cpp-ObjDump",
            r#type: "data",
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
            language_id: 70u64,
            aliases: &["c++-objdump"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Creole {
    pub fn info() -> Language {
        Language {
            name: "Creole",
            r#type: "prose",
            color: "",
            extensions: &[".creole"],
            tm_scope: "text.html.creole",
            ace_mode: "text",
            language_id: 71u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: Some(true),
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Crystal {
    pub fn info() -> Language {
        Language {
            name: "Crystal",
            r#type: "programming",
            color: "#000100",
            extensions: &[".cr"],
            tm_scope: "source.crystal",
            ace_mode: "ruby",
            language_id: 72u64,
            aliases: &[],
            codemirror_mode: Some("crystal"),
            codemirror_mime_type: Some("text/x-crystal"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["crystal"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Csound {
    pub fn info() -> Language {
        Language {
            name: "Csound",
            r#type: "programming",
            color: "#1a1a1a",
            extensions: &[".orc", ".udo"],
            tm_scope: "source.csound",
            ace_mode: "csound_orchestra",
            language_id: 73u64,
            aliases: &["csound-orc"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl CsoundDocument {
    pub fn info() -> Language {
        Language {
            name: "Csound Document",
            r#type: "programming",
            color: "#1a1a1a",
            extensions: &[".csd"],
            tm_scope: "source.csound-document",
            ace_mode: "csound_document",
            language_id: 74u64,
            aliases: &["csound-csd"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl CsoundScore {
    pub fn info() -> Language {
        Language {
            name: "Csound Score",
            r#type: "programming",
            color: "#1a1a1a",
            extensions: &[".sco"],
            tm_scope: "source.csound-score",
            ace_mode: "csound_score",
            language_id: 75u64,
            aliases: &["csound-sco"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Cuda {
    pub fn info() -> Language {
        Language {
            name: "Cuda",
            r#type: "programming",
            color: "#3A4E3A",
            extensions: &[".cu", ".cuh"],
            tm_scope: "source.cuda-c++",
            ace_mode: "c_cpp",
            language_id: 77u64,
            aliases: &[],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-c++src"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl CueSheet {
    pub fn info() -> Language {
        Language {
            name: "Cue Sheet",
            r#type: "data",
            color: "",
            extensions: &[".cue"],
            tm_scope: "source.cuesheet",
            ace_mode: "text",
            language_id: 942714150u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Curry {
    pub fn info() -> Language {
        Language {
            name: "Curry",
            r#type: "programming",
            color: "#531242",
            extensions: &[".curry"],
            tm_scope: "source.curry",
            ace_mode: "haskell",
            language_id: 439829048u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Cycript {
    pub fn info() -> Language {
        Language {
            name: "Cycript",
            r#type: "programming",
            color: "",
            extensions: &[".cy"],
            tm_scope: "source.js",
            ace_mode: "javascript",
            language_id: 78u64,
            aliases: &[],
            codemirror_mode: Some("javascript"),
            codemirror_mime_type: Some("text/javascript"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Cylc {
    pub fn info() -> Language {
        Language {
            name: "Cylc",
            r#type: "data",
            color: "#00b3fd",
            extensions: &[".cylc"],
            tm_scope: "source.cylc",
            ace_mode: "ini",
            language_id: 476447814u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["suite.rc"],
            group: Some("INI"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Cypher {
    pub fn info() -> Language {
        Language {
            name: "Cypher",
            r#type: "programming",
            color: "#34c0eb",
            extensions: &[".cyp", ".cypher"],
            tm_scope: "source.cypher",
            ace_mode: "text",
            language_id: 850806976u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Cython {
    pub fn info() -> Language {
        Language {
            name: "Cython",
            r#type: "programming",
            color: "#fedf5b",
            extensions: &[".pyx", ".pxd", ".pxi"],
            tm_scope: "source.cython",
            ace_mode: "text",
            language_id: 79u64,
            aliases: &["pyrex"],
            codemirror_mode: Some("python"),
            codemirror_mime_type: Some("text/x-cython"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl D {
    pub fn info() -> Language {
        Language {
            name: "D",
            r#type: "programming",
            color: "#ba595e",
            extensions: &[".d", ".di"],
            tm_scope: "source.d",
            ace_mode: "d",
            language_id: 80u64,
            aliases: &["Dlang"],
            codemirror_mode: Some("d"),
            codemirror_mime_type: Some("text/x-d"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl DObjDump {
    pub fn info() -> Language {
        Language {
            name: "D-ObjDump",
            r#type: "data",
            color: "",
            extensions: &[".d-objdump"],
            tm_scope: "objdump.x86asm",
            ace_mode: "assembly_x86",
            language_id: 81u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl D2 {
    pub fn info() -> Language {
        Language {
            name: "D2",
            r#type: "markup",
            color: "#526ee8",
            extensions: &[".d2"],
            tm_scope: "source.d2",
            ace_mode: "text",
            language_id: 37531557u64,
            aliases: &["d2lang"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl DIGITALCommandLanguage {
    pub fn info() -> Language {
        Language {
            name: "DIGITAL Command Language",
            r#type: "programming",
            color: "",
            extensions: &[".com"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 82u64,
            aliases: &["dcl"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl DM {
    pub fn info() -> Language {
        Language {
            name: "DM",
            r#type: "programming",
            color: "#447265",
            extensions: &[".dm"],
            tm_scope: "source.dm",
            ace_mode: "c_cpp",
            language_id: 83u64,
            aliases: &["byond"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl DNSZone {
    pub fn info() -> Language {
        Language {
            name: "DNS Zone",
            r#type: "data",
            color: "",
            extensions: &[".zone", ".arpa"],
            tm_scope: "text.zone_file",
            ace_mode: "text",
            language_id: 84u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl DTrace {
    pub fn info() -> Language {
        Language {
            name: "DTrace",
            r#type: "programming",
            color: "",
            extensions: &[".d"],
            tm_scope: "source.c",
            ace_mode: "c_cpp",
            language_id: 85u64,
            aliases: &["dtrace-script"],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-csrc"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["dtrace"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Dafny {
    pub fn info() -> Language {
        Language {
            name: "Dafny",
            r#type: "programming",
            color: "#FFEC25",
            extensions: &[".dfy"],
            tm_scope: "text.dfy.dafny",
            ace_mode: "text",
            language_id: 969323346u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["dafny"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl DarcsPatch {
    pub fn info() -> Language {
        Language {
            name: "Darcs Patch",
            r#type: "data",
            color: "#8eff23",
            extensions: &[".darcspatch", ".dpatch"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 86u64,
            aliases: &["dpatch"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Dart {
    pub fn info() -> Language {
        Language {
            name: "Dart",
            r#type: "programming",
            color: "#00B4AB",
            extensions: &[".dart"],
            tm_scope: "source.dart",
            ace_mode: "dart",
            language_id: 87u64,
            aliases: &[],
            codemirror_mode: Some("dart"),
            codemirror_mime_type: Some("application/dart"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["dart"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl DataWeave {
    pub fn info() -> Language {
        Language {
            name: "DataWeave",
            r#type: "programming",
            color: "#003a52",
            extensions: &[".dwl"],
            tm_scope: "source.data-weave",
            ace_mode: "text",
            language_id: 974514097u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl DebianPackageControlFile {
    pub fn info() -> Language {
        Language {
            name: "Debian Package Control File",
            r#type: "data",
            color: "#D70751",
            extensions: &[".dsc"],
            tm_scope: "source.deb-control",
            ace_mode: "text",
            language_id: 527438264u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl DenizenScript {
    pub fn info() -> Language {
        Language {
            name: "DenizenScript",
            r#type: "programming",
            color: "#FBEE96",
            extensions: &[".dsc"],
            tm_scope: "source.denizenscript",
            ace_mode: "yaml",
            language_id: 435000929u64,
            aliases: &[],
            codemirror_mode: Some("yaml"),
            codemirror_mime_type: Some("text/x-yaml"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Dhall {
    pub fn info() -> Language {
        Language {
            name: "Dhall",
            r#type: "programming",
            color: "#dfafff",
            extensions: &[".dhall"],
            tm_scope: "source.haskell",
            ace_mode: "haskell",
            language_id: 793969321u64,
            aliases: &[],
            codemirror_mode: Some("haskell"),
            codemirror_mime_type: Some("text/x-haskell"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Diff {
    pub fn info() -> Language {
        Language {
            name: "Diff",
            r#type: "data",
            color: "",
            extensions: &[".diff", ".patch"],
            tm_scope: "source.diff",
            ace_mode: "diff",
            language_id: 88u64,
            aliases: &["udiff"],
            codemirror_mode: Some("diff"),
            codemirror_mime_type: Some("text/x-diff"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl DirectX3DFile {
    pub fn info() -> Language {
        Language {
            name: "DirectX 3D File",
            r#type: "data",
            color: "#aace60",
            extensions: &[".x"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 201049282u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Dockerfile {
    pub fn info() -> Language {
        Language {
            name: "Dockerfile",
            r#type: "programming",
            color: "#384d54",
            extensions: &[".dockerfile", ".containerfile"],
            tm_scope: "source.dockerfile",
            ace_mode: "dockerfile",
            language_id: 89u64,
            aliases: &["Containerfile"],
            codemirror_mode: Some("dockerfile"),
            codemirror_mime_type: Some("text/x-dockerfile"),
            wrap: None,
            filenames: &["Containerfile", "Dockerfile"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Dogescript {
    pub fn info() -> Language {
        Language {
            name: "Dogescript",
            r#type: "programming",
            color: "#cca760",
            extensions: &[".djs"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 90u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Dotenv {
    pub fn info() -> Language {
        Language {
            name: "Dotenv",
            r#type: "data",
            color: "#e5d559",
            extensions: &[".env"],
            tm_scope: "source.dotenv",
            ace_mode: "text",
            language_id: 111148035u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[
                ".env",
                ".env.ci",
                ".env.dev",
                ".env.development",
                ".env.development.local",
                ".env.example",
                ".env.local",
                ".env.prod",
                ".env.production",
                ".env.sample",
                ".env.staging",
                ".env.test",
                ".env.testing",
            ],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Dune {
    pub fn info() -> Language {
        Language {
            name: "Dune",
            r#type: "programming",
            color: "#89421e",
            extensions: &[],
            tm_scope: "source.dune",
            ace_mode: "lisp",
            language_id: 754574151u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["dune-project"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Dylan {
    pub fn info() -> Language {
        Language {
            name: "Dylan",
            r#type: "programming",
            color: "#6c616e",
            extensions: &[".dylan", ".dyl", ".intr", ".lid"],
            tm_scope: "source.dylan",
            ace_mode: "text",
            language_id: 91u64,
            aliases: &[],
            codemirror_mode: Some("dylan"),
            codemirror_mime_type: Some("text/x-dylan"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl E {
    pub fn info() -> Language {
        Language {
            name: "E",
            r#type: "programming",
            color: "#ccce35",
            extensions: &[".e"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 92u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["rune"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl EMail {
    pub fn info() -> Language {
        Language {
            name: "E-mail",
            r#type: "data",
            color: "",
            extensions: &[".eml", ".mbox"],
            tm_scope: "text.eml.basic",
            ace_mode: "text",
            language_id: 529653389u64,
            aliases: &["email", "eml", "mail", "mbox"],
            codemirror_mode: Some("mbox"),
            codemirror_mime_type: Some("application/mbox"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl EBNF {
    pub fn info() -> Language {
        Language {
            name: "EBNF",
            r#type: "data",
            color: "",
            extensions: &[".ebnf"],
            tm_scope: "source.ebnf",
            ace_mode: "text",
            language_id: 430u64,
            aliases: &[],
            codemirror_mode: Some("ebnf"),
            codemirror_mime_type: Some("text/x-ebnf"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ECL {
    pub fn info() -> Language {
        Language {
            name: "ECL",
            r#type: "programming",
            color: "#8a1267",
            extensions: &[".ecl", ".eclxml"],
            tm_scope: "source.ecl",
            ace_mode: "text",
            language_id: 93u64,
            aliases: &[],
            codemirror_mode: Some("ecl"),
            codemirror_mime_type: Some("text/x-ecl"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ECLiPSe {
    pub fn info() -> Language {
        Language {
            name: "ECLiPSe",
            r#type: "programming",
            color: "#001d9d",
            extensions: &[".ecl"],
            tm_scope: "source.prolog.eclipse",
            ace_mode: "prolog",
            language_id: 94u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: Some("Prolog"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl EJS {
    pub fn info() -> Language {
        Language {
            name: "EJS",
            r#type: "markup",
            color: "#a91e50",
            extensions: &[".ejs", ".ect", ".ejs.t", ".jst"],
            tm_scope: "text.html.js",
            ace_mode: "ejs",
            language_id: 95u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl EQ {
    pub fn info() -> Language {
        Language {
            name: "EQ",
            r#type: "programming",
            color: "#a78649",
            extensions: &[".eq"],
            tm_scope: "source.cs",
            ace_mode: "csharp",
            language_id: 96u64,
            aliases: &[],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-csharp"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Eagle {
    pub fn info() -> Language {
        Language {
            name: "Eagle",
            r#type: "data",
            color: "",
            extensions: &[".sch", ".brd"],
            tm_scope: "text.xml",
            ace_mode: "xml",
            language_id: 97u64,
            aliases: &[],
            codemirror_mode: Some("xml"),
            codemirror_mime_type: Some("text/xml"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Earthly {
    pub fn info() -> Language {
        Language {
            name: "Earthly",
            r#type: "programming",
            color: "#2af0ff",
            extensions: &[],
            tm_scope: "source.earthfile",
            ace_mode: "text",
            language_id: 963512632u64,
            aliases: &["Earthfile"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["Earthfile"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Easybuild {
    pub fn info() -> Language {
        Language {
            name: "Easybuild",
            r#type: "data",
            color: "#069406",
            extensions: &[".eb"],
            tm_scope: "source.python",
            ace_mode: "python",
            language_id: 342840477u64,
            aliases: &[],
            codemirror_mode: Some("python"),
            codemirror_mime_type: Some("text/x-python"),
            wrap: None,
            filenames: &[],
            group: Some("Python"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl EcereProjects {
    pub fn info() -> Language {
        Language {
            name: "Ecere Projects",
            r#type: "data",
            color: "#913960",
            extensions: &[".epj"],
            tm_scope: "source.json",
            ace_mode: "json",
            language_id: 98u64,
            aliases: &[],
            codemirror_mode: Some("javascript"),
            codemirror_mime_type: Some("application/json"),
            wrap: None,
            filenames: &[],
            group: Some("JavaScript"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Ecmarkup {
    pub fn info() -> Language {
        Language {
            name: "Ecmarkup",
            r#type: "markup",
            color: "#eb8131",
            extensions: &[".html"],
            tm_scope: "text.html.ecmarkup",
            ace_mode: "html",
            language_id: 844766630u64,
            aliases: &["ecmarkdown"],
            codemirror_mode: Some("htmlmixed"),
            codemirror_mime_type: Some("text/html"),
            wrap: None,
            filenames: &[],
            group: Some("HTML"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Edge {
    pub fn info() -> Language {
        Language {
            name: "Edge",
            r#type: "markup",
            color: "#0dffe0",
            extensions: &[".edge"],
            tm_scope: "text.html.edge",
            ace_mode: "html",
            language_id: 460509620u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl EdgeQL {
    pub fn info() -> Language {
        Language {
            name: "EdgeQL",
            r#type: "programming",
            color: "#31A7FF",
            extensions: &[".edgeql", ".esdl"],
            tm_scope: "source.edgeql",
            ace_mode: "text",
            language_id: 925235833u64,
            aliases: &["esdl"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl EditorConfig {
    pub fn info() -> Language {
        Language {
            name: "EditorConfig",
            r#type: "data",
            color: "#fff1f2",
            extensions: &[".editorconfig"],
            tm_scope: "source.editorconfig",
            ace_mode: "ini",
            language_id: 96139566u64,
            aliases: &["editor-config"],
            codemirror_mode: Some("properties"),
            codemirror_mime_type: Some("text/x-properties"),
            wrap: None,
            filenames: &[".editorconfig"],
            group: Some("INI"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl EdjeDataCollection {
    pub fn info() -> Language {
        Language {
            name: "Edje Data Collection",
            r#type: "data",
            color: "",
            extensions: &[".edc"],
            tm_scope: "source.c++",
            ace_mode: "c_cpp",
            language_id: 342840478u64,
            aliases: &[],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-c++src"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Eiffel {
    pub fn info() -> Language {
        Language {
            name: "Eiffel",
            r#type: "programming",
            color: "#4d6977",
            extensions: &[".e"],
            tm_scope: "source.eiffel",
            ace_mode: "eiffel",
            language_id: 99u64,
            aliases: &[],
            codemirror_mode: Some("eiffel"),
            codemirror_mime_type: Some("text/x-eiffel"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Elixir {
    pub fn info() -> Language {
        Language {
            name: "Elixir",
            r#type: "programming",
            color: "#6e4a7e",
            extensions: &[".ex", ".exs"],
            tm_scope: "source.elixir",
            ace_mode: "elixir",
            language_id: 100u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["mix.lock"],
            group: None,
            interpreters: &["elixir"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Elm {
    pub fn info() -> Language {
        Language {
            name: "Elm",
            r#type: "programming",
            color: "#60B5CC",
            extensions: &[".elm"],
            tm_scope: "source.elm",
            ace_mode: "elm",
            language_id: 101u64,
            aliases: &[],
            codemirror_mode: Some("elm"),
            codemirror_mime_type: Some("text/x-elm"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Elvish {
    pub fn info() -> Language {
        Language {
            name: "Elvish",
            r#type: "programming",
            color: "#55BB55",
            extensions: &[".elv"],
            tm_scope: "source.elvish",
            ace_mode: "text",
            language_id: 570996448u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["elvish"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ElvishTranscript {
    pub fn info() -> Language {
        Language {
            name: "Elvish Transcript",
            r#type: "programming",
            color: "#55BB55",
            extensions: &[],
            tm_scope: "source.elvish-transcript",
            ace_mode: "text",
            language_id: 452025714u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: Some("Elvish"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl EmacsLisp {
    pub fn info() -> Language {
        Language {
            name: "Emacs Lisp",
            r#type: "programming",
            color: "#c065db",
            extensions: &[".el", ".emacs", ".emacs.desktop"],
            tm_scope: "source.emacs.lisp",
            ace_mode: "lisp",
            language_id: 102u64,
            aliases: &["elisp", "emacs"],
            codemirror_mode: Some("commonlisp"),
            codemirror_mime_type: Some("text/x-common-lisp"),
            wrap: None,
            filenames: &[
                ".abbrev_defs",
                ".emacs",
                ".emacs.desktop",
                ".gnus",
                ".spacemacs",
                ".viper",
                "Cask",
                "Project.ede",
                "_emacs",
                "abbrev_defs",
            ],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl EmberScript {
    pub fn info() -> Language {
        Language {
            name: "EmberScript",
            r#type: "programming",
            color: "#FFF4F3",
            extensions: &[".em", ".emberscript"],
            tm_scope: "source.coffee",
            ace_mode: "coffee",
            language_id: 103u64,
            aliases: &[],
            codemirror_mode: Some("coffeescript"),
            codemirror_mime_type: Some("text/x-coffeescript"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Erlang {
    pub fn info() -> Language {
        Language {
            name: "Erlang",
            r#type: "programming",
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
            language_id: 104u64,
            aliases: &[],
            codemirror_mode: Some("erlang"),
            codemirror_mime_type: Some("text/x-erlang"),
            wrap: None,
            filenames: &["Emakefile", "rebar.config", "rebar.config.lock", "rebar.lock"],
            group: None,
            interpreters: &["escript"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Euphoria {
    pub fn info() -> Language {
        Language {
            name: "Euphoria",
            r#type: "programming",
            color: "#FF790B",
            extensions: &[".e", ".ex"],
            tm_scope: "source.euphoria",
            ace_mode: "text",
            language_id: 880693982u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["eui", "euiw"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Fsharp {
    pub fn info() -> Language {
        Language {
            name: "F#",
            r#type: "programming",
            color: "#b845fc",
            extensions: &[".fs", ".fsi", ".fsx"],
            tm_scope: "source.fsharp",
            ace_mode: "text",
            language_id: 105u64,
            aliases: &["fsharp"],
            codemirror_mode: Some("mllike"),
            codemirror_mime_type: Some("text/x-fsharp"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Fstar {
    pub fn info() -> Language {
        Language {
            name: "F*",
            r#type: "programming",
            color: "#572e30",
            extensions: &[".fst", ".fsti"],
            tm_scope: "source.fstar",
            ace_mode: "text",
            language_id: 336943375u64,
            aliases: &["fstar"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: Some("Fstar"),
            searchable: None,
        }
    }
}
impl FIGletFont {
    pub fn info() -> Language {
        Language {
            name: "FIGlet Font",
            r#type: "data",
            color: "#FFDDBB",
            extensions: &[".flf"],
            tm_scope: "source.figfont",
            ace_mode: "text",
            language_id: 686129783u64,
            aliases: &["FIGfont"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl FIRRTL {
    pub fn info() -> Language {
        Language {
            name: "FIRRTL",
            r#type: "programming",
            color: "#2f632f",
            extensions: &[".fir"],
            tm_scope: "source.firrtl",
            ace_mode: "text",
            language_id: 906694254u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl FLUX {
    pub fn info() -> Language {
        Language {
            name: "FLUX",
            r#type: "programming",
            color: "#88ccff",
            extensions: &[".fx", ".flux"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 106u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Factor {
    pub fn info() -> Language {
        Language {
            name: "Factor",
            r#type: "programming",
            color: "#636746",
            extensions: &[".factor"],
            tm_scope: "source.factor",
            ace_mode: "text",
            language_id: 108u64,
            aliases: &[],
            codemirror_mode: Some("factor"),
            codemirror_mime_type: Some("text/x-factor"),
            wrap: None,
            filenames: &[".factor-boot-rc", ".factor-rc"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Fancy {
    pub fn info() -> Language {
        Language {
            name: "Fancy",
            r#type: "programming",
            color: "#7b9db4",
            extensions: &[".fy", ".fancypack"],
            tm_scope: "source.fancy",
            ace_mode: "text",
            language_id: 109u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["Fakefile"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Fantom {
    pub fn info() -> Language {
        Language {
            name: "Fantom",
            r#type: "programming",
            color: "#14253c",
            extensions: &[".fan"],
            tm_scope: "source.fan",
            ace_mode: "text",
            language_id: 110u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Faust {
    pub fn info() -> Language {
        Language {
            name: "Faust",
            r#type: "programming",
            color: "#c37240",
            extensions: &[".dsp"],
            tm_scope: "source.faust",
            ace_mode: "text",
            language_id: 622529198u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Fennel {
    pub fn info() -> Language {
        Language {
            name: "Fennel",
            r#type: "programming",
            color: "#fff3d7",
            extensions: &[".fnl"],
            tm_scope: "source.fnl",
            ace_mode: "text",
            language_id: 239946126u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["fennel"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl FilebenchWML {
    pub fn info() -> Language {
        Language {
            name: "Filebench WML",
            r#type: "programming",
            color: "#F6B900",
            extensions: &[".f"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 111u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Filterscript {
    pub fn info() -> Language {
        Language {
            name: "Filterscript",
            r#type: "programming",
            color: "",
            extensions: &[".fs"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 112u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: Some("RenderScript"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Fluent {
    pub fn info() -> Language {
        Language {
            name: "Fluent",
            r#type: "programming",
            color: "#ffcc33",
            extensions: &[".ftl"],
            tm_scope: "source.ftl",
            ace_mode: "text",
            language_id: 206353404u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Formatted {
    pub fn info() -> Language {
        Language {
            name: "Formatted",
            r#type: "data",
            color: "",
            extensions: &[".for", ".eam.fs"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 113u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Forth {
    pub fn info() -> Language {
        Language {
            name: "Forth",
            r#type: "programming",
            color: "#341708",
            extensions: &[".fth", ".4th", ".f", ".for", ".forth", ".fr", ".frt", ".fs"],
            tm_scope: "source.forth",
            ace_mode: "forth",
            language_id: 114u64,
            aliases: &[],
            codemirror_mode: Some("forth"),
            codemirror_mime_type: Some("text/x-forth"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Fortran {
    pub fn info() -> Language {
        Language {
            name: "Fortran",
            r#type: "programming",
            color: "#4d41b1",
            extensions: &[".f", ".f77", ".for", ".fpp"],
            tm_scope: "source.fortran",
            ace_mode: "text",
            language_id: 107u64,
            aliases: &[],
            codemirror_mode: Some("fortran"),
            codemirror_mime_type: Some("text/x-fortran"),
            wrap: None,
            filenames: &[],
            group: Some("Fortran"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl FortranFreeForm {
    pub fn info() -> Language {
        Language {
            name: "Fortran Free Form",
            r#type: "programming",
            color: "#4d41b1",
            extensions: &[".f90", ".f03", ".f08", ".f95"],
            tm_scope: "source.fortran.modern",
            ace_mode: "text",
            language_id: 761352333u64,
            aliases: &[],
            codemirror_mode: Some("fortran"),
            codemirror_mime_type: Some("text/x-fortran"),
            wrap: None,
            filenames: &[],
            group: Some("Fortran"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl FreeBASIC {
    pub fn info() -> Language {
        Language {
            name: "FreeBASIC",
            r#type: "programming",
            color: "#141AC9",
            extensions: &[".bi", ".bas"],
            tm_scope: "source.vbnet",
            ace_mode: "text",
            language_id: 472896659u64,
            aliases: &["fb"],
            codemirror_mode: Some("vb"),
            codemirror_mime_type: Some("text/x-vb"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl FreeMarker {
    pub fn info() -> Language {
        Language {
            name: "FreeMarker",
            r#type: "programming",
            color: "#0050b2",
            extensions: &[".ftl"],
            tm_scope: "text.html.ftl",
            ace_mode: "ftl",
            language_id: 115u64,
            aliases: &["ftl"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Frege {
    pub fn info() -> Language {
        Language {
            name: "Frege",
            r#type: "programming",
            color: "#00cafe",
            extensions: &[".fr"],
            tm_scope: "source.haskell",
            ace_mode: "haskell",
            language_id: 116u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Futhark {
    pub fn info() -> Language {
        Language {
            name: "Futhark",
            r#type: "programming",
            color: "#5f021f",
            extensions: &[".fut"],
            tm_scope: "source.futhark",
            ace_mode: "text",
            language_id: 97358117u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GCode {
    pub fn info() -> Language {
        Language {
            name: "G-code",
            r#type: "programming",
            color: "#D08CF2",
            extensions: &[".g", ".cnc", ".gco", ".gcode"],
            tm_scope: "source.gcode",
            ace_mode: "gcode",
            language_id: 117u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GAML {
    pub fn info() -> Language {
        Language {
            name: "GAML",
            r#type: "programming",
            color: "#FFC766",
            extensions: &[".gaml"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 290345951u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GAMS {
    pub fn info() -> Language {
        Language {
            name: "GAMS",
            r#type: "programming",
            color: "#f49a22",
            extensions: &[".gms"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 118u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GAP {
    pub fn info() -> Language {
        Language {
            name: "GAP",
            r#type: "programming",
            color: "#0000cc",
            extensions: &[".g", ".gap", ".gd", ".gi", ".tst"],
            tm_scope: "source.gap",
            ace_mode: "text",
            language_id: 119u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GCCMachineDescription {
    pub fn info() -> Language {
        Language {
            name: "GCC Machine Description",
            r#type: "programming",
            color: "#FFCFAB",
            extensions: &[".md"],
            tm_scope: "source.lisp",
            ace_mode: "lisp",
            language_id: 121u64,
            aliases: &[],
            codemirror_mode: Some("commonlisp"),
            codemirror_mime_type: Some("text/x-common-lisp"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GDB {
    pub fn info() -> Language {
        Language {
            name: "GDB",
            r#type: "programming",
            color: "",
            extensions: &[".gdb", ".gdbinit"],
            tm_scope: "source.gdb",
            ace_mode: "text",
            language_id: 122u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GDScript {
    pub fn info() -> Language {
        Language {
            name: "GDScript",
            r#type: "programming",
            color: "#355570",
            extensions: &[".gd"],
            tm_scope: "source.gdscript",
            ace_mode: "text",
            language_id: 123u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GEDCOM {
    pub fn info() -> Language {
        Language {
            name: "GEDCOM",
            r#type: "data",
            color: "#003058",
            extensions: &[".ged"],
            tm_scope: "source.gedcom",
            ace_mode: "text",
            language_id: 459577965u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GLSL {
    pub fn info() -> Language {
        Language {
            name: "GLSL",
            r#type: "programming",
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
            language_id: 124u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GN {
    pub fn info() -> Language {
        Language {
            name: "GN",
            r#type: "data",
            color: "",
            extensions: &[".gn", ".gni"],
            tm_scope: "source.gn",
            ace_mode: "python",
            language_id: 302957008u64,
            aliases: &[],
            codemirror_mode: Some("python"),
            codemirror_mime_type: Some("text/x-python"),
            wrap: None,
            filenames: &[".gn"],
            group: None,
            interpreters: &["gn"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GSC {
    pub fn info() -> Language {
        Language {
            name: "GSC",
            r#type: "programming",
            color: "#FF6800",
            extensions: &[".gsc", ".csc", ".gsh"],
            tm_scope: "source.gsc",
            ace_mode: "c_cpp",
            language_id: 257856279u64,
            aliases: &[],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-csrc"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GameMakerLanguage {
    pub fn info() -> Language {
        Language {
            name: "Game Maker Language",
            r#type: "programming",
            color: "#71b417",
            extensions: &[".gml"],
            tm_scope: "source.c++",
            ace_mode: "c_cpp",
            language_id: 125u64,
            aliases: &[],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-c++src"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Gemfilelock {
    pub fn info() -> Language {
        Language {
            name: "Gemfile.lock",
            r#type: "data",
            color: "#701516",
            extensions: &[],
            tm_scope: "source.gemfile-lock",
            ace_mode: "text",
            language_id: 907065713u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["Gemfile.lock"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: Some(false),
        }
    }
}
impl Gemini {
    pub fn info() -> Language {
        Language {
            name: "Gemini",
            r#type: "prose",
            color: "#ff6900",
            extensions: &[".gmi"],
            tm_scope: "source.gemini",
            ace_mode: "text",
            language_id: 310828396u64,
            aliases: &["gemtext"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: Some(true),
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Genero4gl {
    pub fn info() -> Language {
        Language {
            name: "Genero 4gl",
            r#type: "programming",
            color: "#63408e",
            extensions: &[".4gl"],
            tm_scope: "source.genero-4gl",
            ace_mode: "text",
            language_id: 986054050u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GeneroPer {
    pub fn info() -> Language {
        Language {
            name: "Genero per",
            r#type: "markup",
            color: "#d8df39",
            extensions: &[".per"],
            tm_scope: "source.genero-per",
            ace_mode: "text",
            language_id: 902995658u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Genie {
    pub fn info() -> Language {
        Language {
            name: "Genie",
            r#type: "programming",
            color: "#fb855d",
            extensions: &[".gs"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 792408528u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Genshi {
    pub fn info() -> Language {
        Language {
            name: "Genshi",
            r#type: "programming",
            color: "#951531",
            extensions: &[".kid"],
            tm_scope: "text.xml.genshi",
            ace_mode: "xml",
            language_id: 126u64,
            aliases: &["xml+genshi", "xml+kid"],
            codemirror_mode: Some("xml"),
            codemirror_mime_type: Some("text/xml"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GentooEbuild {
    pub fn info() -> Language {
        Language {
            name: "Gentoo Ebuild",
            r#type: "programming",
            color: "#9400ff",
            extensions: &[".ebuild"],
            tm_scope: "source.shell",
            ace_mode: "sh",
            language_id: 127u64,
            aliases: &[],
            codemirror_mode: Some("shell"),
            codemirror_mime_type: Some("text/x-sh"),
            wrap: None,
            filenames: &[],
            group: Some("Shell"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GentooEclass {
    pub fn info() -> Language {
        Language {
            name: "Gentoo Eclass",
            r#type: "programming",
            color: "#9400ff",
            extensions: &[".eclass"],
            tm_scope: "source.shell",
            ace_mode: "sh",
            language_id: 128u64,
            aliases: &[],
            codemirror_mode: Some("shell"),
            codemirror_mime_type: Some("text/x-sh"),
            wrap: None,
            filenames: &[],
            group: Some("Shell"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GerberImage {
    pub fn info() -> Language {
        Language {
            name: "Gerber Image",
            r#type: "data",
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
            language_id: 404627610u64,
            aliases: &["rs-274x"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["gerbv", "gerbview"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GettextCatalog {
    pub fn info() -> Language {
        Language {
            name: "Gettext Catalog",
            r#type: "prose",
            color: "",
            extensions: &[".po", ".pot"],
            tm_scope: "source.po",
            ace_mode: "text",
            language_id: 129u64,
            aliases: &["pot"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Gherkin {
    pub fn info() -> Language {
        Language {
            name: "Gherkin",
            r#type: "programming",
            color: "#5B2063",
            extensions: &[".feature", ".story"],
            tm_scope: "text.gherkin.feature",
            ace_mode: "text",
            language_id: 76u64,
            aliases: &["cucumber"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GitAttributes {
    pub fn info() -> Language {
        Language {
            name: "Git Attributes",
            r#type: "data",
            color: "#F44D27",
            extensions: &[],
            tm_scope: "source.gitattributes",
            ace_mode: "gitignore",
            language_id: 956324166u64,
            aliases: &["gitattributes"],
            codemirror_mode: Some("shell"),
            codemirror_mime_type: Some("text/x-sh"),
            wrap: None,
            filenames: &[".gitattributes"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GitConfig {
    pub fn info() -> Language {
        Language {
            name: "Git Config",
            r#type: "data",
            color: "#F44D27",
            extensions: &[".gitconfig"],
            tm_scope: "source.gitconfig",
            ace_mode: "ini",
            language_id: 807968997u64,
            aliases: &["gitconfig", "gitmodules"],
            codemirror_mode: Some("properties"),
            codemirror_mime_type: Some("text/x-properties"),
            wrap: None,
            filenames: &[".gitconfig", ".gitmodules"],
            group: Some("INI"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GitRevisionList {
    pub fn info() -> Language {
        Language {
            name: "Git Revision List",
            r#type: "data",
            color: "#F44D27",
            extensions: &[],
            tm_scope: "source.git-revlist",
            ace_mode: "text",
            language_id: 461881235u64,
            aliases: &["Git Blame Ignore Revs"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[".git-blame-ignore-revs"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Gleam {
    pub fn info() -> Language {
        Language {
            name: "Gleam",
            r#type: "programming",
            color: "#ffaff3",
            extensions: &[".gleam"],
            tm_scope: "source.gleam",
            ace_mode: "text",
            language_id: 1054258749u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GlimmerJS {
    pub fn info() -> Language {
        Language {
            name: "Glimmer JS",
            r#type: "programming",
            color: "#F5835F",
            extensions: &[".gjs"],
            tm_scope: "source.gjs",
            ace_mode: "javascript",
            language_id: 5523150u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: Some("JavaScript"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GlimmerTS {
    pub fn info() -> Language {
        Language {
            name: "Glimmer TS",
            r#type: "programming",
            color: "#3178c6",
            extensions: &[".gts"],
            tm_scope: "source.gts",
            ace_mode: "typescript",
            language_id: 95110458u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: Some("TypeScript"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Glyph {
    pub fn info() -> Language {
        Language {
            name: "Glyph",
            r#type: "programming",
            color: "#c1ac7f",
            extensions: &[".glf"],
            tm_scope: "source.tcl",
            ace_mode: "tcl",
            language_id: 130u64,
            aliases: &[],
            codemirror_mode: Some("tcl"),
            codemirror_mime_type: Some("text/x-tcl"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GlyphBitmapDistributionFormat {
    pub fn info() -> Language {
        Language {
            name: "Glyph Bitmap Distribution Format",
            r#type: "data",
            color: "",
            extensions: &[".bdf"],
            tm_scope: "source.bdf",
            ace_mode: "text",
            language_id: 997665271u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Gnuplot {
    pub fn info() -> Language {
        Language {
            name: "Gnuplot",
            r#type: "programming",
            color: "#f0a9f0",
            extensions: &[".gp", ".gnu", ".gnuplot", ".p", ".plot", ".plt"],
            tm_scope: "source.gnuplot",
            ace_mode: "text",
            language_id: 131u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["gnuplot"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Go {
    pub fn info() -> Language {
        Language {
            name: "Go",
            r#type: "programming",
            color: "#00ADD8",
            extensions: &[".go"],
            tm_scope: "source.go",
            ace_mode: "golang",
            language_id: 132u64,
            aliases: &["golang"],
            codemirror_mode: Some("go"),
            codemirror_mime_type: Some("text/x-go"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GoChecksums {
    pub fn info() -> Language {
        Language {
            name: "Go Checksums",
            r#type: "data",
            color: "#00ADD8",
            extensions: &[],
            tm_scope: "go.sum",
            ace_mode: "text",
            language_id: 1054391671u64,
            aliases: &["go.sum", "go sum", "go.work.sum", "go work sum"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["go.sum", "go.work.sum"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GoModule {
    pub fn info() -> Language {
        Language {
            name: "Go Module",
            r#type: "data",
            color: "#00ADD8",
            extensions: &[],
            tm_scope: "go.mod",
            ace_mode: "text",
            language_id: 947461016u64,
            aliases: &["go.mod", "go mod"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["go.mod"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GoWorkspace {
    pub fn info() -> Language {
        Language {
            name: "Go Workspace",
            r#type: "data",
            color: "#00ADD8",
            extensions: &[],
            tm_scope: "go.mod",
            ace_mode: "text",
            language_id: 934546256u64,
            aliases: &["go.work", "go work"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["go.work"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GodotResource {
    pub fn info() -> Language {
        Language {
            name: "Godot Resource",
            r#type: "data",
            color: "#355570",
            extensions: &[".gdnlib", ".gdns", ".tres", ".tscn"],
            tm_scope: "source.gdresource",
            ace_mode: "text",
            language_id: 738107771u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["project.godot"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Golo {
    pub fn info() -> Language {
        Language {
            name: "Golo",
            r#type: "programming",
            color: "#88562A",
            extensions: &[".golo"],
            tm_scope: "source.golo",
            ace_mode: "text",
            language_id: 133u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Gosu {
    pub fn info() -> Language {
        Language {
            name: "Gosu",
            r#type: "programming",
            color: "#82937f",
            extensions: &[".gs", ".gst", ".gsx", ".vark"],
            tm_scope: "source.gosu.2",
            ace_mode: "text",
            language_id: 134u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Grace {
    pub fn info() -> Language {
        Language {
            name: "Grace",
            r#type: "programming",
            color: "#615f8b",
            extensions: &[".grace"],
            tm_scope: "source.grace",
            ace_mode: "text",
            language_id: 135u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Gradle {
    pub fn info() -> Language {
        Language {
            name: "Gradle",
            r#type: "data",
            color: "#02303a",
            extensions: &[".gradle"],
            tm_scope: "source.groovy.gradle",
            ace_mode: "text",
            language_id: 136u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GradleKotlinDSL {
    pub fn info() -> Language {
        Language {
            name: "Gradle Kotlin DSL",
            r#type: "data",
            color: "#02303a",
            extensions: &[".gradle.kts"],
            tm_scope: "source.kotlin",
            ace_mode: "text",
            language_id: 432600901u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: Some("Gradle"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GrammaticalFramework {
    pub fn info() -> Language {
        Language {
            name: "Grammatical Framework",
            r#type: "programming",
            color: "#ff0000",
            extensions: &[".gf"],
            tm_scope: "source.gf",
            ace_mode: "haskell",
            language_id: 137u64,
            aliases: &["gf"],
            codemirror_mode: Some("haskell"),
            codemirror_mime_type: Some("text/x-haskell"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GraphModelingLanguage {
    pub fn info() -> Language {
        Language {
            name: "Graph Modeling Language",
            r#type: "data",
            color: "",
            extensions: &[".gml"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 138u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GraphQL {
    pub fn info() -> Language {
        Language {
            name: "GraphQL",
            r#type: "data",
            color: "#e10098",
            extensions: &[".graphql", ".gql", ".graphqls"],
            tm_scope: "source.graphql",
            ace_mode: "text",
            language_id: 139u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GraphvizDOT {
    pub fn info() -> Language {
        Language {
            name: "Graphviz (DOT)",
            r#type: "data",
            color: "#2596be",
            extensions: &[".dot", ".gv"],
            tm_scope: "source.dot",
            ace_mode: "text",
            language_id: 140u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Groovy {
    pub fn info() -> Language {
        Language {
            name: "Groovy",
            r#type: "programming",
            color: "#4298b8",
            extensions: &[".groovy", ".grt", ".gtpl", ".gvy"],
            tm_scope: "source.groovy",
            ace_mode: "groovy",
            language_id: 142u64,
            aliases: &[],
            codemirror_mode: Some("groovy"),
            codemirror_mime_type: Some("text/x-groovy"),
            wrap: None,
            filenames: &["Jenkinsfile"],
            group: None,
            interpreters: &["groovy"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl GroovyServerPages {
    pub fn info() -> Language {
        Language {
            name: "Groovy Server Pages",
            r#type: "programming",
            color: "#4298b8",
            extensions: &[".gsp"],
            tm_scope: "text.html.jsp",
            ace_mode: "jsp",
            language_id: 143u64,
            aliases: &["gsp", "java server page"],
            codemirror_mode: Some("htmlembedded"),
            codemirror_mime_type: Some("application/x-jsp"),
            wrap: None,
            filenames: &[],
            group: Some("Groovy"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl HAProxy {
    pub fn info() -> Language {
        Language {
            name: "HAProxy",
            r#type: "data",
            color: "#106da9",
            extensions: &[".cfg"],
            tm_scope: "source.haproxy-config",
            ace_mode: "text",
            language_id: 366607477u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["haproxy.cfg"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl HCL {
    pub fn info() -> Language {
        Language {
            name: "HCL",
            r#type: "programming",
            color: "#844FBA",
            extensions: &[".hcl", ".nomad", ".tf", ".tfvars", ".workflow"],
            tm_scope: "source.hcl",
            ace_mode: "ruby",
            language_id: 144u64,
            aliases: &["HashiCorp Configuration Language", "terraform"],
            codemirror_mode: Some("ruby"),
            codemirror_mime_type: Some("text/x-ruby"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl HLSL {
    pub fn info() -> Language {
        Language {
            name: "HLSL",
            r#type: "programming",
            color: "#aace60",
            extensions: &[".hlsl", ".cginc", ".fx", ".fxh", ".hlsli"],
            tm_scope: "source.hlsl",
            ace_mode: "text",
            language_id: 145u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl HOCON {
    pub fn info() -> Language {
        Language {
            name: "HOCON",
            r#type: "data",
            color: "#9ff8ee",
            extensions: &[".hocon"],
            tm_scope: "source.hocon",
            ace_mode: "text",
            language_id: 679725279u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[".scalafix.conf", ".scalafmt.conf"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl HTML {
    pub fn info() -> Language {
        Language {
            name: "HTML",
            r#type: "markup",
            color: "#e34c26",
            extensions: &[".html", ".hta", ".htm", ".html.hl", ".inc", ".xht", ".xhtml"],
            tm_scope: "text.html.basic",
            ace_mode: "html",
            language_id: 146u64,
            aliases: &["xhtml"],
            codemirror_mode: Some("htmlmixed"),
            codemirror_mime_type: Some("text/html"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl HTMLpECR {
    pub fn info() -> Language {
        Language {
            name: "HTML+ECR",
            r#type: "markup",
            color: "#2e1052",
            extensions: &[".ecr"],
            tm_scope: "text.html.ecr",
            ace_mode: "text",
            language_id: 148u64,
            aliases: &["ecr"],
            codemirror_mode: Some("htmlmixed"),
            codemirror_mime_type: Some("text/html"),
            wrap: None,
            filenames: &[],
            group: Some("HTML"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl HTMLpEEX {
    pub fn info() -> Language {
        Language {
            name: "HTML+EEX",
            r#type: "markup",
            color: "#6e4a7e",
            extensions: &[".html.eex", ".heex", ".leex"],
            tm_scope: "text.html.elixir",
            ace_mode: "text",
            language_id: 149u64,
            aliases: &["eex", "heex", "leex"],
            codemirror_mode: Some("htmlmixed"),
            codemirror_mime_type: Some("text/html"),
            wrap: None,
            filenames: &[],
            group: Some("HTML"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl HTMLpERB {
    pub fn info() -> Language {
        Language {
            name: "HTML+ERB",
            r#type: "markup",
            color: "#701516",
            extensions: &[".erb", ".erb.deface", ".rhtml"],
            tm_scope: "text.html.erb",
            ace_mode: "text",
            language_id: 150u64,
            aliases: &["erb", "rhtml", "html+ruby"],
            codemirror_mode: Some("htmlembedded"),
            codemirror_mime_type: Some("application/x-erb"),
            wrap: None,
            filenames: &[],
            group: Some("HTML"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl HTMLpPHP {
    pub fn info() -> Language {
        Language {
            name: "HTML+PHP",
            r#type: "markup",
            color: "#4f5d95",
            extensions: &[".phtml"],
            tm_scope: "text.html.php",
            ace_mode: "php",
            language_id: 151u64,
            aliases: &[],
            codemirror_mode: Some("php"),
            codemirror_mime_type: Some("application/x-httpd-php"),
            wrap: None,
            filenames: &[],
            group: Some("HTML"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl HTMLpRazor {
    pub fn info() -> Language {
        Language {
            name: "HTML+Razor",
            r#type: "markup",
            color: "#512be4",
            extensions: &[".cshtml", ".razor"],
            tm_scope: "text.html.cshtml",
            ace_mode: "razor",
            language_id: 479039817u64,
            aliases: &["razor"],
            codemirror_mode: Some("htmlmixed"),
            codemirror_mime_type: Some("text/html"),
            wrap: None,
            filenames: &[],
            group: Some("HTML"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl HTTP {
    pub fn info() -> Language {
        Language {
            name: "HTTP",
            r#type: "data",
            color: "#005C9C",
            extensions: &[".http"],
            tm_scope: "source.httpspec",
            ace_mode: "text",
            language_id: 152u64,
            aliases: &[],
            codemirror_mode: Some("http"),
            codemirror_mime_type: Some("message/http"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl HXML {
    pub fn info() -> Language {
        Language {
            name: "HXML",
            r#type: "data",
            color: "#f68712",
            extensions: &[".hxml"],
            tm_scope: "source.hxml",
            ace_mode: "text",
            language_id: 786683730u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Hack {
    pub fn info() -> Language {
        Language {
            name: "Hack",
            r#type: "programming",
            color: "#878787",
            extensions: &[".hack", ".hh", ".hhi", ".php"],
            tm_scope: "source.hack",
            ace_mode: "php",
            language_id: 153u64,
            aliases: &[],
            codemirror_mode: Some("php"),
            codemirror_mime_type: Some("application/x-httpd-php"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Haml {
    pub fn info() -> Language {
        Language {
            name: "Haml",
            r#type: "markup",
            color: "#ece2a9",
            extensions: &[".haml", ".haml.deface"],
            tm_scope: "text.haml",
            ace_mode: "haml",
            language_id: 154u64,
            aliases: &[],
            codemirror_mode: Some("haml"),
            codemirror_mime_type: Some("text/x-haml"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Handlebars {
    pub fn info() -> Language {
        Language {
            name: "Handlebars",
            r#type: "markup",
            color: "#f7931e",
            extensions: &[".handlebars", ".hbs"],
            tm_scope: "text.html.handlebars",
            ace_mode: "handlebars",
            language_id: 155u64,
            aliases: &["hbs", "htmlbars"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Harbour {
    pub fn info() -> Language {
        Language {
            name: "Harbour",
            r#type: "programming",
            color: "#0e60e3",
            extensions: &[".hb"],
            tm_scope: "source.harbour",
            ace_mode: "text",
            language_id: 156u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Hare {
    pub fn info() -> Language {
        Language {
            name: "Hare",
            r#type: "programming",
            color: "#9d7424",
            extensions: &[".ha"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 463518941u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Haskell {
    pub fn info() -> Language {
        Language {
            name: "Haskell",
            r#type: "programming",
            color: "#5e5086",
            extensions: &[".hs", ".hs-boot", ".hsc"],
            tm_scope: "source.haskell",
            ace_mode: "haskell",
            language_id: 157u64,
            aliases: &[],
            codemirror_mode: Some("haskell"),
            codemirror_mime_type: Some("text/x-haskell"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["runghc", "runhaskell", "runhugs"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Haxe {
    pub fn info() -> Language {
        Language {
            name: "Haxe",
            r#type: "programming",
            color: "#df7900",
            extensions: &[".hx", ".hxsl"],
            tm_scope: "source.hx",
            ace_mode: "haxe",
            language_id: 158u64,
            aliases: &[],
            codemirror_mode: Some("haxe"),
            codemirror_mime_type: Some("text/x-haxe"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl HiveQL {
    pub fn info() -> Language {
        Language {
            name: "HiveQL",
            r#type: "programming",
            color: "#dce200",
            extensions: &[".q", ".hql"],
            tm_scope: "source.hql",
            ace_mode: "sql",
            language_id: 931814087u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl HolyC {
    pub fn info() -> Language {
        Language {
            name: "HolyC",
            r#type: "programming",
            color: "#ffefaf",
            extensions: &[".hc"],
            tm_scope: "source.hc",
            ace_mode: "c_cpp",
            language_id: 928121743u64,
            aliases: &[],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-csrc"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl HostsFile {
    pub fn info() -> Language {
        Language {
            name: "Hosts File",
            r#type: "data",
            color: "#308888",
            extensions: &[],
            tm_scope: "source.hosts",
            ace_mode: "text",
            language_id: 231021894u64,
            aliases: &["hosts"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["HOSTS", "hosts", "hosts.txt"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Hy {
    pub fn info() -> Language {
        Language {
            name: "Hy",
            r#type: "programming",
            color: "#7790B2",
            extensions: &[".hy"],
            tm_scope: "source.hy",
            ace_mode: "text",
            language_id: 159u64,
            aliases: &["hylang"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["hy"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl HyPhy {
    pub fn info() -> Language {
        Language {
            name: "HyPhy",
            r#type: "programming",
            color: "",
            extensions: &[".bf"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 160u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl IDL {
    pub fn info() -> Language {
        Language {
            name: "IDL",
            r#type: "programming",
            color: "#a3522f",
            extensions: &[".pro", ".dlm"],
            tm_scope: "source.idl",
            ace_mode: "text",
            language_id: 161u64,
            aliases: &[],
            codemirror_mode: Some("idl"),
            codemirror_mime_type: Some("text/x-idl"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl IGORPro {
    pub fn info() -> Language {
        Language {
            name: "IGOR Pro",
            r#type: "programming",
            color: "#0000cc",
            extensions: &[".ipf"],
            tm_scope: "source.igor",
            ace_mode: "text",
            language_id: 162u64,
            aliases: &["igor", "igorpro"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl INI {
    pub fn info() -> Language {
        Language {
            name: "INI",
            r#type: "data",
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
            language_id: 163u64,
            aliases: &["dosini"],
            codemirror_mode: Some("properties"),
            codemirror_mime_type: Some("text/x-properties"),
            wrap: None,
            filenames: &[
                ".buckconfig",
                ".coveragerc",
                ".flake8",
                ".pylintrc",
                "HOSTS",
                "buildozer.spec",
                "hosts",
                "pylintrc",
                "vlcrc",
            ],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl IRCLog {
    pub fn info() -> Language {
        Language {
            name: "IRC log",
            r#type: "data",
            color: "",
            extensions: &[".irclog", ".weechatlog"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 164u64,
            aliases: &["irc", "irc logs"],
            codemirror_mode: Some("mirc"),
            codemirror_mime_type: Some("text/mirc"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Idris {
    pub fn info() -> Language {
        Language {
            name: "Idris",
            r#type: "programming",
            color: "#b30000",
            extensions: &[".idr", ".lidr"],
            tm_scope: "source.idris",
            ace_mode: "text",
            language_id: 165u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl IgnoreList {
    pub fn info() -> Language {
        Language {
            name: "Ignore List",
            r#type: "data",
            color: "#000000",
            extensions: &[".gitignore"],
            tm_scope: "source.gitignore",
            ace_mode: "gitignore",
            language_id: 74444240u64,
            aliases: &["ignore", "gitignore", "git-ignore"],
            codemirror_mode: Some("shell"),
            codemirror_mime_type: Some("text/x-sh"),
            wrap: None,
            filenames: &[
                ".atomignore",
                ".babelignore",
                ".bzrignore",
                ".coffeelintignore",
                ".cvsignore",
                ".dockerignore",
                ".easignore",
                ".eleventyignore",
                ".eslintignore",
                ".gitignore",
                ".ignore",
                ".markdownlintignore",
                ".nodemonignore",
                ".npmignore",
                ".prettierignore",
                ".stylelintignore",
                ".vercelignore",
                ".vscodeignore",
                "gitignore-global",
                "gitignore_global",
            ],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ImageJMacro {
    pub fn info() -> Language {
        Language {
            name: "ImageJ Macro",
            r#type: "programming",
            color: "#99AAFF",
            extensions: &[".ijm"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 575143428u64,
            aliases: &["ijm"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Imba {
    pub fn info() -> Language {
        Language {
            name: "Imba",
            r#type: "programming",
            color: "#16cec6",
            extensions: &[".imba"],
            tm_scope: "source.imba",
            ace_mode: "text",
            language_id: 1057618448u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Inform7 {
    pub fn info() -> Language {
        Language {
            name: "Inform 7",
            r#type: "programming",
            color: "",
            extensions: &[".ni", ".i7x"],
            tm_scope: "source.inform7",
            ace_mode: "text",
            language_id: 166u64,
            aliases: &["i7", "inform7"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: Some(true),
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Ink {
    pub fn info() -> Language {
        Language {
            name: "Ink",
            r#type: "programming",
            color: "",
            extensions: &[".ink"],
            tm_scope: "source.ink",
            ace_mode: "text",
            language_id: 838252715u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: Some(true),
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl InnoSetup {
    pub fn info() -> Language {
        Language {
            name: "Inno Setup",
            r#type: "programming",
            color: "#264b99",
            extensions: &[".iss", ".isl"],
            tm_scope: "source.inno",
            ace_mode: "text",
            language_id: 167u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Io {
    pub fn info() -> Language {
        Language {
            name: "Io",
            r#type: "programming",
            color: "#a9188d",
            extensions: &[".io"],
            tm_scope: "source.io",
            ace_mode: "io",
            language_id: 168u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["io"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Ioke {
    pub fn info() -> Language {
        Language {
            name: "Ioke",
            r#type: "programming",
            color: "#078193",
            extensions: &[".ik"],
            tm_scope: "source.ioke",
            ace_mode: "text",
            language_id: 169u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["ioke"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Isabelle {
    pub fn info() -> Language {
        Language {
            name: "Isabelle",
            r#type: "programming",
            color: "#FEFE00",
            extensions: &[".thy"],
            tm_scope: "source.isabelle.theory",
            ace_mode: "text",
            language_id: 170u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl IsabelleROOT {
    pub fn info() -> Language {
        Language {
            name: "Isabelle ROOT",
            r#type: "programming",
            color: "#FEFE00",
            extensions: &[],
            tm_scope: "source.isabelle.root",
            ace_mode: "text",
            language_id: 171u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["ROOT"],
            group: Some("Isabelle"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl J {
    pub fn info() -> Language {
        Language {
            name: "J",
            r#type: "programming",
            color: "#9EEDFF",
            extensions: &[".ijs"],
            tm_scope: "source.j",
            ace_mode: "text",
            language_id: 172u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["jconsole"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl JARManifest {
    pub fn info() -> Language {
        Language {
            name: "JAR Manifest",
            r#type: "data",
            color: "#b07219",
            extensions: &[],
            tm_scope: "source.yaml",
            ace_mode: "text",
            language_id: 447261135u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["MANIFEST.MF"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl JCL {
    pub fn info() -> Language {
        Language {
            name: "JCL",
            r#type: "programming",
            color: "#d90e09",
            extensions: &[".jcl"],
            tm_scope: "source.jcl",
            ace_mode: "text",
            language_id: 316620079u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl JFlex {
    pub fn info() -> Language {
        Language {
            name: "JFlex",
            r#type: "programming",
            color: "#DBCA00",
            extensions: &[".flex", ".jflex"],
            tm_scope: "source.jflex",
            ace_mode: "text",
            language_id: 173u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: Some("Lex"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl JSON {
    pub fn info() -> Language {
        Language {
            name: "JSON",
            r#type: "data",
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
                ".json.example",
                ".jsonl",
                ".mcmeta",
                ".sarif",
                ".tact",
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
            language_id: 174u64,
            aliases: &["geojson", "jsonl", "sarif", "topojson"],
            codemirror_mode: Some("javascript"),
            codemirror_mime_type: Some("application/json"),
            wrap: None,
            filenames: &[
                ".all-contributorsrc",
                ".arcconfig",
                ".auto-changelog",
                ".c8rc",
                ".htmlhintrc",
                ".imgbotconfig",
                ".nycrc",
                ".tern-config",
                ".tern-project",
                ".watchmanconfig",
                "MODULE.bazel.lock",
                "Pipfile.lock",
                "composer.lock",
                "deno.lock",
                "flake.lock",
                "mcmod.info",
            ],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl JSONWithComments {
    pub fn info() -> Language {
        Language {
            name: "JSON with Comments",
            r#type: "data",
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
            language_id: 423u64,
            aliases: &["jsonc"],
            codemirror_mode: Some("javascript"),
            codemirror_mime_type: Some("text/javascript"),
            wrap: None,
            filenames: &[
                ".babelrc",
                ".devcontainer.json",
                ".eslintrc.json",
                ".jscsrc",
                ".jshintrc",
                ".jslintrc",
                ".swcrc",
                "api-extractor.json",
                "devcontainer.json",
                "jsconfig.json",
                "language-configuration.json",
                "tsconfig.json",
                "tslint.json",
            ],
            group: Some("JSON"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl JSON5 {
    pub fn info() -> Language {
        Language {
            name: "JSON5",
            r#type: "data",
            color: "#267CB9",
            extensions: &[".json5"],
            tm_scope: "source.js",
            ace_mode: "javascript",
            language_id: 175u64,
            aliases: &[],
            codemirror_mode: Some("javascript"),
            codemirror_mime_type: Some("application/json"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl JSONLD {
    pub fn info() -> Language {
        Language {
            name: "JSONLD",
            r#type: "data",
            color: "#0c479c",
            extensions: &[".jsonld"],
            tm_scope: "source.js",
            ace_mode: "javascript",
            language_id: 176u64,
            aliases: &[],
            codemirror_mode: Some("javascript"),
            codemirror_mime_type: Some("application/json"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl JSONiq {
    pub fn info() -> Language {
        Language {
            name: "JSONiq",
            r#type: "programming",
            color: "#40d47e",
            extensions: &[".jq"],
            tm_scope: "source.jsoniq",
            ace_mode: "jsoniq",
            language_id: 177u64,
            aliases: &[],
            codemirror_mode: Some("javascript"),
            codemirror_mime_type: Some("application/json"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Jai {
    pub fn info() -> Language {
        Language {
            name: "Jai",
            r#type: "programming",
            color: "#ab8b4b",
            extensions: &[".jai"],
            tm_scope: "source.jai",
            ace_mode: "text",
            language_id: 70127133u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Janet {
    pub fn info() -> Language {
        Language {
            name: "Janet",
            r#type: "programming",
            color: "#0886a5",
            extensions: &[".janet"],
            tm_scope: "source.janet",
            ace_mode: "scheme",
            language_id: 1028705371u64,
            aliases: &[],
            codemirror_mode: Some("scheme"),
            codemirror_mime_type: Some("text/x-scheme"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["janet"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Jasmin {
    pub fn info() -> Language {
        Language {
            name: "Jasmin",
            r#type: "programming",
            color: "#d03600",
            extensions: &[".j"],
            tm_scope: "source.jasmin",
            ace_mode: "java",
            language_id: 180u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Java {
    pub fn info() -> Language {
        Language {
            name: "Java",
            r#type: "programming",
            color: "#b07219",
            extensions: &[".java", ".jav", ".jsh"],
            tm_scope: "source.java",
            ace_mode: "java",
            language_id: 181u64,
            aliases: &[],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-java"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl JavaProperties {
    pub fn info() -> Language {
        Language {
            name: "Java Properties",
            r#type: "data",
            color: "#2A6277",
            extensions: &[".properties"],
            tm_scope: "source.java-properties",
            ace_mode: "properties",
            language_id: 519377561u64,
            aliases: &[],
            codemirror_mode: Some("properties"),
            codemirror_mime_type: Some("text/x-properties"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl JavaServerPages {
    pub fn info() -> Language {
        Language {
            name: "Java Server Pages",
            r#type: "programming",
            color: "#2A6277",
            extensions: &[".jsp", ".tag"],
            tm_scope: "text.html.jsp",
            ace_mode: "jsp",
            language_id: 182u64,
            aliases: &["jsp"],
            codemirror_mode: Some("htmlembedded"),
            codemirror_mime_type: Some("application/x-jsp"),
            wrap: None,
            filenames: &[],
            group: Some("Java"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl JavaTemplateEngine {
    pub fn info() -> Language {
        Language {
            name: "Java Template Engine",
            r#type: "programming",
            color: "#2A6277",
            extensions: &[".jte"],
            tm_scope: "text.html.jte",
            ace_mode: "text",
            language_id: 599494012u64,
            aliases: &["jte"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: Some("Java"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl JavaScript {
    pub fn info() -> Language {
        Language {
            name: "JavaScript",
            r#type: "programming",
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
            language_id: 183u64,
            aliases: &["js", "node"],
            codemirror_mode: Some("javascript"),
            codemirror_mime_type: Some("text/javascript"),
            wrap: None,
            filenames: &["Jakefile"],
            group: None,
            interpreters: &[
                "chakra",
                "d8",
                "gjs",
                "js",
                "node",
                "nodejs",
                "qjs",
                "rhino",
                "v8",
                "v8-shell",
            ],
            fs_name: None,
            searchable: None,
        }
    }
}
impl JavaScriptpERB {
    pub fn info() -> Language {
        Language {
            name: "JavaScript+ERB",
            r#type: "programming",
            color: "#f1e05a",
            extensions: &[".js.erb"],
            tm_scope: "source.js",
            ace_mode: "javascript",
            language_id: 914318960u64,
            aliases: &[],
            codemirror_mode: Some("javascript"),
            codemirror_mime_type: Some("application/javascript"),
            wrap: None,
            filenames: &[],
            group: Some("JavaScript"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl JestSnapshot {
    pub fn info() -> Language {
        Language {
            name: "Jest Snapshot",
            r#type: "data",
            color: "#15c213",
            extensions: &[".snap"],
            tm_scope: "source.jest.snap",
            ace_mode: "javascript",
            language_id: 774635084u64,
            aliases: &[],
            codemirror_mode: Some("javascript"),
            codemirror_mime_type: Some("application/javascript"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl JetBrainsMPS {
    pub fn info() -> Language {
        Language {
            name: "JetBrains MPS",
            r#type: "programming",
            color: "#21D789",
            extensions: &[".mps", ".mpl", ".msd"],
            tm_scope: "none",
            ace_mode: "xml",
            language_id: 465165328u64,
            aliases: &["mps"],
            codemirror_mode: Some("xml"),
            codemirror_mime_type: Some("text/xml"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Jinja {
    pub fn info() -> Language {
        Language {
            name: "Jinja",
            r#type: "markup",
            color: "#a52a22",
            extensions: &[".jinja", ".j2", ".jinja2"],
            tm_scope: "text.html.django",
            ace_mode: "django",
            language_id: 147u64,
            aliases: &["django", "html+django", "html+jinja", "htmldjango"],
            codemirror_mode: Some("django"),
            codemirror_mime_type: Some("text/x-django"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Jison {
    pub fn info() -> Language {
        Language {
            name: "Jison",
            r#type: "programming",
            color: "#56b3cb",
            extensions: &[".jison"],
            tm_scope: "source.jison",
            ace_mode: "text",
            language_id: 284531423u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: Some("Yacc"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl JisonLex {
    pub fn info() -> Language {
        Language {
            name: "Jison Lex",
            r#type: "programming",
            color: "#56b3cb",
            extensions: &[".jisonlex"],
            tm_scope: "source.jisonlex",
            ace_mode: "text",
            language_id: 406395330u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: Some("Lex"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Jolie {
    pub fn info() -> Language {
        Language {
            name: "Jolie",
            r#type: "programming",
            color: "#843179",
            extensions: &[".ol", ".iol"],
            tm_scope: "source.jolie",
            ace_mode: "text",
            language_id: 998078858u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["jolie"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Jsonnet {
    pub fn info() -> Language {
        Language {
            name: "Jsonnet",
            r#type: "programming",
            color: "#0064bd",
            extensions: &[".jsonnet", ".libsonnet"],
            tm_scope: "source.jsonnet",
            ace_mode: "text",
            language_id: 664885656u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Julia {
    pub fn info() -> Language {
        Language {
            name: "Julia",
            r#type: "programming",
            color: "#a270ba",
            extensions: &[".jl"],
            tm_scope: "source.julia",
            ace_mode: "julia",
            language_id: 184u64,
            aliases: &[],
            codemirror_mode: Some("julia"),
            codemirror_mime_type: Some("text/x-julia"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["julia"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl JuliaREPL {
    pub fn info() -> Language {
        Language {
            name: "Julia REPL",
            r#type: "programming",
            color: "#a270ba",
            extensions: &[],
            tm_scope: "source.julia.console",
            ace_mode: "text",
            language_id: 220689142u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: Some("Julia"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl JupyterNotebook {
    pub fn info() -> Language {
        Language {
            name: "Jupyter Notebook",
            r#type: "markup",
            color: "#DA5B0B",
            extensions: &[".ipynb"],
            tm_scope: "source.json",
            ace_mode: "json",
            language_id: 185u64,
            aliases: &["IPython Notebook"],
            codemirror_mode: Some("javascript"),
            codemirror_mime_type: Some("application/json"),
            wrap: None,
            filenames: &["Notebook"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Just {
    pub fn info() -> Language {
        Language {
            name: "Just",
            r#type: "programming",
            color: "#384d54",
            extensions: &[".just"],
            tm_scope: "source.just",
            ace_mode: "text",
            language_id: 128447695u64,
            aliases: &["Justfile"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[
                ".JUSTFILE",
                ".Justfile",
                ".justfile",
                "JUSTFILE",
                "Justfile",
                "justfile",
            ],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl KDL {
    pub fn info() -> Language {
        Language {
            name: "KDL",
            r#type: "data",
            color: "#ffb3b3",
            extensions: &[".kdl"],
            tm_scope: "source.kdl",
            ace_mode: "tcl",
            language_id: 931123626u64,
            aliases: &[],
            codemirror_mode: Some("yacas"),
            codemirror_mime_type: Some("text/x-yacas"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl KRL {
    pub fn info() -> Language {
        Language {
            name: "KRL",
            r#type: "programming",
            color: "#28430A",
            extensions: &[".krl"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 186u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl KaitaiStruct {
    pub fn info() -> Language {
        Language {
            name: "Kaitai Struct",
            r#type: "programming",
            color: "#773b37",
            extensions: &[".ksy"],
            tm_scope: "source.yaml",
            ace_mode: "yaml",
            language_id: 818804755u64,
            aliases: &["ksy"],
            codemirror_mode: Some("yaml"),
            codemirror_mime_type: Some("text/x-yaml"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl KakouneScript {
    pub fn info() -> Language {
        Language {
            name: "KakouneScript",
            r#type: "programming",
            color: "#6f8042",
            extensions: &[".kak"],
            tm_scope: "source.kakscript",
            ace_mode: "text",
            language_id: 603336474u64,
            aliases: &["kak", "kakscript"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["kakrc"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl KerboScript {
    pub fn info() -> Language {
        Language {
            name: "KerboScript",
            r#type: "programming",
            color: "#41adf0",
            extensions: &[".ks"],
            tm_scope: "source.kerboscript",
            ace_mode: "text",
            language_id: 59716426u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl KiCadLayout {
    pub fn info() -> Language {
        Language {
            name: "KiCad Layout",
            r#type: "data",
            color: "#2f4aab",
            extensions: &[".kicad_pcb", ".kicad_mod", ".kicad_wks"],
            tm_scope: "source.pcb.sexp",
            ace_mode: "lisp",
            language_id: 187u64,
            aliases: &["pcbnew"],
            codemirror_mode: Some("commonlisp"),
            codemirror_mime_type: Some("text/x-common-lisp"),
            wrap: None,
            filenames: &["fp-lib-table"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl KiCadLegacyLayout {
    pub fn info() -> Language {
        Language {
            name: "KiCad Legacy Layout",
            r#type: "data",
            color: "#2f4aab",
            extensions: &[".brd"],
            tm_scope: "source.pcb.board",
            ace_mode: "text",
            language_id: 140848857u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl KiCadSchematic {
    pub fn info() -> Language {
        Language {
            name: "KiCad Schematic",
            r#type: "data",
            color: "#2f4aab",
            extensions: &[".kicad_sch", ".sch"],
            tm_scope: "source.pcb.schematic",
            ace_mode: "text",
            language_id: 622447435u64,
            aliases: &["eeschema schematic"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Kickstart {
    pub fn info() -> Language {
        Language {
            name: "Kickstart",
            r#type: "data",
            color: "",
            extensions: &[".ks"],
            tm_scope: "source.kickstart",
            ace_mode: "text",
            language_id: 692635484u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Kit {
    pub fn info() -> Language {
        Language {
            name: "Kit",
            r#type: "markup",
            color: "",
            extensions: &[".kit"],
            tm_scope: "text.html.basic",
            ace_mode: "html",
            language_id: 188u64,
            aliases: &[],
            codemirror_mode: Some("htmlmixed"),
            codemirror_mime_type: Some("text/html"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Kotlin {
    pub fn info() -> Language {
        Language {
            name: "Kotlin",
            r#type: "programming",
            color: "#A97BFF",
            extensions: &[".kt", ".ktm", ".kts"],
            tm_scope: "source.kotlin",
            ace_mode: "text",
            language_id: 189u64,
            aliases: &[],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-kotlin"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Kusto {
    pub fn info() -> Language {
        Language {
            name: "Kusto",
            r#type: "data",
            color: "",
            extensions: &[".csl", ".kql"],
            tm_scope: "source.kusto",
            ace_mode: "text",
            language_id: 225697190u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl LFE {
    pub fn info() -> Language {
        Language {
            name: "LFE",
            r#type: "programming",
            color: "#4C3023",
            extensions: &[".lfe"],
            tm_scope: "source.lisp",
            ace_mode: "lisp",
            language_id: 190u64,
            aliases: &[],
            codemirror_mode: Some("commonlisp"),
            codemirror_mime_type: Some("text/x-common-lisp"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl LLVM {
    pub fn info() -> Language {
        Language {
            name: "LLVM",
            r#type: "programming",
            color: "#185619",
            extensions: &[".ll"],
            tm_scope: "source.llvm",
            ace_mode: "text",
            language_id: 191u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl LOLCODE {
    pub fn info() -> Language {
        Language {
            name: "LOLCODE",
            r#type: "programming",
            color: "#cc9900",
            extensions: &[".lol"],
            tm_scope: "source.lolcode",
            ace_mode: "text",
            language_id: 192u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl LSL {
    pub fn info() -> Language {
        Language {
            name: "LSL",
            r#type: "programming",
            color: "#3d9970",
            extensions: &[".lsl", ".lslp"],
            tm_scope: "source.lsl",
            ace_mode: "lsl",
            language_id: 193u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["lsl"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl LTspiceSymbol {
    pub fn info() -> Language {
        Language {
            name: "LTspice Symbol",
            r#type: "data",
            color: "",
            extensions: &[".asy"],
            tm_scope: "source.ltspice.symbol",
            ace_mode: "text",
            language_id: 1013566805u64,
            aliases: &[],
            codemirror_mode: Some("spreadsheet"),
            codemirror_mime_type: Some("text/x-spreadsheet"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl LabVIEW {
    pub fn info() -> Language {
        Language {
            name: "LabVIEW",
            r#type: "programming",
            color: "#fede06",
            extensions: &[".lvproj", ".lvclass", ".lvlib"],
            tm_scope: "text.xml",
            ace_mode: "xml",
            language_id: 194u64,
            aliases: &[],
            codemirror_mode: Some("xml"),
            codemirror_mime_type: Some("text/xml"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Lark {
    pub fn info() -> Language {
        Language {
            name: "Lark",
            r#type: "data",
            color: "#2980B9",
            extensions: &[".lark"],
            tm_scope: "source.lark",
            ace_mode: "text",
            language_id: 758480799u64,
            aliases: &[],
            codemirror_mode: Some("ebnf"),
            codemirror_mime_type: Some("text/x-ebnf"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Lasso {
    pub fn info() -> Language {
        Language {
            name: "Lasso",
            r#type: "programming",
            color: "#999999",
            extensions: &[".lasso", ".las", ".lasso8", ".lasso9"],
            tm_scope: "file.lasso",
            ace_mode: "text",
            language_id: 195u64,
            aliases: &["lassoscript"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Latte {
    pub fn info() -> Language {
        Language {
            name: "Latte",
            r#type: "markup",
            color: "#f2a542",
            extensions: &[".latte"],
            tm_scope: "text.html.smarty",
            ace_mode: "smarty",
            language_id: 196u64,
            aliases: &[],
            codemirror_mode: Some("smarty"),
            codemirror_mime_type: Some("text/x-smarty"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Lean {
    pub fn info() -> Language {
        Language {
            name: "Lean",
            r#type: "programming",
            color: "",
            extensions: &[".lean", ".hlean"],
            tm_scope: "source.lean",
            ace_mode: "text",
            language_id: 197u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Lean4 {
    pub fn info() -> Language {
        Language {
            name: "Lean 4",
            r#type: "programming",
            color: "",
            extensions: &[".lean"],
            tm_scope: "source.lean4",
            ace_mode: "text",
            language_id: 455147478u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: Some("Lean"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Less {
    pub fn info() -> Language {
        Language {
            name: "Less",
            r#type: "markup",
            color: "#1d365d",
            extensions: &[".less"],
            tm_scope: "source.css.less",
            ace_mode: "less",
            language_id: 198u64,
            aliases: &["less-css"],
            codemirror_mode: Some("css"),
            codemirror_mime_type: Some("text/css"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Lex {
    pub fn info() -> Language {
        Language {
            name: "Lex",
            r#type: "programming",
            color: "#DBCA00",
            extensions: &[".l", ".lex"],
            tm_scope: "source.lex",
            ace_mode: "text",
            language_id: 199u64,
            aliases: &["flex"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["Lexer.x", "lexer.x"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl LigoLANG {
    pub fn info() -> Language {
        Language {
            name: "LigoLANG",
            r#type: "programming",
            color: "#0e74ff",
            extensions: &[".ligo"],
            tm_scope: "source.ligo",
            ace_mode: "pascal",
            language_id: 1040646257u64,
            aliases: &[],
            codemirror_mode: Some("pascal"),
            codemirror_mime_type: Some("text/x-pascal"),
            wrap: None,
            filenames: &[],
            group: Some("LigoLANG"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl LilyPond {
    pub fn info() -> Language {
        Language {
            name: "LilyPond",
            r#type: "programming",
            color: "#9ccc7c",
            extensions: &[".ly", ".ily"],
            tm_scope: "source.lilypond",
            ace_mode: "text",
            language_id: 200u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Limbo {
    pub fn info() -> Language {
        Language {
            name: "Limbo",
            r#type: "programming",
            color: "",
            extensions: &[".b", ".m"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 201u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl LinearProgramming {
    pub fn info() -> Language {
        Language {
            name: "Linear Programming",
            r#type: "programming",
            color: "",
            extensions: &[".lp"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 377204539u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl LinkerScript {
    pub fn info() -> Language {
        Language {
            name: "Linker Script",
            r#type: "data",
            color: "",
            extensions: &[".ld", ".lds", ".x"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 202u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["ld.script"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl LinuxKernelModule {
    pub fn info() -> Language {
        Language {
            name: "Linux Kernel Module",
            r#type: "data",
            color: "",
            extensions: &[".mod"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 203u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Liquid {
    pub fn info() -> Language {
        Language {
            name: "Liquid",
            r#type: "markup",
            color: "#67b8de",
            extensions: &[".liquid"],
            tm_scope: "text.html.liquid",
            ace_mode: "liquid",
            language_id: 204u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl LiterateAgda {
    pub fn info() -> Language {
        Language {
            name: "Literate Agda",
            r#type: "programming",
            color: "#315665",
            extensions: &[".lagda"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 205u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: Some("Agda"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl LiterateCoffeeScript {
    pub fn info() -> Language {
        Language {
            name: "Literate CoffeeScript",
            r#type: "programming",
            color: "#244776",
            extensions: &[".litcoffee", ".coffee.md"],
            tm_scope: "source.litcoffee",
            ace_mode: "text",
            language_id: 206u64,
            aliases: &["litcoffee"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: Some(true),
            filenames: &[],
            group: Some("CoffeeScript"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl LiterateHaskell {
    pub fn info() -> Language {
        Language {
            name: "Literate Haskell",
            r#type: "programming",
            color: "#5e5086",
            extensions: &[".lhs"],
            tm_scope: "text.tex.latex.haskell",
            ace_mode: "text",
            language_id: 207u64,
            aliases: &["lhaskell", "lhs"],
            codemirror_mode: Some("haskell-literate"),
            codemirror_mime_type: Some("text/x-literate-haskell"),
            wrap: None,
            filenames: &[],
            group: Some("Haskell"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl LiveCodeScript {
    pub fn info() -> Language {
        Language {
            name: "LiveCode Script",
            r#type: "programming",
            color: "#0c5ba5",
            extensions: &[".livecodescript"],
            tm_scope: "source.livecodescript",
            ace_mode: "text",
            language_id: 891017u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl LiveScript {
    pub fn info() -> Language {
        Language {
            name: "LiveScript",
            r#type: "programming",
            color: "#499886",
            extensions: &[".ls", "._ls"],
            tm_scope: "source.livescript",
            ace_mode: "livescript",
            language_id: 208u64,
            aliases: &["live-script", "ls"],
            codemirror_mode: Some("livescript"),
            codemirror_mime_type: Some("text/x-livescript"),
            wrap: None,
            filenames: &["Slakefile"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Logos {
    pub fn info() -> Language {
        Language {
            name: "Logos",
            r#type: "programming",
            color: "",
            extensions: &[".xm", ".x", ".xi"],
            tm_scope: "source.logos",
            ace_mode: "text",
            language_id: 209u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Logtalk {
    pub fn info() -> Language {
        Language {
            name: "Logtalk",
            r#type: "programming",
            color: "#295b9a",
            extensions: &[".lgt", ".logtalk"],
            tm_scope: "source.logtalk",
            ace_mode: "text",
            language_id: 210u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl LookML {
    pub fn info() -> Language {
        Language {
            name: "LookML",
            r#type: "programming",
            color: "#652B81",
            extensions: &[".lkml", ".lookml"],
            tm_scope: "source.yaml",
            ace_mode: "yaml",
            language_id: 211u64,
            aliases: &[],
            codemirror_mode: Some("yaml"),
            codemirror_mime_type: Some("text/x-yaml"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl LoomScript {
    pub fn info() -> Language {
        Language {
            name: "LoomScript",
            r#type: "programming",
            color: "",
            extensions: &[".ls"],
            tm_scope: "source.loomscript",
            ace_mode: "text",
            language_id: 212u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Lua {
    pub fn info() -> Language {
        Language {
            name: "Lua",
            r#type: "programming",
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
            language_id: 213u64,
            aliases: &[],
            codemirror_mode: Some("lua"),
            codemirror_mime_type: Some("text/x-lua"),
            wrap: None,
            filenames: &[".luacheckrc"],
            group: None,
            interpreters: &["lua"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Luau {
    pub fn info() -> Language {
        Language {
            name: "Luau",
            r#type: "programming",
            color: "#00A2FF",
            extensions: &[".luau"],
            tm_scope: "source.luau",
            ace_mode: "lua",
            language_id: 365050359u64,
            aliases: &[],
            codemirror_mode: Some("lua"),
            codemirror_mime_type: Some("text/x-lua"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["luau"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl M {
    pub fn info() -> Language {
        Language {
            name: "M",
            r#type: "programming",
            color: "",
            extensions: &[".mumps", ".m"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 214u64,
            aliases: &["mumps"],
            codemirror_mode: Some("mumps"),
            codemirror_mime_type: Some("text/x-mumps"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl M4 {
    pub fn info() -> Language {
        Language {
            name: "M4",
            r#type: "programming",
            color: "",
            extensions: &[".m4", ".mc"],
            tm_scope: "source.m4",
            ace_mode: "text",
            language_id: 215u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl M4Sugar {
    pub fn info() -> Language {
        Language {
            name: "M4Sugar",
            r#type: "programming",
            color: "",
            extensions: &[".m4"],
            tm_scope: "source.m4",
            ace_mode: "text",
            language_id: 216u64,
            aliases: &["autoconf"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["configure.ac"],
            group: Some("M4"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl MATLAB {
    pub fn info() -> Language {
        Language {
            name: "MATLAB",
            r#type: "programming",
            color: "#e16737",
            extensions: &[".matlab", ".m"],
            tm_scope: "source.matlab",
            ace_mode: "matlab",
            language_id: 225u64,
            aliases: &["octave"],
            codemirror_mode: Some("octave"),
            codemirror_mime_type: Some("text/x-octave"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl MAXScript {
    pub fn info() -> Language {
        Language {
            name: "MAXScript",
            r#type: "programming",
            color: "#00a6a6",
            extensions: &[".ms", ".mcr"],
            tm_scope: "source.maxscript",
            ace_mode: "text",
            language_id: 217u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl MDX {
    pub fn info() -> Language {
        Language {
            name: "MDX",
            r#type: "markup",
            color: "#fcb32c",
            extensions: &[".mdx"],
            tm_scope: "source.mdx",
            ace_mode: "markdown",
            language_id: 512838272u64,
            aliases: &[],
            codemirror_mode: Some("gfm"),
            codemirror_mime_type: Some("text/x-gfm"),
            wrap: Some(true),
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl MLIR {
    pub fn info() -> Language {
        Language {
            name: "MLIR",
            r#type: "programming",
            color: "#5EC8DB",
            extensions: &[".mlir"],
            tm_scope: "source.mlir",
            ace_mode: "text",
            language_id: 448253929u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl MQL4 {
    pub fn info() -> Language {
        Language {
            name: "MQL4",
            r#type: "programming",
            color: "#62A8D6",
            extensions: &[".mq4", ".mqh"],
            tm_scope: "source.mql5",
            ace_mode: "c_cpp",
            language_id: 426u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl MQL5 {
    pub fn info() -> Language {
        Language {
            name: "MQL5",
            r#type: "programming",
            color: "#4A76B8",
            extensions: &[".mq5", ".mqh"],
            tm_scope: "source.mql5",
            ace_mode: "c_cpp",
            language_id: 427u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl MTML {
    pub fn info() -> Language {
        Language {
            name: "MTML",
            r#type: "markup",
            color: "#b7e1f4",
            extensions: &[".mtml"],
            tm_scope: "text.html.basic",
            ace_mode: "html",
            language_id: 218u64,
            aliases: &[],
            codemirror_mode: Some("htmlmixed"),
            codemirror_mime_type: Some("text/html"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl MUF {
    pub fn info() -> Language {
        Language {
            name: "MUF",
            r#type: "programming",
            color: "",
            extensions: &[".muf", ".m"],
            tm_scope: "none",
            ace_mode: "forth",
            language_id: 219u64,
            aliases: &[],
            codemirror_mode: Some("forth"),
            codemirror_mime_type: Some("text/x-forth"),
            wrap: None,
            filenames: &[],
            group: Some("Forth"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Macaulay2 {
    pub fn info() -> Language {
        Language {
            name: "Macaulay2",
            r#type: "programming",
            color: "#d8ffff",
            extensions: &[".m2"],
            tm_scope: "source.m2",
            ace_mode: "text",
            language_id: 34167825u64,
            aliases: &["m2"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["M2"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Makefile {
    pub fn info() -> Language {
        Language {
            name: "Makefile",
            r#type: "programming",
            color: "#427819",
            extensions: &[".mak", ".d", ".make", ".makefile", ".mk", ".mkfile"],
            tm_scope: "source.makefile",
            ace_mode: "makefile",
            language_id: 220u64,
            aliases: &["bsdmake", "make", "mf"],
            codemirror_mode: Some("cmake"),
            codemirror_mime_type: Some("text/x-cmake"),
            wrap: None,
            filenames: &[
                "BSDmakefile",
                "GNUmakefile",
                "Kbuild",
                "Makefile",
                "Makefile.am",
                "Makefile.boot",
                "Makefile.frag",
                "Makefile.in",
                "Makefile.inc",
                "Makefile.wat",
                "makefile",
                "makefile.sco",
                "mkfile",
            ],
            group: None,
            interpreters: &["make"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Mako {
    pub fn info() -> Language {
        Language {
            name: "Mako",
            r#type: "programming",
            color: "#7e858d",
            extensions: &[".mako", ".mao"],
            tm_scope: "text.html.mako",
            ace_mode: "text",
            language_id: 221u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Markdown {
    pub fn info() -> Language {
        Language {
            name: "Markdown",
            r#type: "prose",
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
            language_id: 222u64,
            aliases: &["md", "pandoc"],
            codemirror_mode: Some("gfm"),
            codemirror_mime_type: Some("text/x-gfm"),
            wrap: Some(true),
            filenames: &["contents.lr"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Marko {
    pub fn info() -> Language {
        Language {
            name: "Marko",
            r#type: "markup",
            color: "#42bff2",
            extensions: &[".marko"],
            tm_scope: "text.marko",
            ace_mode: "text",
            language_id: 932782397u64,
            aliases: &["markojs"],
            codemirror_mode: Some("htmlmixed"),
            codemirror_mime_type: Some("text/html"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Mask {
    pub fn info() -> Language {
        Language {
            name: "Mask",
            r#type: "markup",
            color: "#f97732",
            extensions: &[".mask"],
            tm_scope: "source.mask",
            ace_mode: "mask",
            language_id: 223u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Mathematica {
    pub fn info() -> Language {
        Language {
            name: "Mathematica",
            r#type: "programming",
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
            language_id: 224u64,
            aliases: &["mma", "wolfram", "wolfram language", "wolfram lang", "wl"],
            codemirror_mode: Some("mathematica"),
            codemirror_mime_type: Some("text/x-mathematica"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl MavenPOM {
    pub fn info() -> Language {
        Language {
            name: "Maven POM",
            r#type: "data",
            color: "",
            extensions: &[],
            tm_scope: "text.xml.pom",
            ace_mode: "xml",
            language_id: 226u64,
            aliases: &[],
            codemirror_mode: Some("xml"),
            codemirror_mime_type: Some("text/xml"),
            wrap: None,
            filenames: &["pom.xml"],
            group: Some("XML"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Max {
    pub fn info() -> Language {
        Language {
            name: "Max",
            r#type: "programming",
            color: "#c4a79c",
            extensions: &[".maxpat", ".maxhelp", ".maxproj", ".mxt", ".pat"],
            tm_scope: "source.json",
            ace_mode: "json",
            language_id: 227u64,
            aliases: &["max/msp", "maxmsp"],
            codemirror_mode: Some("javascript"),
            codemirror_mime_type: Some("application/json"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Mercury {
    pub fn info() -> Language {
        Language {
            name: "Mercury",
            r#type: "programming",
            color: "#ff2b2b",
            extensions: &[".m", ".moo"],
            tm_scope: "source.mercury",
            ace_mode: "prolog",
            language_id: 229u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["mmi"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Mermaid {
    pub fn info() -> Language {
        Language {
            name: "Mermaid",
            r#type: "markup",
            color: "#ff3670",
            extensions: &[".mmd", ".mermaid"],
            tm_scope: "source.mermaid",
            ace_mode: "text",
            language_id: 385992043u64,
            aliases: &["mermaid example"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Meson {
    pub fn info() -> Language {
        Language {
            name: "Meson",
            r#type: "programming",
            color: "#007800",
            extensions: &[],
            tm_scope: "source.meson",
            ace_mode: "text",
            language_id: 799141244u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["meson.build", "meson_options.txt"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Metal {
    pub fn info() -> Language {
        Language {
            name: "Metal",
            r#type: "programming",
            color: "#8f14e9",
            extensions: &[".metal"],
            tm_scope: "source.c++",
            ace_mode: "c_cpp",
            language_id: 230u64,
            aliases: &[],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-c++src"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl MicrosoftDeveloperStudioProject {
    pub fn info() -> Language {
        Language {
            name: "Microsoft Developer Studio Project",
            r#type: "data",
            color: "",
            extensions: &[".dsp"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 800983837u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl MicrosoftVisualStudioSolution {
    pub fn info() -> Language {
        Language {
            name: "Microsoft Visual Studio Solution",
            r#type: "data",
            color: "",
            extensions: &[".sln"],
            tm_scope: "source.solution",
            ace_mode: "text",
            language_id: 849523096u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl MiniD {
    pub fn info() -> Language {
        Language {
            name: "MiniD",
            r#type: "programming",
            color: "",
            extensions: &[".minid"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 231u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl MiniYAML {
    pub fn info() -> Language {
        Language {
            name: "MiniYAML",
            r#type: "data",
            color: "#ff1111",
            extensions: &[".yaml", ".yml"],
            tm_scope: "source.miniyaml",
            ace_mode: "yaml",
            language_id: 4896465u64,
            aliases: &[],
            codemirror_mode: Some("yaml"),
            codemirror_mime_type: Some("text/x-yaml"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl MiniZinc {
    pub fn info() -> Language {
        Language {
            name: "MiniZinc",
            r#type: "programming",
            color: "#06a9e6",
            extensions: &[".mzn"],
            tm_scope: "source.mzn",
            ace_mode: "text",
            language_id: 238874535u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl MiniZincData {
    pub fn info() -> Language {
        Language {
            name: "MiniZinc Data",
            r#type: "data",
            color: "",
            extensions: &[".dzn"],
            tm_scope: "source.mzn",
            ace_mode: "text",
            language_id: 938193433u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Mint {
    pub fn info() -> Language {
        Language {
            name: "Mint",
            r#type: "programming",
            color: "#02b046",
            extensions: &[".mint"],
            tm_scope: "source.mint",
            ace_mode: "text",
            language_id: 968740319u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Mirah {
    pub fn info() -> Language {
        Language {
            name: "Mirah",
            r#type: "programming",
            color: "#c7a938",
            extensions: &[".druby", ".duby", ".mirah"],
            tm_scope: "source.ruby",
            ace_mode: "ruby",
            language_id: 232u64,
            aliases: &[],
            codemirror_mode: Some("ruby"),
            codemirror_mime_type: Some("text/x-ruby"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Modelica {
    pub fn info() -> Language {
        Language {
            name: "Modelica",
            r#type: "programming",
            color: "#de1d31",
            extensions: &[".mo"],
            tm_scope: "source.modelica",
            ace_mode: "text",
            language_id: 233u64,
            aliases: &[],
            codemirror_mode: Some("modelica"),
            codemirror_mime_type: Some("text/x-modelica"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Modula2 {
    pub fn info() -> Language {
        Language {
            name: "Modula-2",
            r#type: "programming",
            color: "#10253f",
            extensions: &[".mod"],
            tm_scope: "source.modula2",
            ace_mode: "text",
            language_id: 234u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Modula3 {
    pub fn info() -> Language {
        Language {
            name: "Modula-3",
            r#type: "programming",
            color: "#223388",
            extensions: &[".i3", ".ig", ".m3", ".mg"],
            tm_scope: "source.modula-3",
            ace_mode: "text",
            language_id: 564743864u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ModuleManagementSystem {
    pub fn info() -> Language {
        Language {
            name: "Module Management System",
            r#type: "programming",
            color: "",
            extensions: &[".mms", ".mmk"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 235u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["descrip.mmk", "descrip.mms"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Mojo {
    pub fn info() -> Language {
        Language {
            name: "Mojo",
            r#type: "programming",
            color: "#ff4c1f",
            extensions: &[".mojo"],
            tm_scope: "source.mojo",
            ace_mode: "python",
            language_id: 1045019587u64,
            aliases: &[],
            codemirror_mode: Some("python"),
            codemirror_mime_type: Some("text/x-python"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Monkey {
    pub fn info() -> Language {
        Language {
            name: "Monkey",
            r#type: "programming",
            color: "",
            extensions: &[".monkey", ".monkey2"],
            tm_scope: "source.monkey",
            ace_mode: "text",
            language_id: 236u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl MonkeyC {
    pub fn info() -> Language {
        Language {
            name: "Monkey C",
            r#type: "programming",
            color: "#8D6747",
            extensions: &[".mc"],
            tm_scope: "source.mc",
            ace_mode: "c_cpp",
            language_id: 231751931u64,
            aliases: &[],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-csrc"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Moocode {
    pub fn info() -> Language {
        Language {
            name: "Moocode",
            r#type: "programming",
            color: "",
            extensions: &[".moo"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 237u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl MoonBit {
    pub fn info() -> Language {
        Language {
            name: "MoonBit",
            r#type: "programming",
            color: "#b92381",
            extensions: &[".mbt"],
            tm_scope: "source.moonbit",
            ace_mode: "text",
            language_id: 181453007u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl MoonScript {
    pub fn info() -> Language {
        Language {
            name: "MoonScript",
            r#type: "programming",
            color: "#ff4585",
            extensions: &[".moon"],
            tm_scope: "source.moonscript",
            ace_mode: "text",
            language_id: 238u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["moon"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Motoko {
    pub fn info() -> Language {
        Language {
            name: "Motoko",
            r#type: "programming",
            color: "#fbb03b",
            extensions: &[".mo"],
            tm_scope: "source.mo",
            ace_mode: "text",
            language_id: 202937027u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Motorola68KAssembly {
    pub fn info() -> Language {
        Language {
            name: "Motorola 68K Assembly",
            r#type: "programming",
            color: "#005daa",
            extensions: &[".asm", ".i", ".inc", ".s", ".x68"],
            tm_scope: "source.m68k",
            ace_mode: "assembly_x86",
            language_id: 477582706u64,
            aliases: &["m68k"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: Some("Assembly"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Move {
    pub fn info() -> Language {
        Language {
            name: "Move",
            r#type: "programming",
            color: "#4a137a",
            extensions: &[".move"],
            tm_scope: "source.move",
            ace_mode: "text",
            language_id: 638334599u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Muse {
    pub fn info() -> Language {
        Language {
            name: "Muse",
            r#type: "prose",
            color: "",
            extensions: &[".muse"],
            tm_scope: "text.muse",
            ace_mode: "text",
            language_id: 474864066u64,
            aliases: &["amusewiki", "emacs muse"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: Some(true),
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Mustache {
    pub fn info() -> Language {
        Language {
            name: "Mustache",
            r#type: "markup",
            color: "#724b3b",
            extensions: &[".mustache"],
            tm_scope: "text.html.smarty",
            ace_mode: "smarty",
            language_id: 638334590u64,
            aliases: &[],
            codemirror_mode: Some("smarty"),
            codemirror_mime_type: Some("text/x-smarty"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Myghty {
    pub fn info() -> Language {
        Language {
            name: "Myghty",
            r#type: "programming",
            color: "",
            extensions: &[".myt"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 239u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl NASL {
    pub fn info() -> Language {
        Language {
            name: "NASL",
            r#type: "programming",
            color: "",
            extensions: &[".nasl", ".inc"],
            tm_scope: "source.nasl",
            ace_mode: "text",
            language_id: 171666519u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl NCL {
    pub fn info() -> Language {
        Language {
            name: "NCL",
            r#type: "programming",
            color: "#28431f",
            extensions: &[".ncl"],
            tm_scope: "source.ncl",
            ace_mode: "text",
            language_id: 240u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl NEON {
    pub fn info() -> Language {
        Language {
            name: "NEON",
            r#type: "data",
            color: "",
            extensions: &[".neon"],
            tm_scope: "source.neon",
            ace_mode: "text",
            language_id: 481192983u64,
            aliases: &["nette object notation", "ne-on"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl NL {
    pub fn info() -> Language {
        Language {
            name: "NL",
            r#type: "data",
            color: "",
            extensions: &[".nl"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 241u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl NMODL {
    pub fn info() -> Language {
        Language {
            name: "NMODL",
            r#type: "programming",
            color: "#00356B",
            extensions: &[".mod"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 136456478u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl NPMConfig {
    pub fn info() -> Language {
        Language {
            name: "NPM Config",
            r#type: "data",
            color: "#cb3837",
            extensions: &[],
            tm_scope: "source.ini.npmrc",
            ace_mode: "text",
            language_id: 685022663u64,
            aliases: &["npmrc"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[".npmrc"],
            group: Some("INI"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl NSIS {
    pub fn info() -> Language {
        Language {
            name: "NSIS",
            r#type: "programming",
            color: "",
            extensions: &[".nsi", ".nsh"],
            tm_scope: "source.nsis",
            ace_mode: "text",
            language_id: 242u64,
            aliases: &[],
            codemirror_mode: Some("nsis"),
            codemirror_mime_type: Some("text/x-nsis"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl NWScript {
    pub fn info() -> Language {
        Language {
            name: "NWScript",
            r#type: "programming",
            color: "#111522",
            extensions: &[".nss"],
            tm_scope: "source.c.nwscript",
            ace_mode: "c_cpp",
            language_id: 731233819u64,
            aliases: &[],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-csrc"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Nasal {
    pub fn info() -> Language {
        Language {
            name: "Nasal",
            r#type: "programming",
            color: "#1d2c4e",
            extensions: &[".nas"],
            tm_scope: "source.nasal",
            ace_mode: "nasal",
            language_id: 178322513u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Nearley {
    pub fn info() -> Language {
        Language {
            name: "Nearley",
            r#type: "programming",
            color: "#990000",
            extensions: &[".ne", ".nearley"],
            tm_scope: "source.ne",
            ace_mode: "text",
            language_id: 521429430u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Nemerle {
    pub fn info() -> Language {
        Language {
            name: "Nemerle",
            r#type: "programming",
            color: "#3d3c6e",
            extensions: &[".n"],
            tm_scope: "source.nemerle",
            ace_mode: "text",
            language_id: 243u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl NetLinx {
    pub fn info() -> Language {
        Language {
            name: "NetLinx",
            r#type: "programming",
            color: "#0aa0ff",
            extensions: &[".axs", ".axi"],
            tm_scope: "source.netlinx",
            ace_mode: "text",
            language_id: 244u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl NetLinxpERB {
    pub fn info() -> Language {
        Language {
            name: "NetLinx+ERB",
            r#type: "programming",
            color: "#747faa",
            extensions: &[".axs.erb", ".axi.erb"],
            tm_scope: "source.netlinx.erb",
            ace_mode: "text",
            language_id: 245u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl NetLogo {
    pub fn info() -> Language {
        Language {
            name: "NetLogo",
            r#type: "programming",
            color: "#ff6375",
            extensions: &[".nlogo"],
            tm_scope: "source.lisp",
            ace_mode: "lisp",
            language_id: 246u64,
            aliases: &[],
            codemirror_mode: Some("commonlisp"),
            codemirror_mime_type: Some("text/x-common-lisp"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl NewLisp {
    pub fn info() -> Language {
        Language {
            name: "NewLisp",
            r#type: "programming",
            color: "#87AED7",
            extensions: &[".nl", ".lisp", ".lsp"],
            tm_scope: "source.lisp",
            ace_mode: "lisp",
            language_id: 247u64,
            aliases: &[],
            codemirror_mode: Some("commonlisp"),
            codemirror_mime_type: Some("text/x-common-lisp"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["newlisp"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Nextflow {
    pub fn info() -> Language {
        Language {
            name: "Nextflow",
            r#type: "programming",
            color: "#3ac486",
            extensions: &[".nf"],
            tm_scope: "source.nextflow",
            ace_mode: "groovy",
            language_id: 506780613u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["nextflow.config"],
            group: None,
            interpreters: &["nextflow"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Nginx {
    pub fn info() -> Language {
        Language {
            name: "Nginx",
            r#type: "data",
            color: "#009639",
            extensions: &[".nginx", ".nginxconf", ".vhost"],
            tm_scope: "source.nginx",
            ace_mode: "text",
            language_id: 248u64,
            aliases: &["nginx configuration file"],
            codemirror_mode: Some("nginx"),
            codemirror_mime_type: Some("text/x-nginx-conf"),
            wrap: None,
            filenames: &["nginx.conf"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Nim {
    pub fn info() -> Language {
        Language {
            name: "Nim",
            r#type: "programming",
            color: "#ffc200",
            extensions: &[".nim", ".nim.cfg", ".nimble", ".nimrod", ".nims"],
            tm_scope: "source.nim",
            ace_mode: "text",
            language_id: 249u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["nim.cfg"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Ninja {
    pub fn info() -> Language {
        Language {
            name: "Ninja",
            r#type: "data",
            color: "",
            extensions: &[".ninja"],
            tm_scope: "source.ninja",
            ace_mode: "text",
            language_id: 250u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Nit {
    pub fn info() -> Language {
        Language {
            name: "Nit",
            r#type: "programming",
            color: "#009917",
            extensions: &[".nit"],
            tm_scope: "source.nit",
            ace_mode: "text",
            language_id: 251u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Nix {
    pub fn info() -> Language {
        Language {
            name: "Nix",
            r#type: "programming",
            color: "#7e7eff",
            extensions: &[".nix"],
            tm_scope: "source.nix",
            ace_mode: "nix",
            language_id: 252u64,
            aliases: &["nixos"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Noir {
    pub fn info() -> Language {
        Language {
            name: "Noir",
            r#type: "programming",
            color: "#2f1f49",
            extensions: &[".nr"],
            tm_scope: "source.nr",
            ace_mode: "rust",
            language_id: 813068465u64,
            aliases: &["nargo"],
            codemirror_mode: Some("rust"),
            codemirror_mime_type: Some("text/x-rustsrc"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Nu {
    pub fn info() -> Language {
        Language {
            name: "Nu",
            r#type: "programming",
            color: "#c9df40",
            extensions: &[".nu"],
            tm_scope: "source.nu",
            ace_mode: "scheme",
            language_id: 253u64,
            aliases: &["nush"],
            codemirror_mode: Some("scheme"),
            codemirror_mime_type: Some("text/x-scheme"),
            wrap: None,
            filenames: &["Nukefile"],
            group: None,
            interpreters: &["nush"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl NumPy {
    pub fn info() -> Language {
        Language {
            name: "NumPy",
            r#type: "programming",
            color: "#9C8AF9",
            extensions: &[".numpy", ".numpyw", ".numsc"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 254u64,
            aliases: &[],
            codemirror_mode: Some("python"),
            codemirror_mime_type: Some("text/x-python"),
            wrap: None,
            filenames: &[],
            group: Some("Python"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Nunjucks {
    pub fn info() -> Language {
        Language {
            name: "Nunjucks",
            r#type: "markup",
            color: "#3d8137",
            extensions: &[".njk"],
            tm_scope: "text.html.nunjucks",
            ace_mode: "nunjucks",
            language_id: 461856962u64,
            aliases: &["njk"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Nushell {
    pub fn info() -> Language {
        Language {
            name: "Nushell",
            r#type: "programming",
            color: "#4E9906",
            extensions: &[".nu"],
            tm_scope: "source.nushell",
            ace_mode: "sh",
            language_id: 446573572u64,
            aliases: &["nu-script", "nushell-script"],
            codemirror_mode: Some("shell"),
            codemirror_mime_type: Some("text/x-sh"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["nu"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl OASv2Json {
    pub fn info() -> Language {
        Language {
            name: "OASv2-json",
            r#type: "data",
            color: "#85ea2d",
            extensions: &[".json"],
            tm_scope: "source.json",
            ace_mode: "json",
            language_id: 834374816u64,
            aliases: &[],
            codemirror_mode: Some("javascript"),
            codemirror_mime_type: Some("application/json"),
            wrap: None,
            filenames: &[],
            group: Some("OpenAPI Specification v2"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl OASv2Yaml {
    pub fn info() -> Language {
        Language {
            name: "OASv2-yaml",
            r#type: "data",
            color: "#85ea2d",
            extensions: &[".yaml", ".yml"],
            tm_scope: "source.yaml",
            ace_mode: "yaml",
            language_id: 105187618u64,
            aliases: &[],
            codemirror_mode: Some("yaml"),
            codemirror_mime_type: Some("text/x-yaml"),
            wrap: None,
            filenames: &[],
            group: Some("OpenAPI Specification v2"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl OASv3Json {
    pub fn info() -> Language {
        Language {
            name: "OASv3-json",
            r#type: "data",
            color: "#85ea2d",
            extensions: &[".json"],
            tm_scope: "source.json",
            ace_mode: "json",
            language_id: 980062566u64,
            aliases: &[],
            codemirror_mode: Some("javascript"),
            codemirror_mime_type: Some("application/json"),
            wrap: None,
            filenames: &[],
            group: Some("OpenAPI Specification v3"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl OASv3Yaml {
    pub fn info() -> Language {
        Language {
            name: "OASv3-yaml",
            r#type: "data",
            color: "#85ea2d",
            extensions: &[".yaml", ".yml"],
            tm_scope: "source.yaml",
            ace_mode: "yaml",
            language_id: 51239111u64,
            aliases: &[],
            codemirror_mode: Some("yaml"),
            codemirror_mime_type: Some("text/x-yaml"),
            wrap: None,
            filenames: &[],
            group: Some("OpenAPI Specification v3"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl OCaml {
    pub fn info() -> Language {
        Language {
            name: "OCaml",
            r#type: "programming",
            color: "#ef7a08",
            extensions: &[".ml", ".eliom", ".eliomi", ".ml4", ".mli", ".mll", ".mly"],
            tm_scope: "source.ocaml",
            ace_mode: "ocaml",
            language_id: 255u64,
            aliases: &[],
            codemirror_mode: Some("mllike"),
            codemirror_mime_type: Some("text/x-ocaml"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["ocaml", "ocamlrun", "ocamlscript"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl OMNeTppMSG {
    pub fn info() -> Language {
        Language {
            name: "OMNeT++ MSG",
            r#type: "programming",
            color: "#a0e0a0",
            extensions: &[".msg"],
            tm_scope: "source.msg",
            ace_mode: "text",
            language_id: 664100008u64,
            aliases: &["omnetpp-msg"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl OMNeTppNED {
    pub fn info() -> Language {
        Language {
            name: "OMNeT++ NED",
            r#type: "programming",
            color: "#08607c",
            extensions: &[".ned"],
            tm_scope: "source.ned",
            ace_mode: "text",
            language_id: 924868392u64,
            aliases: &["omnetpp-ned"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Oberon {
    pub fn info() -> Language {
        Language {
            name: "Oberon",
            r#type: "programming",
            color: "",
            extensions: &[".ob2"],
            tm_scope: "source.modula2",
            ace_mode: "text",
            language_id: 677210597u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ObjDump {
    pub fn info() -> Language {
        Language {
            name: "ObjDump",
            r#type: "data",
            color: "",
            extensions: &[".objdump"],
            tm_scope: "objdump.x86asm",
            ace_mode: "assembly_x86",
            language_id: 256u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ObjectDataInstanceNotation {
    pub fn info() -> Language {
        Language {
            name: "Object Data Instance Notation",
            r#type: "data",
            color: "",
            extensions: &[".odin"],
            tm_scope: "source.odin-ehr",
            ace_mode: "text",
            language_id: 985227236u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ObjectScript {
    pub fn info() -> Language {
        Language {
            name: "ObjectScript",
            r#type: "programming",
            color: "#424893",
            extensions: &[".cls"],
            tm_scope: "source.objectscript",
            ace_mode: "text",
            language_id: 202735509u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ObjectiveC {
    pub fn info() -> Language {
        Language {
            name: "Objective-C",
            r#type: "programming",
            color: "#438eff",
            extensions: &[".m", ".h"],
            tm_scope: "source.objc",
            ace_mode: "objectivec",
            language_id: 257u64,
            aliases: &["obj-c", "objc", "objectivec"],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-objectivec"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ObjectiveCpp {
    pub fn info() -> Language {
        Language {
            name: "Objective-C++",
            r#type: "programming",
            color: "#6866fb",
            extensions: &[".mm"],
            tm_scope: "source.objc++",
            ace_mode: "objectivec",
            language_id: 258u64,
            aliases: &["obj-c++", "objc++", "objectivec++"],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-objectivec"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ObjectiveJ {
    pub fn info() -> Language {
        Language {
            name: "Objective-J",
            r#type: "programming",
            color: "#ff0c5a",
            extensions: &[".j", ".sj"],
            tm_scope: "source.js.objj",
            ace_mode: "text",
            language_id: 259u64,
            aliases: &["obj-j", "objectivej", "objj"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Odin {
    pub fn info() -> Language {
        Language {
            name: "Odin",
            r#type: "programming",
            color: "#60AFFE",
            extensions: &[".odin"],
            tm_scope: "source.odin",
            ace_mode: "text",
            language_id: 889244082u64,
            aliases: &["odinlang", "odin-lang"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Omgrofl {
    pub fn info() -> Language {
        Language {
            name: "Omgrofl",
            r#type: "programming",
            color: "#cabbff",
            extensions: &[".omgrofl"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 260u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Opa {
    pub fn info() -> Language {
        Language {
            name: "Opa",
            r#type: "programming",
            color: "",
            extensions: &[".opa"],
            tm_scope: "source.opa",
            ace_mode: "text",
            language_id: 261u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Opal {
    pub fn info() -> Language {
        Language {
            name: "Opal",
            r#type: "programming",
            color: "#f7ede0",
            extensions: &[".opal"],
            tm_scope: "source.opal",
            ace_mode: "text",
            language_id: 262u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl OpenPolicyAgent {
    pub fn info() -> Language {
        Language {
            name: "Open Policy Agent",
            r#type: "programming",
            color: "#7d9199",
            extensions: &[".rego"],
            tm_scope: "source.rego",
            ace_mode: "text",
            language_id: 840483232u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl OpenAPISpecificationV2 {
    pub fn info() -> Language {
        Language {
            name: "OpenAPI Specification v2",
            r#type: "data",
            color: "#85ea2d",
            extensions: &[],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 848295328u64,
            aliases: &["oasv2"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl OpenAPISpecificationV3 {
    pub fn info() -> Language {
        Language {
            name: "OpenAPI Specification v3",
            r#type: "data",
            color: "#85ea2d",
            extensions: &[],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 557959099u64,
            aliases: &["oasv3"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl OpenCL {
    pub fn info() -> Language {
        Language {
            name: "OpenCL",
            r#type: "programming",
            color: "#ed2e2d",
            extensions: &[".cl", ".opencl"],
            tm_scope: "source.c",
            ace_mode: "c_cpp",
            language_id: 263u64,
            aliases: &[],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-csrc"),
            wrap: None,
            filenames: &[],
            group: Some("C"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl OpenEdgeABL {
    pub fn info() -> Language {
        Language {
            name: "OpenEdge ABL",
            r#type: "programming",
            color: "#5ce600",
            extensions: &[".p", ".cls", ".w"],
            tm_scope: "source.abl",
            ace_mode: "text",
            language_id: 264u64,
            aliases: &["progress", "openedge", "abl"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl OpenQASM {
    pub fn info() -> Language {
        Language {
            name: "OpenQASM",
            r#type: "programming",
            color: "#AA70FF",
            extensions: &[".qasm"],
            tm_scope: "source.qasm",
            ace_mode: "text",
            language_id: 153739399u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl OpenRCRunscript {
    pub fn info() -> Language {
        Language {
            name: "OpenRC runscript",
            r#type: "programming",
            color: "",
            extensions: &[],
            tm_scope: "source.shell",
            ace_mode: "sh",
            language_id: 265u64,
            aliases: &["openrc"],
            codemirror_mode: Some("shell"),
            codemirror_mime_type: Some("text/x-sh"),
            wrap: None,
            filenames: &[],
            group: Some("Shell"),
            interpreters: &["openrc-run"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl OpenSCAD {
    pub fn info() -> Language {
        Language {
            name: "OpenSCAD",
            r#type: "programming",
            color: "#e5cd45",
            extensions: &[".scad"],
            tm_scope: "source.scad",
            ace_mode: "scad",
            language_id: 266u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl OpenStepPropertyList {
    pub fn info() -> Language {
        Language {
            name: "OpenStep Property List",
            r#type: "data",
            color: "",
            extensions: &[".plist", ".glyphs"],
            tm_scope: "source.plist",
            ace_mode: "text",
            language_id: 598917541u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl OpenTypeFeatureFile {
    pub fn info() -> Language {
        Language {
            name: "OpenType Feature File",
            r#type: "data",
            color: "",
            extensions: &[".fea"],
            tm_scope: "source.opentype",
            ace_mode: "text",
            language_id: 374317347u64,
            aliases: &["AFDKO"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl OptionList {
    pub fn info() -> Language {
        Language {
            name: "Option List",
            r#type: "data",
            color: "#476732",
            extensions: &[],
            tm_scope: "source.opts",
            ace_mode: "sh",
            language_id: 723589315u64,
            aliases: &["opts", "ackrc"],
            codemirror_mode: Some("shell"),
            codemirror_mime_type: Some("text/x-sh"),
            wrap: None,
            filenames: &[".ackrc", ".rspec", ".yardopts", "ackrc", "mocha.opts"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Org {
    pub fn info() -> Language {
        Language {
            name: "Org",
            r#type: "prose",
            color: "#77aa99",
            extensions: &[".org"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 267u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: Some(true),
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl OverpassQL {
    pub fn info() -> Language {
        Language {
            name: "OverpassQL",
            r#type: "programming",
            color: "#cce2aa",
            extensions: &[".overpassql"],
            tm_scope: "source.overpassql",
            ace_mode: "text",
            language_id: 689079655u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: Some(true),
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Ox {
    pub fn info() -> Language {
        Language {
            name: "Ox",
            r#type: "programming",
            color: "",
            extensions: &[".ox", ".oxh", ".oxo"],
            tm_scope: "source.ox",
            ace_mode: "text",
            language_id: 268u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Oxygene {
    pub fn info() -> Language {
        Language {
            name: "Oxygene",
            r#type: "programming",
            color: "#cdd0e3",
            extensions: &[".oxygene"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 269u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Oz {
    pub fn info() -> Language {
        Language {
            name: "Oz",
            r#type: "programming",
            color: "#fab738",
            extensions: &[".oz"],
            tm_scope: "source.oz",
            ace_mode: "text",
            language_id: 270u64,
            aliases: &[],
            codemirror_mode: Some("oz"),
            codemirror_mime_type: Some("text/x-oz"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl P4 {
    pub fn info() -> Language {
        Language {
            name: "P4",
            r#type: "programming",
            color: "#7055b5",
            extensions: &[".p4"],
            tm_scope: "source.p4",
            ace_mode: "text",
            language_id: 348895984u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl PDDL {
    pub fn info() -> Language {
        Language {
            name: "PDDL",
            r#type: "programming",
            color: "#0d00ff",
            extensions: &[".pddl"],
            tm_scope: "source.pddl",
            ace_mode: "text",
            language_id: 736235603u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl PEGjs {
    pub fn info() -> Language {
        Language {
            name: "PEG.js",
            r#type: "programming",
            color: "#234d6b",
            extensions: &[".pegjs", ".peggy"],
            tm_scope: "source.peggy",
            ace_mode: "javascript",
            language_id: 81442128u64,
            aliases: &[],
            codemirror_mode: Some("javascript"),
            codemirror_mime_type: Some("text/javascript"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl PHP {
    pub fn info() -> Language {
        Language {
            name: "PHP",
            r#type: "programming",
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
            language_id: 272u64,
            aliases: &["inc"],
            codemirror_mode: Some("php"),
            codemirror_mime_type: Some("application/x-httpd-php"),
            wrap: None,
            filenames: &[".php", ".php_cs", ".php_cs.dist", "Phakefile"],
            group: None,
            interpreters: &["php"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl PLSQL {
    pub fn info() -> Language {
        Language {
            name: "PLSQL",
            r#type: "programming",
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
            language_id: 273u64,
            aliases: &[],
            codemirror_mode: Some("sql"),
            codemirror_mime_type: Some("text/x-plsql"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl PLpgSQL {
    pub fn info() -> Language {
        Language {
            name: "PLpgSQL",
            r#type: "programming",
            color: "#336790",
            extensions: &[".pgsql", ".sql"],
            tm_scope: "source.sql",
            ace_mode: "pgsql",
            language_id: 274u64,
            aliases: &[],
            codemirror_mode: Some("sql"),
            codemirror_mime_type: Some("text/x-sql"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl POVRaySDL {
    pub fn info() -> Language {
        Language {
            name: "POV-Ray SDL",
            r#type: "programming",
            color: "#6bac65",
            extensions: &[".pov", ".inc"],
            tm_scope: "source.pov-ray sdl",
            ace_mode: "text",
            language_id: 275u64,
            aliases: &["pov-ray", "povray"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Pact {
    pub fn info() -> Language {
        Language {
            name: "Pact",
            r#type: "programming",
            color: "#F7A8B8",
            extensions: &[".pact"],
            tm_scope: "source.pact",
            ace_mode: "text",
            language_id: 756774415u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Pan {
    pub fn info() -> Language {
        Language {
            name: "Pan",
            r#type: "programming",
            color: "#cc0000",
            extensions: &[".pan"],
            tm_scope: "source.pan",
            ace_mode: "text",
            language_id: 276u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Papyrus {
    pub fn info() -> Language {
        Language {
            name: "Papyrus",
            r#type: "programming",
            color: "#6600cc",
            extensions: &[".psc"],
            tm_scope: "source.papyrus.skyrim",
            ace_mode: "text",
            language_id: 277u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Parrot {
    pub fn info() -> Language {
        Language {
            name: "Parrot",
            r#type: "programming",
            color: "#f3ca0a",
            extensions: &[".parrot"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 278u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ParrotAssembly {
    pub fn info() -> Language {
        Language {
            name: "Parrot Assembly",
            r#type: "programming",
            color: "",
            extensions: &[".pasm"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 279u64,
            aliases: &["pasm"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: Some("Parrot"),
            interpreters: &["parrot"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ParrotInternalRepresentation {
    pub fn info() -> Language {
        Language {
            name: "Parrot Internal Representation",
            r#type: "programming",
            color: "",
            extensions: &[".pir"],
            tm_scope: "source.parrot.pir",
            ace_mode: "text",
            language_id: 280u64,
            aliases: &["pir"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: Some("Parrot"),
            interpreters: &["parrot"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Pascal {
    pub fn info() -> Language {
        Language {
            name: "Pascal",
            r#type: "programming",
            color: "#E3F171",
            extensions: &[".pas", ".dfm", ".dpr", ".inc", ".lpr", ".pascal", ".pp"],
            tm_scope: "source.pascal",
            ace_mode: "pascal",
            language_id: 281u64,
            aliases: &["delphi", "objectpascal"],
            codemirror_mode: Some("pascal"),
            codemirror_mime_type: Some("text/x-pascal"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["instantfpc"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Pawn {
    pub fn info() -> Language {
        Language {
            name: "Pawn",
            r#type: "programming",
            color: "#dbb284",
            extensions: &[".pwn", ".inc", ".sma"],
            tm_scope: "source.pawn",
            ace_mode: "text",
            language_id: 271u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Pep8 {
    pub fn info() -> Language {
        Language {
            name: "Pep8",
            r#type: "programming",
            color: "#C76F5B",
            extensions: &[".pep"],
            tm_scope: "source.pep8",
            ace_mode: "text",
            language_id: 840372442u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Perl {
    pub fn info() -> Language {
        Language {
            name: "Perl",
            r#type: "programming",
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
            language_id: 282u64,
            aliases: &["cperl"],
            codemirror_mode: Some("perl"),
            codemirror_mime_type: Some("text/x-perl"),
            wrap: None,
            filenames: &[
                ".latexmkrc",
                "Makefile.PL",
                "Rexfile",
                "ack",
                "cpanfile",
                "latexmkrc",
            ],
            group: None,
            interpreters: &["cperl", "perl"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Pic {
    pub fn info() -> Language {
        Language {
            name: "Pic",
            r#type: "markup",
            color: "",
            extensions: &[".pic", ".chem"],
            tm_scope: "source.pic",
            ace_mode: "text",
            language_id: 425u64,
            aliases: &["pikchr"],
            codemirror_mode: Some("troff"),
            codemirror_mime_type: Some("text/troff"),
            wrap: None,
            filenames: &[],
            group: Some("Roff"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Pickle {
    pub fn info() -> Language {
        Language {
            name: "Pickle",
            r#type: "data",
            color: "",
            extensions: &[".pkl"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 284u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl PicoLisp {
    pub fn info() -> Language {
        Language {
            name: "PicoLisp",
            r#type: "programming",
            color: "#6067af",
            extensions: &[".l"],
            tm_scope: "source.lisp",
            ace_mode: "lisp",
            language_id: 285u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["picolisp", "pil"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl PigLatin {
    pub fn info() -> Language {
        Language {
            name: "PigLatin",
            r#type: "programming",
            color: "#fcd7de",
            extensions: &[".pig"],
            tm_scope: "source.pig_latin",
            ace_mode: "text",
            language_id: 286u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Pike {
    pub fn info() -> Language {
        Language {
            name: "Pike",
            r#type: "programming",
            color: "#005390",
            extensions: &[".pike", ".pmod"],
            tm_scope: "source.pike",
            ace_mode: "text",
            language_id: 287u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["pike"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl PipRequirements {
    pub fn info() -> Language {
        Language {
            name: "Pip Requirements",
            r#type: "data",
            color: "#FFD343",
            extensions: &[],
            tm_scope: "source.pip-requirements",
            ace_mode: "text",
            language_id: 684385621u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["requirements-dev.txt", "requirements.txt"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Pkl {
    pub fn info() -> Language {
        Language {
            name: "Pkl",
            r#type: "programming",
            color: "#6b9543",
            extensions: &[".pkl"],
            tm_scope: "source.pkl",
            ace_mode: "text",
            language_id: 288822799u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["pkl"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl PlantUML {
    pub fn info() -> Language {
        Language {
            name: "PlantUML",
            r#type: "data",
            color: "#fbbd16",
            extensions: &[".puml", ".iuml", ".plantuml"],
            tm_scope: "source.wsd",
            ace_mode: "text",
            language_id: 833504686u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Pod {
    pub fn info() -> Language {
        Language {
            name: "Pod",
            r#type: "prose",
            color: "",
            extensions: &[".pod"],
            tm_scope: "none",
            ace_mode: "perl",
            language_id: 288u64,
            aliases: &[],
            codemirror_mode: Some("perl"),
            codemirror_mime_type: Some("text/x-perl"),
            wrap: Some(true),
            filenames: &[],
            group: None,
            interpreters: &["perl"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Pod6 {
    pub fn info() -> Language {
        Language {
            name: "Pod 6",
            r#type: "prose",
            color: "",
            extensions: &[".pod", ".pod6"],
            tm_scope: "source.raku",
            ace_mode: "perl",
            language_id: 155357471u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: Some(true),
            filenames: &[],
            group: None,
            interpreters: &["perl6"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl PogoScript {
    pub fn info() -> Language {
        Language {
            name: "PogoScript",
            r#type: "programming",
            color: "#d80074",
            extensions: &[".pogo"],
            tm_scope: "source.pogoscript",
            ace_mode: "text",
            language_id: 289u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Polar {
    pub fn info() -> Language {
        Language {
            name: "Polar",
            r#type: "programming",
            color: "#ae81ff",
            extensions: &[".polar"],
            tm_scope: "source.polar",
            ace_mode: "text",
            language_id: 839112914u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Pony {
    pub fn info() -> Language {
        Language {
            name: "Pony",
            r#type: "programming",
            color: "",
            extensions: &[".pony"],
            tm_scope: "source.pony",
            ace_mode: "text",
            language_id: 290u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Portugol {
    pub fn info() -> Language {
        Language {
            name: "Portugol",
            r#type: "programming",
            color: "#f8bd00",
            extensions: &[".por"],
            tm_scope: "source.portugol",
            ace_mode: "text",
            language_id: 832391833u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl PostCSS {
    pub fn info() -> Language {
        Language {
            name: "PostCSS",
            r#type: "markup",
            color: "#dc3a0c",
            extensions: &[".pcss", ".postcss"],
            tm_scope: "source.postcss",
            ace_mode: "text",
            language_id: 262764437u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: Some("CSS"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl PostScript {
    pub fn info() -> Language {
        Language {
            name: "PostScript",
            r#type: "markup",
            color: "#da291c",
            extensions: &[".ps", ".eps", ".epsi", ".pfa"],
            tm_scope: "source.postscript",
            ace_mode: "text",
            language_id: 291u64,
            aliases: &["postscr"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl PowerBuilder {
    pub fn info() -> Language {
        Language {
            name: "PowerBuilder",
            r#type: "programming",
            color: "#8f0f8d",
            extensions: &[".pbt", ".sra", ".sru", ".srw"],
            tm_scope: "source.powerbuilder",
            ace_mode: "text",
            language_id: 292u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl PowerShell {
    pub fn info() -> Language {
        Language {
            name: "PowerShell",
            r#type: "programming",
            color: "#012456",
            extensions: &[".ps1", ".psd1", ".psm1"],
            tm_scope: "source.powershell",
            ace_mode: "powershell",
            language_id: 293u64,
            aliases: &["posh", "pwsh"],
            codemirror_mode: Some("powershell"),
            codemirror_mime_type: Some("application/x-powershell"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["pwsh"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Praat {
    pub fn info() -> Language {
        Language {
            name: "Praat",
            r#type: "programming",
            color: "#c8506d",
            extensions: &[".praat"],
            tm_scope: "source.praat",
            ace_mode: "praat",
            language_id: 106029007u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Prisma {
    pub fn info() -> Language {
        Language {
            name: "Prisma",
            r#type: "data",
            color: "#0c344b",
            extensions: &[".prisma"],
            tm_scope: "source.prisma",
            ace_mode: "text",
            language_id: 499933428u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Processing {
    pub fn info() -> Language {
        Language {
            name: "Processing",
            r#type: "programming",
            color: "#0096D8",
            extensions: &[".pde"],
            tm_scope: "source.processing",
            ace_mode: "text",
            language_id: 294u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Procfile {
    pub fn info() -> Language {
        Language {
            name: "Procfile",
            r#type: "programming",
            color: "#3B2F63",
            extensions: &[],
            tm_scope: "source.procfile",
            ace_mode: "batchfile",
            language_id: 305313959u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["Procfile"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Proguard {
    pub fn info() -> Language {
        Language {
            name: "Proguard",
            r#type: "data",
            color: "",
            extensions: &[".pro"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 716513858u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Prolog {
    pub fn info() -> Language {
        Language {
            name: "Prolog",
            r#type: "programming",
            color: "#74283c",
            extensions: &[".pl", ".plt", ".pro", ".prolog", ".yap"],
            tm_scope: "source.prolog",
            ace_mode: "prolog",
            language_id: 295u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["swipl", "yap"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Promela {
    pub fn info() -> Language {
        Language {
            name: "Promela",
            r#type: "programming",
            color: "#de0000",
            extensions: &[".pml"],
            tm_scope: "source.promela",
            ace_mode: "text",
            language_id: 441858312u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl PropellerSpin {
    pub fn info() -> Language {
        Language {
            name: "Propeller Spin",
            r#type: "programming",
            color: "#7fa2a7",
            extensions: &[".spin"],
            tm_scope: "source.spin",
            ace_mode: "text",
            language_id: 296u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ProtocolBuffer {
    pub fn info() -> Language {
        Language {
            name: "Protocol Buffer",
            r#type: "data",
            color: "",
            extensions: &[".proto"],
            tm_scope: "source.proto",
            ace_mode: "protobuf",
            language_id: 297u64,
            aliases: &["proto", "protobuf", "Protocol Buffers"],
            codemirror_mode: Some("protobuf"),
            codemirror_mime_type: Some("text/x-protobuf"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ProtocolBufferTextFormat {
    pub fn info() -> Language {
        Language {
            name: "Protocol Buffer Text Format",
            r#type: "data",
            color: "",
            extensions: &[".textproto", ".pbt", ".pbtxt"],
            tm_scope: "source.textproto",
            ace_mode: "text",
            language_id: 436568854u64,
            aliases: &["text proto", "protobuf text format"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl PublicKey {
    pub fn info() -> Language {
        Language {
            name: "Public Key",
            r#type: "data",
            color: "",
            extensions: &[".asc", ".pub"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 298u64,
            aliases: &[],
            codemirror_mode: Some("asciiarmor"),
            codemirror_mime_type: Some("application/pgp"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Pug {
    pub fn info() -> Language {
        Language {
            name: "Pug",
            r#type: "markup",
            color: "#a86454",
            extensions: &[".jade", ".pug"],
            tm_scope: "text.jade",
            ace_mode: "jade",
            language_id: 179u64,
            aliases: &[],
            codemirror_mode: Some("pug"),
            codemirror_mime_type: Some("text/x-pug"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Puppet {
    pub fn info() -> Language {
        Language {
            name: "Puppet",
            r#type: "programming",
            color: "#302B6D",
            extensions: &[".pp"],
            tm_scope: "source.puppet",
            ace_mode: "text",
            language_id: 299u64,
            aliases: &[],
            codemirror_mode: Some("puppet"),
            codemirror_mime_type: Some("text/x-puppet"),
            wrap: None,
            filenames: &["Modulefile"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl PureData {
    pub fn info() -> Language {
        Language {
            name: "Pure Data",
            r#type: "data",
            color: "",
            extensions: &[".pd"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 300u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl PureBasic {
    pub fn info() -> Language {
        Language {
            name: "PureBasic",
            r#type: "programming",
            color: "#5a6986",
            extensions: &[".pb", ".pbi"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 301u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl PureScript {
    pub fn info() -> Language {
        Language {
            name: "PureScript",
            r#type: "programming",
            color: "#1D222D",
            extensions: &[".purs"],
            tm_scope: "source.purescript",
            ace_mode: "haskell",
            language_id: 302u64,
            aliases: &[],
            codemirror_mode: Some("haskell"),
            codemirror_mime_type: Some("text/x-haskell"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Pyret {
    pub fn info() -> Language {
        Language {
            name: "Pyret",
            r#type: "programming",
            color: "#ee1e10",
            extensions: &[".arr"],
            tm_scope: "source.arr",
            ace_mode: "python",
            language_id: 252961827u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Python {
    pub fn info() -> Language {
        Language {
            name: "Python",
            r#type: "programming",
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
            language_id: 303u64,
            aliases: &["python3", "rusthon"],
            codemirror_mode: Some("python"),
            codemirror_mime_type: Some("text/x-python"),
            wrap: None,
            filenames: &[".gclient", "DEPS", "SConscript", "SConstruct", "wscript"],
            group: None,
            interpreters: &["python", "python2", "python3", "py", "pypy", "pypy3"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl PythonConsole {
    pub fn info() -> Language {
        Language {
            name: "Python console",
            r#type: "programming",
            color: "#3572A5",
            extensions: &[],
            tm_scope: "text.python.console",
            ace_mode: "text",
            language_id: 428u64,
            aliases: &["pycon"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: Some("Python"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl PythonTraceback {
    pub fn info() -> Language {
        Language {
            name: "Python traceback",
            r#type: "data",
            color: "#3572A5",
            extensions: &[".pytb"],
            tm_scope: "text.python.traceback",
            ace_mode: "text",
            language_id: 304u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: Some("Python"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Qsharp {
    pub fn info() -> Language {
        Language {
            name: "Q#",
            r#type: "programming",
            color: "#fed659",
            extensions: &[".qs"],
            tm_scope: "source.qsharp",
            ace_mode: "text",
            language_id: 697448245u64,
            aliases: &["qsharp"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl QML {
    pub fn info() -> Language {
        Language {
            name: "QML",
            r#type: "programming",
            color: "#44a51c",
            extensions: &[".qml", ".qbs"],
            tm_scope: "source.qml",
            ace_mode: "text",
            language_id: 305u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl QMake {
    pub fn info() -> Language {
        Language {
            name: "QMake",
            r#type: "programming",
            color: "",
            extensions: &[".pro", ".pri"],
            tm_scope: "source.qmake",
            ace_mode: "text",
            language_id: 306u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["qmake"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl QtScript {
    pub fn info() -> Language {
        Language {
            name: "Qt Script",
            r#type: "programming",
            color: "#00b841",
            extensions: &[".qs"],
            tm_scope: "source.js",
            ace_mode: "javascript",
            language_id: 558193693u64,
            aliases: &[],
            codemirror_mode: Some("javascript"),
            codemirror_mime_type: Some("text/javascript"),
            wrap: None,
            filenames: &["installscript.qs", "toolchain_installscript.qs"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Quake {
    pub fn info() -> Language {
        Language {
            name: "Quake",
            r#type: "programming",
            color: "#882233",
            extensions: &[],
            tm_scope: "source.quake",
            ace_mode: "text",
            language_id: 375265331u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["m3makefile", "m3overrides"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl QuickBASIC {
    pub fn info() -> Language {
        Language {
            name: "QuickBASIC",
            r#type: "programming",
            color: "#008080",
            extensions: &[".bas"],
            tm_scope: "source.QB64",
            ace_mode: "text",
            language_id: 593107205u64,
            aliases: &["qb", "qbasic", "qb64", "classic qbasic", "classic quickbasic"],
            codemirror_mode: Some("vb"),
            codemirror_mime_type: Some("text/x-vb"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl R {
    pub fn info() -> Language {
        Language {
            name: "R",
            r#type: "programming",
            color: "#198CE7",
            extensions: &[".r", ".rd", ".rsx"],
            tm_scope: "source.r",
            ace_mode: "r",
            language_id: 307u64,
            aliases: &["Rscript", "splus"],
            codemirror_mode: Some("r"),
            codemirror_mime_type: Some("text/x-rsrc"),
            wrap: None,
            filenames: &[".Rprofile", "expr-dist"],
            group: None,
            interpreters: &["Rscript"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl RAML {
    pub fn info() -> Language {
        Language {
            name: "RAML",
            r#type: "markup",
            color: "#77d9fb",
            extensions: &[".raml"],
            tm_scope: "source.yaml",
            ace_mode: "yaml",
            language_id: 308u64,
            aliases: &[],
            codemirror_mode: Some("yaml"),
            codemirror_mime_type: Some("text/x-yaml"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl RBS {
    pub fn info() -> Language {
        Language {
            name: "RBS",
            r#type: "data",
            color: "#701516",
            extensions: &[".rbs"],
            tm_scope: "source.rbs",
            ace_mode: "ruby",
            language_id: 899227493u64,
            aliases: &[],
            codemirror_mode: Some("ruby"),
            codemirror_mime_type: Some("text/x-ruby"),
            wrap: None,
            filenames: &[],
            group: Some("Ruby"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl RDoc {
    pub fn info() -> Language {
        Language {
            name: "RDoc",
            r#type: "prose",
            color: "#701516",
            extensions: &[".rdoc"],
            tm_scope: "text.rdoc",
            ace_mode: "rdoc",
            language_id: 309u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: Some(true),
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl REALbasic {
    pub fn info() -> Language {
        Language {
            name: "REALbasic",
            r#type: "programming",
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
            language_id: 310u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl REXX {
    pub fn info() -> Language {
        Language {
            name: "REXX",
            r#type: "programming",
            color: "#d90e09",
            extensions: &[".rexx", ".pprx", ".rex"],
            tm_scope: "source.rexx",
            ace_mode: "text",
            language_id: 311u64,
            aliases: &["arexx"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["regina", "rexx"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl RMarkdown {
    pub fn info() -> Language {
        Language {
            name: "RMarkdown",
            r#type: "prose",
            color: "#198ce7",
            extensions: &[".qmd", ".rmd"],
            tm_scope: "text.md",
            ace_mode: "markdown",
            language_id: 313u64,
            aliases: &[],
            codemirror_mode: Some("gfm"),
            codemirror_mime_type: Some("text/x-gfm"),
            wrap: Some(true),
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl RON {
    pub fn info() -> Language {
        Language {
            name: "RON",
            r#type: "data",
            color: "#a62c00",
            extensions: &[".ron"],
            tm_scope: "source.ron",
            ace_mode: "rust",
            language_id: 587855233u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl RPC {
    pub fn info() -> Language {
        Language {
            name: "RPC",
            r#type: "programming",
            color: "",
            extensions: &[".x"],
            tm_scope: "source.c",
            ace_mode: "c_cpp",
            language_id: 1031374237u64,
            aliases: &["rpcgen", "oncrpc", "xdr"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl RPGLE {
    pub fn info() -> Language {
        Language {
            name: "RPGLE",
            r#type: "programming",
            color: "#2BDE21",
            extensions: &[".rpgle", ".sqlrpgle"],
            tm_scope: "source.rpgle",
            ace_mode: "text",
            language_id: 609977990u64,
            aliases: &["ile rpg", "sqlrpgle"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl RPMSpec {
    pub fn info() -> Language {
        Language {
            name: "RPM Spec",
            r#type: "data",
            color: "",
            extensions: &[".spec"],
            tm_scope: "source.rpm-spec",
            ace_mode: "text",
            language_id: 314u64,
            aliases: &["specfile"],
            codemirror_mode: Some("rpm"),
            codemirror_mime_type: Some("text/x-rpm-spec"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl RUNOFF {
    pub fn info() -> Language {
        Language {
            name: "RUNOFF",
            r#type: "markup",
            color: "#665a4e",
            extensions: &[".rnh", ".rno"],
            tm_scope: "text.runoff",
            ace_mode: "text",
            language_id: 315u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: Some(true),
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Racket {
    pub fn info() -> Language {
        Language {
            name: "Racket",
            r#type: "programming",
            color: "#3c5caa",
            extensions: &[".rkt", ".rktd", ".rktl", ".scrbl"],
            tm_scope: "source.racket",
            ace_mode: "lisp",
            language_id: 316u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["racket"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Ragel {
    pub fn info() -> Language {
        Language {
            name: "Ragel",
            r#type: "programming",
            color: "#9d5200",
            extensions: &[".rl"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 317u64,
            aliases: &["ragel-rb", "ragel-ruby"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Raku {
    pub fn info() -> Language {
        Language {
            name: "Raku",
            r#type: "programming",
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
            language_id: 283u64,
            aliases: &["perl6", "perl-6"],
            codemirror_mode: Some("perl"),
            codemirror_mime_type: Some("text/x-perl"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["perl6", "raku", "rakudo"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Rascal {
    pub fn info() -> Language {
        Language {
            name: "Rascal",
            r#type: "programming",
            color: "#fffaa0",
            extensions: &[".rsc"],
            tm_scope: "source.rascal",
            ace_mode: "text",
            language_id: 173616037u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl RawTokenData {
    pub fn info() -> Language {
        Language {
            name: "Raw token data",
            r#type: "data",
            color: "",
            extensions: &[".raw"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 318u64,
            aliases: &["raw"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ReScript {
    pub fn info() -> Language {
        Language {
            name: "ReScript",
            r#type: "programming",
            color: "#ed5051",
            extensions: &[".res"],
            tm_scope: "source.rescript",
            ace_mode: "rust",
            language_id: 501875647u64,
            aliases: &[],
            codemirror_mode: Some("rust"),
            codemirror_mime_type: Some("text/x-rustsrc"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["ocaml"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ReadlineConfig {
    pub fn info() -> Language {
        Language {
            name: "Readline Config",
            r#type: "data",
            color: "",
            extensions: &[],
            tm_scope: "source.inputrc",
            ace_mode: "text",
            language_id: 538732839u64,
            aliases: &["inputrc", "readline"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[".inputrc", "inputrc"],
            group: Some("INI"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Reason {
    pub fn info() -> Language {
        Language {
            name: "Reason",
            r#type: "programming",
            color: "#ff5847",
            extensions: &[".re", ".rei"],
            tm_scope: "source.reason",
            ace_mode: "rust",
            language_id: 869538413u64,
            aliases: &[],
            codemirror_mode: Some("rust"),
            codemirror_mime_type: Some("text/x-rustsrc"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ReasonLIGO {
    pub fn info() -> Language {
        Language {
            name: "ReasonLIGO",
            r#type: "programming",
            color: "#ff5847",
            extensions: &[".religo"],
            tm_scope: "source.religo",
            ace_mode: "rust",
            language_id: 319002153u64,
            aliases: &[],
            codemirror_mode: Some("rust"),
            codemirror_mime_type: Some("text/x-rustsrc"),
            wrap: None,
            filenames: &[],
            group: Some("LigoLANG"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Rebol {
    pub fn info() -> Language {
        Language {
            name: "Rebol",
            r#type: "programming",
            color: "#358a5b",
            extensions: &[".reb", ".r", ".r2", ".r3", ".rebol"],
            tm_scope: "source.rebol",
            ace_mode: "text",
            language_id: 319u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl RecordJar {
    pub fn info() -> Language {
        Language {
            name: "Record Jar",
            r#type: "data",
            color: "#0673ba",
            extensions: &[],
            tm_scope: "source.record-jar",
            ace_mode: "text",
            language_id: 865765202u64,
            aliases: &[],
            codemirror_mode: Some("properties"),
            codemirror_mime_type: Some("text/x-properties"),
            wrap: None,
            filenames: &["language-subtag-registry.txt"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Red {
    pub fn info() -> Language {
        Language {
            name: "Red",
            r#type: "programming",
            color: "#f50000",
            extensions: &[".red", ".reds"],
            tm_scope: "source.red",
            ace_mode: "text",
            language_id: 320u64,
            aliases: &["red/system"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Redcode {
    pub fn info() -> Language {
        Language {
            name: "Redcode",
            r#type: "programming",
            color: "",
            extensions: &[".cw"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 321u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl RedirectRules {
    pub fn info() -> Language {
        Language {
            name: "Redirect Rules",
            r#type: "data",
            color: "",
            extensions: &[],
            tm_scope: "source.redirects",
            ace_mode: "text",
            language_id: 1020148948u64,
            aliases: &["redirects"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["_redirects"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl RegularExpression {
    pub fn info() -> Language {
        Language {
            name: "Regular Expression",
            r#type: "data",
            color: "#009a00",
            extensions: &[".regexp", ".regex"],
            tm_scope: "source.regexp",
            ace_mode: "text",
            language_id: 363378884u64,
            aliases: &["regexp", "regex"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl RenPy {
    pub fn info() -> Language {
        Language {
            name: "Ren'Py",
            r#type: "programming",
            color: "#ff7f7f",
            extensions: &[".rpy"],
            tm_scope: "source.renpy",
            ace_mode: "python",
            language_id: 322u64,
            aliases: &["renpy"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl RenderScript {
    pub fn info() -> Language {
        Language {
            name: "RenderScript",
            r#type: "programming",
            color: "",
            extensions: &[".rs", ".rsh"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 323u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Rez {
    pub fn info() -> Language {
        Language {
            name: "Rez",
            r#type: "programming",
            color: "#FFDAB3",
            extensions: &[".r"],
            tm_scope: "source.rez",
            ace_mode: "text",
            language_id: 498022874u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl RichTextFormat {
    pub fn info() -> Language {
        Language {
            name: "Rich Text Format",
            r#type: "markup",
            color: "",
            extensions: &[".rtf"],
            tm_scope: "text.rtf",
            ace_mode: "text",
            language_id: 51601661u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Ring {
    pub fn info() -> Language {
        Language {
            name: "Ring",
            r#type: "programming",
            color: "#2D54CB",
            extensions: &[".ring"],
            tm_scope: "source.ring",
            ace_mode: "text",
            language_id: 431u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Riot {
    pub fn info() -> Language {
        Language {
            name: "Riot",
            r#type: "markup",
            color: "#A71E49",
            extensions: &[".riot"],
            tm_scope: "text.html.riot",
            ace_mode: "html",
            language_id: 878396783u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl RobotFramework {
    pub fn info() -> Language {
        Language {
            name: "RobotFramework",
            r#type: "programming",
            color: "#00c0b5",
            extensions: &[".robot", ".resource"],
            tm_scope: "text.robot",
            ace_mode: "text",
            language_id: 324u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Roc {
    pub fn info() -> Language {
        Language {
            name: "Roc",
            r#type: "programming",
            color: "#7c38f5",
            extensions: &[".roc"],
            tm_scope: "source.roc",
            ace_mode: "text",
            language_id: 440182480u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Roff {
    pub fn info() -> Language {
        Language {
            name: "Roff",
            r#type: "markup",
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
            language_id: 141u64,
            aliases: &[
                "groff",
                "man",
                "manpage",
                "man page",
                "man-page",
                "mdoc",
                "nroff",
                "troff",
            ],
            codemirror_mode: Some("troff"),
            codemirror_mime_type: Some("text/troff"),
            wrap: Some(true),
            filenames: &["eqnrc", "mmn", "mmt", "troffrc", "troffrc-end"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl RoffManpage {
    pub fn info() -> Language {
        Language {
            name: "Roff Manpage",
            r#type: "markup",
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
            language_id: 612669833u64,
            aliases: &[],
            codemirror_mode: Some("troff"),
            codemirror_mime_type: Some("text/troff"),
            wrap: Some(true),
            filenames: &[],
            group: Some("Roff"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Rouge {
    pub fn info() -> Language {
        Language {
            name: "Rouge",
            r#type: "programming",
            color: "#cc0088",
            extensions: &[".rg"],
            tm_scope: "source.clojure",
            ace_mode: "clojure",
            language_id: 325u64,
            aliases: &[],
            codemirror_mode: Some("clojure"),
            codemirror_mime_type: Some("text/x-clojure"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl RouterOSScript {
    pub fn info() -> Language {
        Language {
            name: "RouterOS Script",
            r#type: "programming",
            color: "#DE3941",
            extensions: &[".rsc"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 592853203u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["RouterOS"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Ruby {
    pub fn info() -> Language {
        Language {
            name: "Ruby",
            r#type: "programming",
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
            language_id: 326u64,
            aliases: &["jruby", "macruby", "rake", "rb", "rbx"],
            codemirror_mode: Some("ruby"),
            codemirror_mime_type: Some("text/x-ruby"),
            wrap: None,
            filenames: &[
                ".irbrc",
                ".pryrc",
                ".simplecov",
                "Appraisals",
                "Berksfile",
                "Brewfile",
                "Buildfile",
                "Capfile",
                "Dangerfile",
                "Deliverfile",
                "Fastfile",
                "Gemfile",
                "Guardfile",
                "Jarfile",
                "Mavenfile",
                "Podfile",
                "Puppetfile",
                "Rakefile",
                "Snapfile",
                "Steepfile",
                "Thorfile",
                "Vagrantfile",
                "buildfile",
            ],
            group: None,
            interpreters: &["ruby", "macruby", "rake", "jruby", "rbx"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Rust {
    pub fn info() -> Language {
        Language {
            name: "Rust",
            r#type: "programming",
            color: "#dea584",
            extensions: &[".rs", ".rs.in"],
            tm_scope: "source.rust",
            ace_mode: "rust",
            language_id: 327u64,
            aliases: &["rs"],
            codemirror_mode: Some("rust"),
            codemirror_mime_type: Some("text/x-rustsrc"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["rust-script"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl SAS {
    pub fn info() -> Language {
        Language {
            name: "SAS",
            r#type: "programming",
            color: "#B34936",
            extensions: &[".sas"],
            tm_scope: "source.sas",
            ace_mode: "text",
            language_id: 328u64,
            aliases: &[],
            codemirror_mode: Some("sas"),
            codemirror_mime_type: Some("text/x-sas"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl SCSS {
    pub fn info() -> Language {
        Language {
            name: "SCSS",
            r#type: "markup",
            color: "#c6538c",
            extensions: &[".scss"],
            tm_scope: "source.css.scss",
            ace_mode: "scss",
            language_id: 329u64,
            aliases: &[],
            codemirror_mode: Some("css"),
            codemirror_mime_type: Some("text/x-scss"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl SELinuxPolicy {
    pub fn info() -> Language {
        Language {
            name: "SELinux Policy",
            r#type: "data",
            color: "",
            extensions: &[".te"],
            tm_scope: "source.sepolicy",
            ace_mode: "text",
            language_id: 880010326u64,
            aliases: &["SELinux Kernel Policy Language", "sepolicy"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[
                "file_contexts",
                "genfs_contexts",
                "initial_sids",
                "port_contexts",
                "security_classes",
            ],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl SMT {
    pub fn info() -> Language {
        Language {
            name: "SMT",
            r#type: "programming",
            color: "",
            extensions: &[".smt2", ".smt"],
            tm_scope: "source.smt",
            ace_mode: "text",
            language_id: 330u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[
                "boolector",
                "cvc4",
                "mathsat5",
                "opensmt",
                "smtinterpol",
                "smt-rat",
                "stp",
                "verit",
                "yices2",
                "z3",
            ],
            fs_name: None,
            searchable: None,
        }
    }
}
impl SPARQL {
    pub fn info() -> Language {
        Language {
            name: "SPARQL",
            r#type: "data",
            color: "#0C4597",
            extensions: &[".sparql", ".rq"],
            tm_scope: "source.sparql",
            ace_mode: "text",
            language_id: 331u64,
            aliases: &[],
            codemirror_mode: Some("sparql"),
            codemirror_mime_type: Some("application/sparql-query"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl SQF {
    pub fn info() -> Language {
        Language {
            name: "SQF",
            r#type: "programming",
            color: "#3F3F3F",
            extensions: &[".sqf", ".hqf"],
            tm_scope: "source.sqf",
            ace_mode: "text",
            language_id: 332u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl SQL {
    pub fn info() -> Language {
        Language {
            name: "SQL",
            r#type: "data",
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
            language_id: 333u64,
            aliases: &[],
            codemirror_mode: Some("sql"),
            codemirror_mime_type: Some("text/x-sql"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl SQLPL {
    pub fn info() -> Language {
        Language {
            name: "SQLPL",
            r#type: "programming",
            color: "#e38c00",
            extensions: &[".sql", ".db2"],
            tm_scope: "source.sql",
            ace_mode: "sql",
            language_id: 334u64,
            aliases: &[],
            codemirror_mode: Some("sql"),
            codemirror_mime_type: Some("text/x-sql"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl SRecodeTemplate {
    pub fn info() -> Language {
        Language {
            name: "SRecode Template",
            r#type: "markup",
            color: "#348a34",
            extensions: &[".srt"],
            tm_scope: "source.lisp",
            ace_mode: "lisp",
            language_id: 335u64,
            aliases: &[],
            codemirror_mode: Some("commonlisp"),
            codemirror_mime_type: Some("text/x-common-lisp"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl SSHConfig {
    pub fn info() -> Language {
        Language {
            name: "SSH Config",
            r#type: "data",
            color: "",
            extensions: &[],
            tm_scope: "source.ssh-config",
            ace_mode: "text",
            language_id: 554920715u64,
            aliases: &["sshconfig", "sshdconfig", "ssh_config", "sshd_config"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[
                "ssh-config",
                "ssh_config",
                "sshconfig",
                "sshconfig.snip",
                "sshd-config",
                "sshd_config",
            ],
            group: Some("INI"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl STAR {
    pub fn info() -> Language {
        Language {
            name: "STAR",
            r#type: "data",
            color: "",
            extensions: &[".star"],
            tm_scope: "source.star",
            ace_mode: "text",
            language_id: 424510560u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl STL {
    pub fn info() -> Language {
        Language {
            name: "STL",
            r#type: "data",
            color: "#373b5e",
            extensions: &[".stl"],
            tm_scope: "source.stl",
            ace_mode: "text",
            language_id: 455361735u64,
            aliases: &["ascii stl", "stla"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl STON {
    pub fn info() -> Language {
        Language {
            name: "STON",
            r#type: "data",
            color: "",
            extensions: &[".ston"],
            tm_scope: "source.smalltalk",
            ace_mode: "text",
            language_id: 336u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: Some("Smalltalk"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl SVG {
    pub fn info() -> Language {
        Language {
            name: "SVG",
            r#type: "data",
            color: "#ff9900",
            extensions: &[".svg"],
            tm_scope: "text.xml.svg",
            ace_mode: "xml",
            language_id: 337u64,
            aliases: &[],
            codemirror_mode: Some("xml"),
            codemirror_mime_type: Some("text/xml"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl SWIG {
    pub fn info() -> Language {
        Language {
            name: "SWIG",
            r#type: "programming",
            color: "",
            extensions: &[".i"],
            tm_scope: "source.c++",
            ace_mode: "c_cpp",
            language_id: 1066250075u64,
            aliases: &[],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-c++src"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Sage {
    pub fn info() -> Language {
        Language {
            name: "Sage",
            r#type: "programming",
            color: "",
            extensions: &[".sage", ".sagews"],
            tm_scope: "source.python",
            ace_mode: "python",
            language_id: 338u64,
            aliases: &[],
            codemirror_mode: Some("python"),
            codemirror_mime_type: Some("text/x-python"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl SaltStack {
    pub fn info() -> Language {
        Language {
            name: "SaltStack",
            r#type: "programming",
            color: "#646464",
            extensions: &[".sls"],
            tm_scope: "source.yaml.salt",
            ace_mode: "yaml",
            language_id: 339u64,
            aliases: &["saltstate", "salt"],
            codemirror_mode: Some("yaml"),
            codemirror_mime_type: Some("text/x-yaml"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Sass {
    pub fn info() -> Language {
        Language {
            name: "Sass",
            r#type: "markup",
            color: "#a53b70",
            extensions: &[".sass"],
            tm_scope: "source.sass",
            ace_mode: "sass",
            language_id: 340u64,
            aliases: &[],
            codemirror_mode: Some("sass"),
            codemirror_mime_type: Some("text/x-sass"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Scala {
    pub fn info() -> Language {
        Language {
            name: "Scala",
            r#type: "programming",
            color: "#c22d40",
            extensions: &[".scala", ".kojo", ".sbt", ".sc"],
            tm_scope: "source.scala",
            ace_mode: "scala",
            language_id: 341u64,
            aliases: &[],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-scala"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["scala"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Scaml {
    pub fn info() -> Language {
        Language {
            name: "Scaml",
            r#type: "markup",
            color: "#bd181a",
            extensions: &[".scaml"],
            tm_scope: "source.scaml",
            ace_mode: "text",
            language_id: 342u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Scenic {
    pub fn info() -> Language {
        Language {
            name: "Scenic",
            r#type: "programming",
            color: "#fdc700",
            extensions: &[".scenic"],
            tm_scope: "source.scenic",
            ace_mode: "text",
            language_id: 619814037u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["scenic"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Scheme {
    pub fn info() -> Language {
        Language {
            name: "Scheme",
            r#type: "programming",
            color: "#1e4aec",
            extensions: &[".scm", ".sch", ".sld", ".sls", ".sps", ".ss"],
            tm_scope: "source.scheme",
            ace_mode: "scheme",
            language_id: 343u64,
            aliases: &[],
            codemirror_mode: Some("scheme"),
            codemirror_mime_type: Some("text/x-scheme"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[
                "scheme",
                "guile",
                "bigloo",
                "chicken",
                "csi",
                "gosh",
                "r6rs",
            ],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Scilab {
    pub fn info() -> Language {
        Language {
            name: "Scilab",
            r#type: "programming",
            color: "#ca0f21",
            extensions: &[".sci", ".sce", ".tst"],
            tm_scope: "source.scilab",
            ace_mode: "text",
            language_id: 344u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl _Self {
    pub fn info() -> Language {
        Language {
            name: "Self",
            r#type: "programming",
            color: "#0579aa",
            extensions: &[".self"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 345u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ShaderLab {
    pub fn info() -> Language {
        Language {
            name: "ShaderLab",
            r#type: "programming",
            color: "#222c37",
            extensions: &[".shader"],
            tm_scope: "source.shaderlab",
            ace_mode: "text",
            language_id: 664257356u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Shell {
    pub fn info() -> Language {
        Language {
            name: "Shell",
            r#type: "programming",
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
            language_id: 346u64,
            aliases: &["sh", "shell-script", "bash", "zsh", "envrc"],
            codemirror_mode: Some("shell"),
            codemirror_mime_type: Some("text/x-sh"),
            wrap: None,
            filenames: &[
                ".bash_aliases",
                ".bash_functions",
                ".bash_history",
                ".bash_logout",
                ".bash_profile",
                ".bashrc",
                ".cshrc",
                ".envrc",
                ".flaskenv",
                ".kshrc",
                ".login",
                ".profile",
                ".tmux.conf",
                ".zlogin",
                ".zlogout",
                ".zprofile",
                ".zshenv",
                ".zshrc",
                "9fs",
                "PKGBUILD",
                "bash_aliases",
                "bash_logout",
                "bash_profile",
                "bashrc",
                "cshrc",
                "gradlew",
                "kshrc",
                "login",
                "man",
                "profile",
                "tmux.conf",
                "zlogin",
                "zlogout",
                "zprofile",
                "zshenv",
                "zshrc",
            ],
            group: None,
            interpreters: &[
                "ash",
                "bash",
                "dash",
                "ksh",
                "mksh",
                "pdksh",
                "rc",
                "sh",
                "zsh",
            ],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ShellCheckConfig {
    pub fn info() -> Language {
        Language {
            name: "ShellCheck Config",
            r#type: "data",
            color: "#cecfcb",
            extensions: &[],
            tm_scope: "source.shellcheckrc",
            ace_mode: "ini",
            language_id: 687511714u64,
            aliases: &["shellcheckrc"],
            codemirror_mode: Some("properties"),
            codemirror_mime_type: Some("text/x-properties"),
            wrap: None,
            filenames: &[".shellcheckrc"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ShellSession {
    pub fn info() -> Language {
        Language {
            name: "ShellSession",
            r#type: "programming",
            color: "",
            extensions: &[".sh-session"],
            tm_scope: "text.shell-session",
            ace_mode: "sh",
            language_id: 347u64,
            aliases: &["bash session", "console"],
            codemirror_mode: Some("shell"),
            codemirror_mime_type: Some("text/x-sh"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Shen {
    pub fn info() -> Language {
        Language {
            name: "Shen",
            r#type: "programming",
            color: "#120F14",
            extensions: &[".shen"],
            tm_scope: "source.shen",
            ace_mode: "text",
            language_id: 348u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Sieve {
    pub fn info() -> Language {
        Language {
            name: "Sieve",
            r#type: "programming",
            color: "",
            extensions: &[".sieve"],
            tm_scope: "source.sieve",
            ace_mode: "text",
            language_id: 208976687u64,
            aliases: &[],
            codemirror_mode: Some("sieve"),
            codemirror_mime_type: Some("application/sieve"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl SimpleFileVerification {
    pub fn info() -> Language {
        Language {
            name: "Simple File Verification",
            r#type: "data",
            color: "#C9BFED",
            extensions: &[".sfv"],
            tm_scope: "source.sfv",
            ace_mode: "ini",
            language_id: 735623761u64,
            aliases: &["sfv"],
            codemirror_mode: Some("properties"),
            codemirror_mime_type: Some("text/x-properties"),
            wrap: None,
            filenames: &[],
            group: Some("Checksums"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Singularity {
    pub fn info() -> Language {
        Language {
            name: "Singularity",
            r#type: "programming",
            color: "#64E6AD",
            extensions: &[],
            tm_scope: "source.singularity",
            ace_mode: "text",
            language_id: 987024632u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["Singularity"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Slash {
    pub fn info() -> Language {
        Language {
            name: "Slash",
            r#type: "programming",
            color: "#007eff",
            extensions: &[".sl"],
            tm_scope: "text.html.slash",
            ace_mode: "text",
            language_id: 349u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Slice {
    pub fn info() -> Language {
        Language {
            name: "Slice",
            r#type: "programming",
            color: "#003fa2",
            extensions: &[".ice"],
            tm_scope: "source.ice",
            ace_mode: "text",
            language_id: 894641667u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Slim {
    pub fn info() -> Language {
        Language {
            name: "Slim",
            r#type: "markup",
            color: "#2b2b2b",
            extensions: &[".slim"],
            tm_scope: "text.slim",
            ace_mode: "text",
            language_id: 350u64,
            aliases: &[],
            codemirror_mode: Some("slim"),
            codemirror_mime_type: Some("text/x-slim"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Slint {
    pub fn info() -> Language {
        Language {
            name: "Slint",
            r#type: "markup",
            color: "#2379F4",
            extensions: &[".slint"],
            tm_scope: "source.slint",
            ace_mode: "text",
            language_id: 119900149u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl SmPL {
    pub fn info() -> Language {
        Language {
            name: "SmPL",
            r#type: "programming",
            color: "#c94949",
            extensions: &[".cocci"],
            tm_scope: "source.smpl",
            ace_mode: "text",
            language_id: 164123055u64,
            aliases: &["coccinelle"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Smali {
    pub fn info() -> Language {
        Language {
            name: "Smali",
            r#type: "programming",
            color: "",
            extensions: &[".smali"],
            tm_scope: "source.smali",
            ace_mode: "text",
            language_id: 351u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Smalltalk {
    pub fn info() -> Language {
        Language {
            name: "Smalltalk",
            r#type: "programming",
            color: "#596706",
            extensions: &[".st", ".cs"],
            tm_scope: "source.smalltalk",
            ace_mode: "text",
            language_id: 352u64,
            aliases: &["squeak"],
            codemirror_mode: Some("smalltalk"),
            codemirror_mime_type: Some("text/x-stsrc"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Smarty {
    pub fn info() -> Language {
        Language {
            name: "Smarty",
            r#type: "programming",
            color: "#f0c040",
            extensions: &[".tpl"],
            tm_scope: "text.html.smarty",
            ace_mode: "smarty",
            language_id: 353u64,
            aliases: &[],
            codemirror_mode: Some("smarty"),
            codemirror_mime_type: Some("text/x-smarty"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Smithy {
    pub fn info() -> Language {
        Language {
            name: "Smithy",
            r#type: "programming",
            color: "#c44536",
            extensions: &[".smithy"],
            tm_scope: "source.smithy",
            ace_mode: "text",
            language_id: 1027892786u64,
            aliases: &[],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-csrc"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Snakemake {
    pub fn info() -> Language {
        Language {
            name: "Snakemake",
            r#type: "programming",
            color: "#419179",
            extensions: &[".smk", ".snakefile"],
            tm_scope: "source.python",
            ace_mode: "python",
            language_id: 151241392u64,
            aliases: &["snakefile"],
            codemirror_mode: Some("python"),
            codemirror_mime_type: Some("text/x-python"),
            wrap: None,
            filenames: &["Snakefile"],
            group: Some("Python"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Solidity {
    pub fn info() -> Language {
        Language {
            name: "Solidity",
            r#type: "programming",
            color: "#AA6746",
            extensions: &[".sol"],
            tm_scope: "source.solidity",
            ace_mode: "text",
            language_id: 237469032u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Soong {
    pub fn info() -> Language {
        Language {
            name: "Soong",
            r#type: "data",
            color: "",
            extensions: &[],
            tm_scope: "source.bp",
            ace_mode: "text",
            language_id: 222900098u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["Android.bp"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl SourcePawn {
    pub fn info() -> Language {
        Language {
            name: "SourcePawn",
            r#type: "programming",
            color: "#f69e1d",
            extensions: &[".sp", ".inc"],
            tm_scope: "source.sourcepawn",
            ace_mode: "text",
            language_id: 354u64,
            aliases: &["sourcemod"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl SplineFontDatabase {
    pub fn info() -> Language {
        Language {
            name: "Spline Font Database",
            r#type: "data",
            color: "",
            extensions: &[".sfd"],
            tm_scope: "text.sfd",
            ace_mode: "yaml",
            language_id: 767169629u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Squirrel {
    pub fn info() -> Language {
        Language {
            name: "Squirrel",
            r#type: "programming",
            color: "#800000",
            extensions: &[".nut"],
            tm_scope: "source.nut",
            ace_mode: "c_cpp",
            language_id: 355u64,
            aliases: &[],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-c++src"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Stan {
    pub fn info() -> Language {
        Language {
            name: "Stan",
            r#type: "programming",
            color: "#b2011d",
            extensions: &[".stan"],
            tm_scope: "source.stan",
            ace_mode: "text",
            language_id: 356u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl StandardML {
    pub fn info() -> Language {
        Language {
            name: "Standard ML",
            r#type: "programming",
            color: "#dc566d",
            extensions: &[".ml", ".fun", ".sig", ".sml"],
            tm_scope: "source.ml",
            ace_mode: "text",
            language_id: 357u64,
            aliases: &["sml"],
            codemirror_mode: Some("mllike"),
            codemirror_mime_type: Some("text/x-ocaml"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Starlark {
    pub fn info() -> Language {
        Language {
            name: "Starlark",
            r#type: "programming",
            color: "#76d275",
            extensions: &[".bzl", ".star"],
            tm_scope: "source.python",
            ace_mode: "python",
            language_id: 960266174u64,
            aliases: &["bazel", "bzl"],
            codemirror_mode: Some("python"),
            codemirror_mime_type: Some("text/x-python"),
            wrap: None,
            filenames: &[
                "BUCK",
                "BUILD",
                "BUILD.bazel",
                "MODULE.bazel",
                "Tiltfile",
                "WORKSPACE",
                "WORKSPACE.bazel",
                "WORKSPACE.bzlmod",
            ],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Stata {
    pub fn info() -> Language {
        Language {
            name: "Stata",
            r#type: "programming",
            color: "#1a5f91",
            extensions: &[".do", ".ado", ".doh", ".ihlp", ".mata", ".matah", ".sthlp"],
            tm_scope: "source.stata",
            ace_mode: "text",
            language_id: 358u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl StringTemplate {
    pub fn info() -> Language {
        Language {
            name: "StringTemplate",
            r#type: "markup",
            color: "#3fb34f",
            extensions: &[".st"],
            tm_scope: "source.string-template",
            ace_mode: "html",
            language_id: 89855901u64,
            aliases: &[],
            codemirror_mode: Some("htmlmixed"),
            codemirror_mime_type: Some("text/html"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Stylus {
    pub fn info() -> Language {
        Language {
            name: "Stylus",
            r#type: "markup",
            color: "#ff6347",
            extensions: &[".styl"],
            tm_scope: "source.stylus",
            ace_mode: "stylus",
            language_id: 359u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl SubRipText {
    pub fn info() -> Language {
        Language {
            name: "SubRip Text",
            r#type: "data",
            color: "#9e0101",
            extensions: &[".srt"],
            tm_scope: "text.srt",
            ace_mode: "text",
            language_id: 360u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl SugarSS {
    pub fn info() -> Language {
        Language {
            name: "SugarSS",
            r#type: "markup",
            color: "#2fcc9f",
            extensions: &[".sss"],
            tm_scope: "source.css.postcss.sugarss",
            ace_mode: "text",
            language_id: 826404698u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl SuperCollider {
    pub fn info() -> Language {
        Language {
            name: "SuperCollider",
            r#type: "programming",
            color: "#46390b",
            extensions: &[".sc", ".scd"],
            tm_scope: "source.supercollider",
            ace_mode: "text",
            language_id: 361u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["sclang", "scsynth"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl SurvexData {
    pub fn info() -> Language {
        Language {
            name: "Survex data",
            r#type: "data",
            color: "#ffcc99",
            extensions: &[".svx"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 24470517u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Svelte {
    pub fn info() -> Language {
        Language {
            name: "Svelte",
            r#type: "markup",
            color: "#ff3e00",
            extensions: &[".svelte"],
            tm_scope: "source.svelte",
            ace_mode: "html",
            language_id: 928734530u64,
            aliases: &[],
            codemirror_mode: Some("htmlmixed"),
            codemirror_mime_type: Some("text/html"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Sway {
    pub fn info() -> Language {
        Language {
            name: "Sway",
            r#type: "programming",
            color: "#00F58C",
            extensions: &[".sw"],
            tm_scope: "source.sway",
            ace_mode: "rust",
            language_id: 271471144u64,
            aliases: &[],
            codemirror_mode: Some("rust"),
            codemirror_mime_type: Some("text/x-rustsrc"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Sweave {
    pub fn info() -> Language {
        Language {
            name: "Sweave",
            r#type: "prose",
            color: "#198ce7",
            extensions: &[".rnw"],
            tm_scope: "text.tex.latex.sweave",
            ace_mode: "tex",
            language_id: 558779190u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Swift {
    pub fn info() -> Language {
        Language {
            name: "Swift",
            r#type: "programming",
            color: "#F05138",
            extensions: &[".swift"],
            tm_scope: "source.swift",
            ace_mode: "text",
            language_id: 362u64,
            aliases: &[],
            codemirror_mode: Some("swift"),
            codemirror_mime_type: Some("text/x-swift"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl SystemVerilog {
    pub fn info() -> Language {
        Language {
            name: "SystemVerilog",
            r#type: "programming",
            color: "#DAE1C2",
            extensions: &[".sv", ".svh", ".vh"],
            tm_scope: "source.systemverilog",
            ace_mode: "verilog",
            language_id: 363u64,
            aliases: &[],
            codemirror_mode: Some("verilog"),
            codemirror_mime_type: Some("text/x-systemverilog"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl TIProgram {
    pub fn info() -> Language {
        Language {
            name: "TI Program",
            r#type: "programming",
            color: "#A0AA87",
            extensions: &[".8xp", ".8xp.txt"],
            tm_scope: "source.8xp",
            ace_mode: "text",
            language_id: 422u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl TLVerilog {
    pub fn info() -> Language {
        Language {
            name: "TL-Verilog",
            r#type: "programming",
            color: "#C40023",
            extensions: &[".tlv"],
            tm_scope: "source.tlverilog",
            ace_mode: "verilog",
            language_id: 118656070u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl TLA {
    pub fn info() -> Language {
        Language {
            name: "TLA",
            r#type: "programming",
            color: "#4b0079",
            extensions: &[".tla"],
            tm_scope: "source.tla",
            ace_mode: "text",
            language_id: 364u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl TOML {
    pub fn info() -> Language {
        Language {
            name: "TOML",
            r#type: "data",
            color: "#9c4221",
            extensions: &[".toml"],
            tm_scope: "source.toml",
            ace_mode: "toml",
            language_id: 365u64,
            aliases: &[],
            codemirror_mode: Some("toml"),
            codemirror_mime_type: Some("text/x-toml"),
            wrap: None,
            filenames: &[
                "Cargo.lock",
                "Cargo.toml.orig",
                "Gopkg.lock",
                "Pipfile",
                "pdm.lock",
                "poetry.lock",
                "uv.lock",
            ],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl TSPLIBData {
    pub fn info() -> Language {
        Language {
            name: "TSPLIB data",
            r#type: "data",
            color: "",
            extensions: &[".tsp"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 89289301u64,
            aliases: &["travelling salesman problem", "traveling salesman problem"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl TSQL {
    pub fn info() -> Language {
        Language {
            name: "TSQL",
            r#type: "programming",
            color: "#e38c00",
            extensions: &[".sql"],
            tm_scope: "source.tsql",
            ace_mode: "sql",
            language_id: 918334941u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl TSV {
    pub fn info() -> Language {
        Language {
            name: "TSV",
            r#type: "data",
            color: "#237346",
            extensions: &[".tsv", ".vcf"],
            tm_scope: "source.generic-db",
            ace_mode: "text",
            language_id: 1035892117u64,
            aliases: &["tab-seperated values"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl TSX {
    pub fn info() -> Language {
        Language {
            name: "TSX",
            r#type: "programming",
            color: "#3178c6",
            extensions: &[".tsx"],
            tm_scope: "source.tsx",
            ace_mode: "javascript",
            language_id: 94901924u64,
            aliases: &[],
            codemirror_mode: Some("jsx"),
            codemirror_mime_type: Some("text/jsx"),
            wrap: None,
            filenames: &[],
            group: Some("TypeScript"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl TXL {
    pub fn info() -> Language {
        Language {
            name: "TXL",
            r#type: "programming",
            color: "#0178b8",
            extensions: &[".txl"],
            tm_scope: "source.txl",
            ace_mode: "text",
            language_id: 366u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Tact {
    pub fn info() -> Language {
        Language {
            name: "Tact",
            r#type: "programming",
            color: "#48b5ff",
            extensions: &[".tact"],
            tm_scope: "source.tact",
            ace_mode: "text",
            language_id: 606708469u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Talon {
    pub fn info() -> Language {
        Language {
            name: "Talon",
            r#type: "programming",
            color: "#333333",
            extensions: &[".talon"],
            tm_scope: "source.talon",
            ace_mode: "text",
            language_id: 959889508u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Tcl {
    pub fn info() -> Language {
        Language {
            name: "Tcl",
            r#type: "programming",
            color: "#e4cc98",
            extensions: &[".tcl", ".adp", ".sdc", ".tcl.in", ".tm", ".xdc"],
            tm_scope: "source.tcl",
            ace_mode: "tcl",
            language_id: 367u64,
            aliases: &["sdc", "xdc"],
            codemirror_mode: Some("tcl"),
            codemirror_mime_type: Some("text/x-tcl"),
            wrap: None,
            filenames: &["owh", "starfield"],
            group: None,
            interpreters: &["tclsh", "wish"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Tcsh {
    pub fn info() -> Language {
        Language {
            name: "Tcsh",
            r#type: "programming",
            color: "",
            extensions: &[".tcsh", ".csh"],
            tm_scope: "source.shell",
            ace_mode: "sh",
            language_id: 368u64,
            aliases: &[],
            codemirror_mode: Some("shell"),
            codemirror_mime_type: Some("text/x-sh"),
            wrap: None,
            filenames: &[],
            group: Some("Shell"),
            interpreters: &["tcsh", "csh"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl TeX {
    pub fn info() -> Language {
        Language {
            name: "TeX",
            r#type: "markup",
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
            language_id: 369u64,
            aliases: &["latex"],
            codemirror_mode: Some("stex"),
            codemirror_mime_type: Some("text/x-stex"),
            wrap: Some(true),
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Tea {
    pub fn info() -> Language {
        Language {
            name: "Tea",
            r#type: "markup",
            color: "",
            extensions: &[".tea"],
            tm_scope: "source.tea",
            ace_mode: "text",
            language_id: 370u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Terra {
    pub fn info() -> Language {
        Language {
            name: "Terra",
            r#type: "programming",
            color: "#00004c",
            extensions: &[".t"],
            tm_scope: "source.terra",
            ace_mode: "lua",
            language_id: 371u64,
            aliases: &[],
            codemirror_mode: Some("lua"),
            codemirror_mime_type: Some("text/x-lua"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["lua"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl TerraformTemplate {
    pub fn info() -> Language {
        Language {
            name: "Terraform Template",
            r#type: "markup",
            color: "#7b42bb",
            extensions: &[".tftpl"],
            tm_scope: "source.hcl.terraform",
            ace_mode: "ruby",
            language_id: 856832701u64,
            aliases: &[],
            codemirror_mode: Some("ruby"),
            codemirror_mime_type: Some("text/x-ruby"),
            wrap: None,
            filenames: &[],
            group: Some("HCL"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Texinfo {
    pub fn info() -> Language {
        Language {
            name: "Texinfo",
            r#type: "prose",
            color: "",
            extensions: &[".texinfo", ".texi", ".txi"],
            tm_scope: "text.texinfo",
            ace_mode: "text",
            language_id: 988020015u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: Some(true),
            filenames: &[],
            group: None,
            interpreters: &["makeinfo"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Text {
    pub fn info() -> Language {
        Language {
            name: "Text",
            r#type: "prose",
            color: "",
            extensions: &[".txt", ".fr", ".nb", ".ncl", ".no"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 372u64,
            aliases: &["fundamental", "plain text"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: Some(true),
            filenames: &[
                "CITATION",
                "CITATIONS",
                "COPYING",
                "COPYING.regex",
                "COPYRIGHT.regex",
                "FONTLOG",
                "INSTALL",
                "INSTALL.mysql",
                "LICENSE",
                "LICENSE.mysql",
                "NEWS",
                "README.me",
                "README.mysql",
                "README.nss",
                "click.me",
                "delete.me",
                "keep.me",
                "package.mask",
                "package.use.mask",
                "package.use.stable.mask",
                "read.me",
                "readme.1st",
                "test.me",
                "use.mask",
                "use.stable.mask",
            ],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl TextGrid {
    pub fn info() -> Language {
        Language {
            name: "TextGrid",
            r#type: "data",
            color: "#c8506d",
            extensions: &[".TextGrid"],
            tm_scope: "source.textgrid",
            ace_mode: "text",
            language_id: 965696054u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl TextMateProperties {
    pub fn info() -> Language {
        Language {
            name: "TextMate Properties",
            r#type: "data",
            color: "#df66e4",
            extensions: &[],
            tm_scope: "source.tm-properties",
            ace_mode: "properties",
            language_id: 981795023u64,
            aliases: &["tm-properties"],
            codemirror_mode: Some("properties"),
            codemirror_mime_type: Some("text/x-properties"),
            wrap: None,
            filenames: &[".tm_properties"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Textile {
    pub fn info() -> Language {
        Language {
            name: "Textile",
            r#type: "prose",
            color: "#ffe7ac",
            extensions: &[".textile"],
            tm_scope: "none",
            ace_mode: "textile",
            language_id: 373u64,
            aliases: &[],
            codemirror_mode: Some("textile"),
            codemirror_mime_type: Some("text/x-textile"),
            wrap: Some(true),
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Thrift {
    pub fn info() -> Language {
        Language {
            name: "Thrift",
            r#type: "programming",
            color: "#D12127",
            extensions: &[".thrift"],
            tm_scope: "source.thrift",
            ace_mode: "text",
            language_id: 374u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Toit {
    pub fn info() -> Language {
        Language {
            name: "Toit",
            r#type: "programming",
            color: "#c2c9fb",
            extensions: &[".toit"],
            tm_scope: "source.toit",
            ace_mode: "text",
            language_id: 356554395u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Turing {
    pub fn info() -> Language {
        Language {
            name: "Turing",
            r#type: "programming",
            color: "#cf142b",
            extensions: &[".t", ".tu"],
            tm_scope: "source.turing",
            ace_mode: "text",
            language_id: 375u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Turtle {
    pub fn info() -> Language {
        Language {
            name: "Turtle",
            r#type: "data",
            color: "",
            extensions: &[".ttl"],
            tm_scope: "source.turtle",
            ace_mode: "text",
            language_id: 376u64,
            aliases: &[],
            codemirror_mode: Some("turtle"),
            codemirror_mime_type: Some("text/turtle"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Twig {
    pub fn info() -> Language {
        Language {
            name: "Twig",
            r#type: "markup",
            color: "#c1d026",
            extensions: &[".twig"],
            tm_scope: "text.html.twig",
            ace_mode: "twig",
            language_id: 377u64,
            aliases: &[],
            codemirror_mode: Some("twig"),
            codemirror_mime_type: Some("text/x-twig"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl TypeLanguage {
    pub fn info() -> Language {
        Language {
            name: "Type Language",
            r#type: "data",
            color: "",
            extensions: &[".tl"],
            tm_scope: "source.tl",
            ace_mode: "text",
            language_id: 632765617u64,
            aliases: &["tl"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl TypeScript {
    pub fn info() -> Language {
        Language {
            name: "TypeScript",
            r#type: "programming",
            color: "#3178c6",
            extensions: &[".ts", ".cts", ".mts"],
            tm_scope: "source.ts",
            ace_mode: "typescript",
            language_id: 378u64,
            aliases: &["ts"],
            codemirror_mode: Some("javascript"),
            codemirror_mime_type: Some("application/typescript"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["deno", "ts-node", "tsx"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl TypeSpec {
    pub fn info() -> Language {
        Language {
            name: "TypeSpec",
            r#type: "programming",
            color: "#4A3665",
            extensions: &[".tsp"],
            tm_scope: "source.tsp",
            ace_mode: "text",
            language_id: 952272597u64,
            aliases: &["tsp"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Typst {
    pub fn info() -> Language {
        Language {
            name: "Typst",
            r#type: "programming",
            color: "#239dad",
            extensions: &[".typ"],
            tm_scope: "source.typst",
            ace_mode: "text",
            language_id: 704730682u64,
            aliases: &["typ"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl UnifiedParallelC {
    pub fn info() -> Language {
        Language {
            name: "Unified Parallel C",
            r#type: "programming",
            color: "#4e3617",
            extensions: &[".upc"],
            tm_scope: "source.c",
            ace_mode: "c_cpp",
            language_id: 379u64,
            aliases: &[],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-csrc"),
            wrap: None,
            filenames: &[],
            group: Some("C"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Unity3DAsset {
    pub fn info() -> Language {
        Language {
            name: "Unity3D Asset",
            r#type: "data",
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
            language_id: 380u64,
            aliases: &[],
            codemirror_mode: Some("yaml"),
            codemirror_mime_type: Some("text/x-yaml"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl UnixAssembly {
    pub fn info() -> Language {
        Language {
            name: "Unix Assembly",
            r#type: "programming",
            color: "",
            extensions: &[".s", ".ms"],
            tm_scope: "source.x86",
            ace_mode: "assembly_x86",
            language_id: 120u64,
            aliases: &["gas", "gnu asm", "unix asm"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: Some("Assembly"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Uno {
    pub fn info() -> Language {
        Language {
            name: "Uno",
            r#type: "programming",
            color: "#9933cc",
            extensions: &[".uno"],
            tm_scope: "source.cs",
            ace_mode: "csharp",
            language_id: 381u64,
            aliases: &[],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-csharp"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl UnrealScript {
    pub fn info() -> Language {
        Language {
            name: "UnrealScript",
            r#type: "programming",
            color: "#a54c4d",
            extensions: &[".uc"],
            tm_scope: "source.java",
            ace_mode: "java",
            language_id: 382u64,
            aliases: &[],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-java"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl UrWeb {
    pub fn info() -> Language {
        Language {
            name: "UrWeb",
            r#type: "programming",
            color: "#ccccee",
            extensions: &[".ur", ".urs"],
            tm_scope: "source.ur",
            ace_mode: "text",
            language_id: 383u64,
            aliases: &["Ur/Web", "Ur"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl V {
    pub fn info() -> Language {
        Language {
            name: "V",
            r#type: "programming",
            color: "#4f87c4",
            extensions: &[".v"],
            tm_scope: "source.v",
            ace_mode: "golang",
            language_id: 603371597u64,
            aliases: &["vlang"],
            codemirror_mode: Some("go"),
            codemirror_mime_type: Some("text/x-go"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl VBA {
    pub fn info() -> Language {
        Language {
            name: "VBA",
            r#type: "programming",
            color: "#867db1",
            extensions: &[".bas", ".cls", ".frm", ".vba"],
            tm_scope: "source.vba",
            ace_mode: "text",
            language_id: 399230729u64,
            aliases: &["visual basic for applications"],
            codemirror_mode: Some("vb"),
            codemirror_mime_type: Some("text/x-vb"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl VBScript {
    pub fn info() -> Language {
        Language {
            name: "VBScript",
            r#type: "programming",
            color: "#15dcdc",
            extensions: &[".vbs"],
            tm_scope: "source.vbnet",
            ace_mode: "text",
            language_id: 408016005u64,
            aliases: &[],
            codemirror_mode: Some("vbscript"),
            codemirror_mime_type: Some("text/vbscript"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl VCL {
    pub fn info() -> Language {
        Language {
            name: "VCL",
            r#type: "programming",
            color: "#148AA8",
            extensions: &[".vcl"],
            tm_scope: "source.varnish.vcl",
            ace_mode: "text",
            language_id: 384u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl VHDL {
    pub fn info() -> Language {
        Language {
            name: "VHDL",
            r#type: "programming",
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
            language_id: 385u64,
            aliases: &[],
            codemirror_mode: Some("vhdl"),
            codemirror_mime_type: Some("text/x-vhdl"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Vala {
    pub fn info() -> Language {
        Language {
            name: "Vala",
            r#type: "programming",
            color: "#a56de2",
            extensions: &[".vala", ".vapi"],
            tm_scope: "source.vala",
            ace_mode: "vala",
            language_id: 386u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ValveDataFormat {
    pub fn info() -> Language {
        Language {
            name: "Valve Data Format",
            r#type: "data",
            color: "#f26025",
            extensions: &[".vdf"],
            tm_scope: "source.keyvalues",
            ace_mode: "text",
            language_id: 544060961u64,
            aliases: &["keyvalues", "vdf"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl VelocityTemplateLanguage {
    pub fn info() -> Language {
        Language {
            name: "Velocity Template Language",
            r#type: "markup",
            color: "#507cff",
            extensions: &[".vtl"],
            tm_scope: "source.velocity",
            ace_mode: "velocity",
            language_id: 292377326u64,
            aliases: &["vtl", "velocity"],
            codemirror_mode: Some("velocity"),
            codemirror_mime_type: Some("text/velocity"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Verilog {
    pub fn info() -> Language {
        Language {
            name: "Verilog",
            r#type: "programming",
            color: "#b2b7f8",
            extensions: &[".v", ".veo"],
            tm_scope: "source.verilog",
            ace_mode: "verilog",
            language_id: 387u64,
            aliases: &[],
            codemirror_mode: Some("verilog"),
            codemirror_mime_type: Some("text/x-verilog"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl VimHelpFile {
    pub fn info() -> Language {
        Language {
            name: "Vim Help File",
            r#type: "prose",
            color: "#199f4b",
            extensions: &[".txt"],
            tm_scope: "text.vim-help",
            ace_mode: "text",
            language_id: 508563686u64,
            aliases: &["help", "vimhelp"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl VimScript {
    pub fn info() -> Language {
        Language {
            name: "Vim Script",
            r#type: "programming",
            color: "#199f4b",
            extensions: &[".vim", ".vba", ".vimrc", ".vmb"],
            tm_scope: "source.viml",
            ace_mode: "text",
            language_id: 388u64,
            aliases: &["vim", "viml", "nvim", "vimscript"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[
                ".exrc",
                ".gvimrc",
                ".nvimrc",
                ".vimrc",
                "_vimrc",
                "gvimrc",
                "nvimrc",
                "vimrc",
            ],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl VimSnippet {
    pub fn info() -> Language {
        Language {
            name: "Vim Snippet",
            r#type: "markup",
            color: "#199f4b",
            extensions: &[".snip", ".snippet", ".snippets"],
            tm_scope: "source.vim-snippet",
            ace_mode: "text",
            language_id: 81265970u64,
            aliases: &["SnipMate", "UltiSnip", "UltiSnips", "NeoSnippet"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl VisualBasicNET {
    pub fn info() -> Language {
        Language {
            name: "Visual Basic .NET",
            r#type: "programming",
            color: "#945db7",
            extensions: &[".vb", ".vbhtml"],
            tm_scope: "source.vbnet",
            ace_mode: "text",
            language_id: 389u64,
            aliases: &["visual basic", "vbnet", "vb .net", "vb.net"],
            codemirror_mode: Some("vb"),
            codemirror_mime_type: Some("text/x-vb"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl VisualBasic60 {
    pub fn info() -> Language {
        Language {
            name: "Visual Basic 6.0",
            r#type: "programming",
            color: "#2c6353",
            extensions: &[".bas", ".cls", ".ctl", ".Dsr", ".frm"],
            tm_scope: "source.vba",
            ace_mode: "text",
            language_id: 679594952u64,
            aliases: &[
                "vb6",
                "vb 6",
                "visual basic 6",
                "visual basic classic",
                "classic visual basic",
            ],
            codemirror_mode: Some("vb"),
            codemirror_mime_type: Some("text/x-vb"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Volt {
    pub fn info() -> Language {
        Language {
            name: "Volt",
            r#type: "programming",
            color: "#1F1F1F",
            extensions: &[".volt"],
            tm_scope: "source.d",
            ace_mode: "d",
            language_id: 390u64,
            aliases: &[],
            codemirror_mode: Some("d"),
            codemirror_mime_type: Some("text/x-d"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Vue {
    pub fn info() -> Language {
        Language {
            name: "Vue",
            r#type: "markup",
            color: "#41b883",
            extensions: &[".vue"],
            tm_scope: "source.vue",
            ace_mode: "html",
            language_id: 391u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Vyper {
    pub fn info() -> Language {
        Language {
            name: "Vyper",
            r#type: "programming",
            color: "#2980b9",
            extensions: &[".vy"],
            tm_scope: "source.vyper",
            ace_mode: "text",
            language_id: 1055641948u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl WDL {
    pub fn info() -> Language {
        Language {
            name: "WDL",
            r#type: "programming",
            color: "#42f1f4",
            extensions: &[".wdl"],
            tm_scope: "source.wdl",
            ace_mode: "text",
            language_id: 374521672u64,
            aliases: &["Workflow Description Language"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl WGSL {
    pub fn info() -> Language {
        Language {
            name: "WGSL",
            r#type: "programming",
            color: "#1a5e9a",
            extensions: &[".wgsl"],
            tm_scope: "source.wgsl",
            ace_mode: "text",
            language_id: 836605993u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl WavefrontMaterial {
    pub fn info() -> Language {
        Language {
            name: "Wavefront Material",
            r#type: "data",
            color: "",
            extensions: &[".mtl"],
            tm_scope: "source.wavefront.mtl",
            ace_mode: "text",
            language_id: 392u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl WavefrontObject {
    pub fn info() -> Language {
        Language {
            name: "Wavefront Object",
            r#type: "data",
            color: "",
            extensions: &[".obj"],
            tm_scope: "source.wavefront.obj",
            ace_mode: "text",
            language_id: 393u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl WebOntologyLanguage {
    pub fn info() -> Language {
        Language {
            name: "Web Ontology Language",
            r#type: "data",
            color: "#5b70bd",
            extensions: &[".owl"],
            tm_scope: "text.xml",
            ace_mode: "xml",
            language_id: 394u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl WebAssembly {
    pub fn info() -> Language {
        Language {
            name: "WebAssembly",
            r#type: "programming",
            color: "#04133b",
            extensions: &[".wast", ".wat"],
            tm_scope: "source.webassembly",
            ace_mode: "lisp",
            language_id: 956556503u64,
            aliases: &["wast", "wasm"],
            codemirror_mode: Some("commonlisp"),
            codemirror_mime_type: Some("text/x-common-lisp"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl WebAssemblyInterfaceType {
    pub fn info() -> Language {
        Language {
            name: "WebAssembly Interface Type",
            r#type: "data",
            color: "#6250e7",
            extensions: &[".wit"],
            tm_scope: "source.wit",
            ace_mode: "text",
            language_id: 134534086u64,
            aliases: &["wit"],
            codemirror_mode: Some("webidl"),
            codemirror_mime_type: Some("text/x-webidl"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl WebIDL {
    pub fn info() -> Language {
        Language {
            name: "WebIDL",
            r#type: "programming",
            color: "",
            extensions: &[".webidl"],
            tm_scope: "source.webidl",
            ace_mode: "text",
            language_id: 395u64,
            aliases: &[],
            codemirror_mode: Some("webidl"),
            codemirror_mime_type: Some("text/x-webidl"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl WebVTT {
    pub fn info() -> Language {
        Language {
            name: "WebVTT",
            r#type: "data",
            color: "",
            extensions: &[".vtt"],
            tm_scope: "text.vtt",
            ace_mode: "text",
            language_id: 658679714u64,
            aliases: &["vtt"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: Some(true),
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl WgetConfig {
    pub fn info() -> Language {
        Language {
            name: "Wget Config",
            r#type: "data",
            color: "",
            extensions: &[],
            tm_scope: "source.wgetrc",
            ace_mode: "text",
            language_id: 668457123u64,
            aliases: &["wgetrc"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[".wgetrc"],
            group: Some("INI"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Whiley {
    pub fn info() -> Language {
        Language {
            name: "Whiley",
            r#type: "programming",
            color: "#d5c397",
            extensions: &[".whiley"],
            tm_scope: "source.whiley",
            ace_mode: "text",
            language_id: 888779559u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Wikitext {
    pub fn info() -> Language {
        Language {
            name: "Wikitext",
            r#type: "prose",
            color: "#fc5757",
            extensions: &[".mediawiki", ".wiki", ".wikitext"],
            tm_scope: "text.html.mediawiki",
            ace_mode: "text",
            language_id: 228u64,
            aliases: &["mediawiki", "wiki"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: Some(true),
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Win32MessageFile {
    pub fn info() -> Language {
        Language {
            name: "Win32 Message File",
            r#type: "data",
            color: "",
            extensions: &[".mc"],
            tm_scope: "source.win32-messages",
            ace_mode: "ini",
            language_id: 950967261u64,
            aliases: &[],
            codemirror_mode: Some("properties"),
            codemirror_mime_type: Some("text/x-properties"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl WindowsRegistryEntries {
    pub fn info() -> Language {
        Language {
            name: "Windows Registry Entries",
            r#type: "data",
            color: "#52d5ff",
            extensions: &[".reg"],
            tm_scope: "source.reg",
            ace_mode: "ini",
            language_id: 969674868u64,
            aliases: &[],
            codemirror_mode: Some("properties"),
            codemirror_mime_type: Some("text/x-properties"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl WitcherScript {
    pub fn info() -> Language {
        Language {
            name: "Witcher Script",
            r#type: "programming",
            color: "#ff0000",
            extensions: &[".ws"],
            tm_scope: "source.witcherscript",
            ace_mode: "text",
            language_id: 686821385u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Wollok {
    pub fn info() -> Language {
        Language {
            name: "Wollok",
            r#type: "programming",
            color: "#a23738",
            extensions: &[".wlk"],
            tm_scope: "source.wollok",
            ace_mode: "text",
            language_id: 632745969u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl WorldOfWarcraftAddonData {
    pub fn info() -> Language {
        Language {
            name: "World of Warcraft Addon Data",
            r#type: "data",
            color: "#f7e43f",
            extensions: &[".toc"],
            tm_scope: "source.toc",
            ace_mode: "text",
            language_id: 396u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Wren {
    pub fn info() -> Language {
        Language {
            name: "Wren",
            r#type: "programming",
            color: "#383838",
            extensions: &[".wren"],
            tm_scope: "source.wren",
            ace_mode: "text",
            language_id: 713580619u64,
            aliases: &["wrenlang"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl XBitMap {
    pub fn info() -> Language {
        Language {
            name: "X BitMap",
            r#type: "data",
            color: "",
            extensions: &[".xbm"],
            tm_scope: "source.c",
            ace_mode: "c_cpp",
            language_id: 782911107u64,
            aliases: &["xbm"],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-csrc"),
            wrap: None,
            filenames: &[],
            group: Some("C"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl XFontDirectoryIndex {
    pub fn info() -> Language {
        Language {
            name: "X Font Directory Index",
            r#type: "data",
            color: "",
            extensions: &[],
            tm_scope: "source.fontdir",
            ace_mode: "text",
            language_id: 208700028u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["encodings.dir", "fonts.alias", "fonts.dir", "fonts.scale"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl XPixMap {
    pub fn info() -> Language {
        Language {
            name: "X PixMap",
            r#type: "data",
            color: "",
            extensions: &[".xpm", ".pm"],
            tm_scope: "source.c",
            ace_mode: "c_cpp",
            language_id: 781846279u64,
            aliases: &["xpm"],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-csrc"),
            wrap: None,
            filenames: &[],
            group: Some("C"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl X10 {
    pub fn info() -> Language {
        Language {
            name: "X10",
            r#type: "programming",
            color: "#4B6BEF",
            extensions: &[".x10"],
            tm_scope: "source.x10",
            ace_mode: "text",
            language_id: 397u64,
            aliases: &["xten"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl XC {
    pub fn info() -> Language {
        Language {
            name: "XC",
            r#type: "programming",
            color: "#99DA07",
            extensions: &[".xc"],
            tm_scope: "source.xc",
            ace_mode: "c_cpp",
            language_id: 398u64,
            aliases: &[],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-csrc"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl XCompose {
    pub fn info() -> Language {
        Language {
            name: "XCompose",
            r#type: "data",
            color: "",
            extensions: &[],
            tm_scope: "config.xcompose",
            ace_mode: "text",
            language_id: 225167241u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[".XCompose", "XCompose", "xcompose"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl XML {
    pub fn info() -> Language {
        Language {
            name: "XML",
            r#type: "data",
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
                ".gpx",
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
            language_id: 399u64,
            aliases: &["rss", "xsd", "wsdl"],
            codemirror_mode: Some("xml"),
            codemirror_mime_type: Some("text/xml"),
            wrap: None,
            filenames: &[
                ".classpath",
                ".cproject",
                ".project",
                "App.config",
                "NuGet.config",
                "Settings.StyleCop",
                "Web.Debug.config",
                "Web.Release.config",
                "Web.config",
                "packages.config",
            ],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl XMLPropertyList {
    pub fn info() -> Language {
        Language {
            name: "XML Property List",
            r#type: "data",
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
            language_id: 75622871u64,
            aliases: &[],
            codemirror_mode: Some("xml"),
            codemirror_mime_type: Some("text/xml"),
            wrap: None,
            filenames: &[],
            group: Some("XML"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl XPages {
    pub fn info() -> Language {
        Language {
            name: "XPages",
            r#type: "data",
            color: "",
            extensions: &[".xsp-config", ".xsp.metadata"],
            tm_scope: "text.xml",
            ace_mode: "xml",
            language_id: 400u64,
            aliases: &[],
            codemirror_mode: Some("xml"),
            codemirror_mime_type: Some("text/xml"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl XProc {
    pub fn info() -> Language {
        Language {
            name: "XProc",
            r#type: "programming",
            color: "",
            extensions: &[".xpl", ".xproc"],
            tm_scope: "text.xml",
            ace_mode: "xml",
            language_id: 401u64,
            aliases: &[],
            codemirror_mode: Some("xml"),
            codemirror_mime_type: Some("text/xml"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl XQuery {
    pub fn info() -> Language {
        Language {
            name: "XQuery",
            r#type: "programming",
            color: "#5232e7",
            extensions: &[".xquery", ".xq", ".xql", ".xqm", ".xqy"],
            tm_scope: "source.xq",
            ace_mode: "xquery",
            language_id: 402u64,
            aliases: &[],
            codemirror_mode: Some("xquery"),
            codemirror_mime_type: Some("application/xquery"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl XS {
    pub fn info() -> Language {
        Language {
            name: "XS",
            r#type: "programming",
            color: "",
            extensions: &[".xs"],
            tm_scope: "source.c",
            ace_mode: "c_cpp",
            language_id: 403u64,
            aliases: &[],
            codemirror_mode: Some("clike"),
            codemirror_mime_type: Some("text/x-csrc"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl XSLT {
    pub fn info() -> Language {
        Language {
            name: "XSLT",
            r#type: "programming",
            color: "#EB8CEB",
            extensions: &[".xslt", ".xsl"],
            tm_scope: "text.xml.xsl",
            ace_mode: "xml",
            language_id: 404u64,
            aliases: &["xsl"],
            codemirror_mode: Some("xml"),
            codemirror_mime_type: Some("text/xml"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Xmake {
    pub fn info() -> Language {
        Language {
            name: "Xmake",
            r#type: "programming",
            color: "#22a079",
            extensions: &[],
            tm_scope: "source.xmake",
            ace_mode: "text",
            language_id: 225223071u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["xmake.lua"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Xojo {
    pub fn info() -> Language {
        Language {
            name: "Xojo",
            r#type: "programming",
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
            language_id: 405u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Xonsh {
    pub fn info() -> Language {
        Language {
            name: "Xonsh",
            r#type: "programming",
            color: "#285EEF",
            extensions: &[".xsh"],
            tm_scope: "source.python",
            ace_mode: "text",
            language_id: 614078284u64,
            aliases: &[],
            codemirror_mode: Some("python"),
            codemirror_mime_type: Some("text/x-python"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Xtend {
    pub fn info() -> Language {
        Language {
            name: "Xtend",
            r#type: "programming",
            color: "#24255d",
            extensions: &[".xtend"],
            tm_scope: "source.xtend",
            ace_mode: "text",
            language_id: 406u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl YAML {
    pub fn info() -> Language {
        Language {
            name: "YAML",
            r#type: "data",
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
            language_id: 407u64,
            aliases: &["yml"],
            codemirror_mode: Some("yaml"),
            codemirror_mime_type: Some("text/x-yaml"),
            wrap: None,
            filenames: &[
                ".clang-format",
                ".clang-tidy",
                ".clangd",
                ".gemrc",
                "CITATION.cff",
                "glide.lock",
                "pixi.lock",
                "yarn.lock",
            ],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl YANG {
    pub fn info() -> Language {
        Language {
            name: "YANG",
            r#type: "data",
            color: "",
            extensions: &[".yang"],
            tm_scope: "source.yang",
            ace_mode: "text",
            language_id: 408u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl YARA {
    pub fn info() -> Language {
        Language {
            name: "YARA",
            r#type: "programming",
            color: "#220000",
            extensions: &[".yar", ".yara"],
            tm_scope: "source.yara",
            ace_mode: "text",
            language_id: 805122868u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl YASnippet {
    pub fn info() -> Language {
        Language {
            name: "YASnippet",
            r#type: "markup",
            color: "#32AB90",
            extensions: &[".yasnippet"],
            tm_scope: "source.yasnippet",
            ace_mode: "text",
            language_id: 378760102u64,
            aliases: &["snippet", "yas"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Yacc {
    pub fn info() -> Language {
        Language {
            name: "Yacc",
            r#type: "programming",
            color: "#4B6C4B",
            extensions: &[".y", ".yacc", ".yy"],
            tm_scope: "source.yacc",
            ace_mode: "text",
            language_id: 409u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Yul {
    pub fn info() -> Language {
        Language {
            name: "Yul",
            r#type: "programming",
            color: "#794932",
            extensions: &[".yul"],
            tm_scope: "source.yul",
            ace_mode: "text",
            language_id: 237469033u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ZAP {
    pub fn info() -> Language {
        Language {
            name: "ZAP",
            r#type: "programming",
            color: "#0d665e",
            extensions: &[".zap", ".xzap"],
            tm_scope: "source.zap",
            ace_mode: "text",
            language_id: 952972794u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ZIL {
    pub fn info() -> Language {
        Language {
            name: "ZIL",
            r#type: "programming",
            color: "#dc75e5",
            extensions: &[".zil", ".mud"],
            tm_scope: "source.zil",
            ace_mode: "text",
            language_id: 973483626u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Zeek {
    pub fn info() -> Language {
        Language {
            name: "Zeek",
            r#type: "programming",
            color: "",
            extensions: &[".zeek", ".bro"],
            tm_scope: "source.zeek",
            ace_mode: "text",
            language_id: 40u64,
            aliases: &["bro"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ZenScript {
    pub fn info() -> Language {
        Language {
            name: "ZenScript",
            r#type: "programming",
            color: "#00BCD1",
            extensions: &[".zs"],
            tm_scope: "source.zenscript",
            ace_mode: "text",
            language_id: 494938890u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Zephir {
    pub fn info() -> Language {
        Language {
            name: "Zephir",
            r#type: "programming",
            color: "#118f9e",
            extensions: &[".zep"],
            tm_scope: "source.php.zephir",
            ace_mode: "php",
            language_id: 410u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Zig {
    pub fn info() -> Language {
        Language {
            name: "Zig",
            r#type: "programming",
            color: "#ec915c",
            extensions: &[".zig", ".zig.zon"],
            tm_scope: "source.zig",
            ace_mode: "text",
            language_id: 646424281u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Zimpl {
    pub fn info() -> Language {
        Language {
            name: "Zimpl",
            r#type: "programming",
            color: "#d67711",
            extensions: &[".zimpl", ".zmpl", ".zpl"],
            tm_scope: "none",
            ace_mode: "text",
            language_id: 411u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl CURLConfig {
    pub fn info() -> Language {
        Language {
            name: "cURL Config",
            r#type: "data",
            color: "",
            extensions: &[],
            tm_scope: "source.curlrc",
            ace_mode: "text",
            language_id: 992375436u64,
            aliases: &["curlrc"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[".curlrc", "_curlrc"],
            group: Some("INI"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Crontab {
    pub fn info() -> Language {
        Language {
            name: "crontab",
            r#type: "data",
            color: "#ead7ac",
            extensions: &[],
            tm_scope: "text.crontab",
            ace_mode: "tcl",
            language_id: 705203557u64,
            aliases: &["cron", "cron table"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["crontab"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Desktop {
    pub fn info() -> Language {
        Language {
            name: "desktop",
            r#type: "data",
            color: "",
            extensions: &[".desktop", ".desktop.in", ".service"],
            tm_scope: "source.desktop",
            ace_mode: "text",
            language_id: 412u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Dircolors {
    pub fn info() -> Language {
        Language {
            name: "dircolors",
            r#type: "data",
            color: "",
            extensions: &[".dircolors"],
            tm_scope: "source.dircolors",
            ace_mode: "text",
            language_id: 691605112u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[
                ".dir_colors",
                ".dircolors",
                "DIR_COLORS",
                "_dir_colors",
                "_dircolors",
                "dir_colors",
            ],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl EC {
    pub fn info() -> Language {
        Language {
            name: "eC",
            r#type: "programming",
            color: "#913960",
            extensions: &[".ec", ".eh"],
            tm_scope: "source.c.ec",
            ace_mode: "text",
            language_id: 413u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Edn {
    pub fn info() -> Language {
        Language {
            name: "edn",
            r#type: "data",
            color: "",
            extensions: &[".edn"],
            tm_scope: "source.clojure",
            ace_mode: "clojure",
            language_id: 414u64,
            aliases: &[],
            codemirror_mode: Some("clojure"),
            codemirror_mime_type: Some("text/x-clojure"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Fish {
    pub fn info() -> Language {
        Language {
            name: "fish",
            r#type: "programming",
            color: "#4aae47",
            extensions: &[".fish"],
            tm_scope: "source.fish",
            ace_mode: "text",
            language_id: 415u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: Some("Shell"),
            interpreters: &["fish"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Hoon {
    pub fn info() -> Language {
        Language {
            name: "hoon",
            r#type: "programming",
            color: "#00b171",
            extensions: &[".hoon"],
            tm_scope: "source.hoon",
            ace_mode: "text",
            language_id: 560883276u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ICalendar {
    pub fn info() -> Language {
        Language {
            name: "iCalendar",
            r#type: "data",
            color: "#ec564c",
            extensions: &[".ics", ".ical"],
            tm_scope: "source.iCalendar",
            ace_mode: "properties",
            language_id: 98384424u64,
            aliases: &["iCal"],
            codemirror_mode: Some("properties"),
            codemirror_mime_type: Some("text/x-properties"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Jq {
    pub fn info() -> Language {
        Language {
            name: "jq",
            r#type: "programming",
            color: "#c7254e",
            extensions: &[".jq"],
            tm_scope: "source.jq",
            ace_mode: "text",
            language_id: 905371884u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["gojq", "jaq", "jq", "jqjq", "jqq", "query-json"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Kvlang {
    pub fn info() -> Language {
        Language {
            name: "kvlang",
            r#type: "markup",
            color: "#1da6e0",
            extensions: &[".kv"],
            tm_scope: "source.python.kivy",
            ace_mode: "text",
            language_id: 970675279u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl MIRCScript {
    pub fn info() -> Language {
        Language {
            name: "mIRC Script",
            r#type: "programming",
            color: "#3d57c3",
            extensions: &[".mrc"],
            tm_scope: "source.msl",
            ace_mode: "text",
            language_id: 517654727u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Mcfunction {
    pub fn info() -> Language {
        Language {
            name: "mcfunction",
            r#type: "programming",
            color: "#E22837",
            extensions: &[".mcfunction"],
            tm_scope: "source.mcfunction",
            ace_mode: "text",
            language_id: 462488745u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Mdsvex {
    pub fn info() -> Language {
        Language {
            name: "mdsvex",
            r#type: "markup",
            color: "#5f9ea0",
            extensions: &[".svx"],
            tm_scope: "none",
            ace_mode: "markdown",
            language_id: 566198445u64,
            aliases: &[],
            codemirror_mode: Some("gfm"),
            codemirror_mime_type: Some("text/x-gfm"),
            wrap: Some(true),
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Mupad {
    pub fn info() -> Language {
        Language {
            name: "mupad",
            r#type: "programming",
            color: "#244963",
            extensions: &[".mu"],
            tm_scope: "source.mupad",
            ace_mode: "text",
            language_id: 416u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Nanorc {
    pub fn info() -> Language {
        Language {
            name: "nanorc",
            r#type: "data",
            color: "#2d004d",
            extensions: &[".nanorc"],
            tm_scope: "source.nanorc",
            ace_mode: "text",
            language_id: 775996197u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[".nanorc", "nanorc"],
            group: Some("INI"),
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl NesC {
    pub fn info() -> Language {
        Language {
            name: "nesC",
            r#type: "programming",
            color: "#94B0C7",
            extensions: &[".nc"],
            tm_scope: "source.nesc",
            ace_mode: "text",
            language_id: 417u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Ooc {
    pub fn info() -> Language {
        Language {
            name: "ooc",
            r#type: "programming",
            color: "#b0b77e",
            extensions: &[".ooc"],
            tm_scope: "source.ooc",
            ace_mode: "text",
            language_id: 418u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Q {
    pub fn info() -> Language {
        Language {
            name: "q",
            r#type: "programming",
            color: "#0040cd",
            extensions: &[".q"],
            tm_scope: "source.q",
            ace_mode: "text",
            language_id: 970539067u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl ReStructuredText {
    pub fn info() -> Language {
        Language {
            name: "reStructuredText",
            r#type: "prose",
            color: "#141414",
            extensions: &[".rst", ".rest", ".rest.txt", ".rst.txt"],
            tm_scope: "text.restructuredtext",
            ace_mode: "text",
            language_id: 419u64,
            aliases: &["rst"],
            codemirror_mode: Some("rst"),
            codemirror_mime_type: Some("text/x-rst"),
            wrap: Some(true),
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Robotstxt {
    pub fn info() -> Language {
        Language {
            name: "robots.txt",
            r#type: "data",
            color: "",
            extensions: &[],
            tm_scope: "text.robots-txt",
            ace_mode: "text",
            language_id: 674736065u64,
            aliases: &["robots", "robots txt"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &["robots.txt"],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Sed {
    pub fn info() -> Language {
        Language {
            name: "sed",
            r#type: "programming",
            color: "#64b970",
            extensions: &[".sed"],
            tm_scope: "source.sed",
            ace_mode: "text",
            language_id: 847830017u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &["gsed", "minised", "sed", "ssed"],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Templ {
    pub fn info() -> Language {
        Language {
            name: "templ",
            r#type: "markup",
            color: "#66D0DD",
            extensions: &[".templ"],
            tm_scope: "source.templ",
            ace_mode: "text",
            language_id: 795579337u64,
            aliases: &[],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl VCard {
    pub fn info() -> Language {
        Language {
            name: "vCard",
            r#type: "data",
            color: "#ee2647",
            extensions: &[".vcf"],
            tm_scope: "source.vcard",
            ace_mode: "properties",
            language_id: 851476558u64,
            aliases: &["virtual contact file", "electronic business card"],
            codemirror_mode: Some("properties"),
            codemirror_mime_type: Some("text/x-properties"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl Wisp {
    pub fn info() -> Language {
        Language {
            name: "wisp",
            r#type: "programming",
            color: "#7582D1",
            extensions: &[".wisp"],
            tm_scope: "source.clojure",
            ace_mode: "clojure",
            language_id: 420u64,
            aliases: &[],
            codemirror_mode: Some("clojure"),
            codemirror_mime_type: Some("text/x-clojure"),
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
impl XBase {
    pub fn info() -> Language {
        Language {
            name: "xBase",
            r#type: "programming",
            color: "#403a40",
            extensions: &[".prg", ".ch", ".prw"],
            tm_scope: "source.harbour",
            ace_mode: "text",
            language_id: 421u64,
            aliases: &["advpl", "clipper", "foxpro"],
            codemirror_mode: None,
            codemirror_mime_type: None,
            wrap: None,
            filenames: &[],
            group: None,
            interpreters: &[],
            fs_name: None,
            searchable: None,
        }
    }
}
pub struct Languages {
    languages: Vec<Language>,
    by_name: HashMap<&'static str, usize>,
    by_extension: HashMap<&'static str, Vec<usize>>,
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
            AngelScript::info(), AnswerSetProgramming::info(), AntBuildSystem::info(),
            Antlers::info(), ApacheConf::info(), Apex::info(),
            ApolloGuidanceComputer::info(), AppleScript::info(), Arc::info(),
            AsciiDoc::info(), AspectJ::info(), Assembly::info(), Astro::info(),
            Asymptote::info(), Augeas::info(), AutoHotkey::info(), AutoIt::info(),
            AvroIDL::info(), Awk::info(), B4X::info(), BASIC::info(), BQN::info(),
            Ballerina::info(), Batchfile::info(), Beef::info(), Befunge::info(),
            Berry::info(), BibTeX::info(), Bicep::info(), Bikeshed::info(),
            Bison::info(), BitBake::info(), Blade::info(), BlitzBasic::info(),
            BlitzMax::info(), Bluespec::info(), BluespecBH::info(), Boo::info(),
            Boogie::info(), Brainfuck::info(), BrighterScript::info(),
            Brightscript::info(), Browserslist::info(), C::info(), Csharp::info(),
            Cpp::info(), CObjDump::info(), C2hsHaskell::info(), CAPCDS::info(),
            CIL::info(), CLIPS::info(), CMake::info(), COBOL::info(), CODEOWNERS::info(),
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
            FreeBASIC::info(), FreeMarker::info(), Frege::info(), Futhark::info(),
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
            Harbour::info(), Hare::info(), Haskell::info(), Haxe::info(), HiveQL::info(),
            HolyC::info(), HostsFile::info(), Hy::info(), HyPhy::info(), IDL::info(),
            IGORPro::info(), INI::info(), IRCLog::info(), Idris::info(),
            IgnoreList::info(), ImageJMacro::info(), Imba::info(), Inform7::info(),
            Ink::info(), InnoSetup::info(), Io::info(), Ioke::info(), Isabelle::info(),
            IsabelleROOT::info(), J::info(), JARManifest::info(), JCL::info(),
            JFlex::info(), JSON::info(), JSONWithComments::info(), JSON5::info(),
            JSONLD::info(), JSONiq::info(), Jai::info(), Janet::info(), Jasmin::info(),
            Java::info(), JavaProperties::info(), JavaServerPages::info(),
            JavaTemplateEngine::info(), JavaScript::info(), JavaScriptpERB::info(),
            JestSnapshot::info(), JetBrainsMPS::info(), Jinja::info(), Jison::info(),
            JisonLex::info(), Jolie::info(), Jsonnet::info(), Julia::info(),
            JuliaREPL::info(), JupyterNotebook::info(), Just::info(), KDL::info(),
            KRL::info(), KaitaiStruct::info(), KakouneScript::info(),
            KerboScript::info(), KiCadLayout::info(), KiCadLegacyLayout::info(),
            KiCadSchematic::info(), Kickstart::info(), Kit::info(), Kotlin::info(),
            Kusto::info(), LFE::info(), LLVM::info(), LOLCODE::info(), LSL::info(),
            LTspiceSymbol::info(), LabVIEW::info(), Lark::info(), Lasso::info(),
            Latte::info(), Lean::info(), Lean4::info(), Less::info(), Lex::info(),
            LigoLANG::info(), LilyPond::info(), Limbo::info(), LinearProgramming::info(),
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
            MiniZinc::info(), MiniZincData::info(), Mint::info(), Mirah::info(),
            Modelica::info(), Modula2::info(), Modula3::info(),
            ModuleManagementSystem::info(), Mojo::info(), Monkey::info(),
            MonkeyC::info(), Moocode::info(), MoonBit::info(), MoonScript::info(),
            Motoko::info(), Motorola68KAssembly::info(), Move::info(), Muse::info(),
            Mustache::info(), Myghty::info(), NASL::info(), NCL::info(), NEON::info(),
            NL::info(), NMODL::info(), NPMConfig::info(), NSIS::info(), NWScript::info(),
            Nasal::info(), Nearley::info(), Nemerle::info(), NetLinx::info(),
            NetLinxpERB::info(), NetLogo::info(), NewLisp::info(), Nextflow::info(),
            Nginx::info(), Nim::info(), Ninja::info(), Nit::info(), Nix::info(),
            Noir::info(), Nu::info(), NumPy::info(), Nunjucks::info(), Nushell::info(),
            OASv2Json::info(), OASv2Yaml::info(), OASv3Json::info(), OASv3Yaml::info(),
            OCaml::info(), OMNeTppMSG::info(), OMNeTppNED::info(), Oberon::info(),
            ObjDump::info(), ObjectDataInstanceNotation::info(), ObjectScript::info(),
            ObjectiveC::info(), ObjectiveCpp::info(), ObjectiveJ::info(), Odin::info(),
            Omgrofl::info(), Opa::info(), Opal::info(), OpenPolicyAgent::info(),
            OpenAPISpecificationV2::info(), OpenAPISpecificationV3::info(),
            OpenCL::info(), OpenEdgeABL::info(), OpenQASM::info(),
            OpenRCRunscript::info(), OpenSCAD::info(), OpenStepPropertyList::info(),
            OpenTypeFeatureFile::info(), OptionList::info(), Org::info(),
            OverpassQL::info(), Ox::info(), Oxygene::info(), Oz::info(), P4::info(),
            PDDL::info(), PEGjs::info(), PHP::info(), PLSQL::info(), PLpgSQL::info(),
            POVRaySDL::info(), Pact::info(), Pan::info(), Papyrus::info(),
            Parrot::info(), ParrotAssembly::info(), ParrotInternalRepresentation::info(),
            Pascal::info(), Pawn::info(), Pep8::info(), Perl::info(), Pic::info(),
            Pickle::info(), PicoLisp::info(), PigLatin::info(), Pike::info(),
            PipRequirements::info(), Pkl::info(), PlantUML::info(), Pod::info(),
            Pod6::info(), PogoScript::info(), Polar::info(), Pony::info(),
            Portugol::info(), PostCSS::info(), PostScript::info(), PowerBuilder::info(),
            PowerShell::info(), Praat::info(), Prisma::info(), Processing::info(),
            Procfile::info(), Proguard::info(), Prolog::info(), Promela::info(),
            PropellerSpin::info(), ProtocolBuffer::info(),
            ProtocolBufferTextFormat::info(), PublicKey::info(), Pug::info(),
            Puppet::info(), PureData::info(), PureBasic::info(), PureScript::info(),
            Pyret::info(), Python::info(), PythonConsole::info(),
            PythonTraceback::info(), Qsharp::info(), QML::info(), QMake::info(),
            QtScript::info(), Quake::info(), QuickBASIC::info(), R::info(), RAML::info(),
            RBS::info(), RDoc::info(), REALbasic::info(), REXX::info(),
            RMarkdown::info(), RON::info(), RPC::info(), RPGLE::info(), RPMSpec::info(),
            RUNOFF::info(), Racket::info(), Ragel::info(), Raku::info(), Rascal::info(),
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
            SurvexData::info(), Svelte::info(), Sway::info(), Sweave::info(),
            Swift::info(), SystemVerilog::info(), TIProgram::info(), TLVerilog::info(),
            TLA::info(), TOML::info(), TSPLIBData::info(), TSQL::info(), TSV::info(),
            TSX::info(), TXL::info(), Tact::info(), Talon::info(), Tcl::info(),
            Tcsh::info(), TeX::info(), Tea::info(), Terra::info(),
            TerraformTemplate::info(), Texinfo::info(), Text::info(), TextGrid::info(),
            TextMateProperties::info(), Textile::info(), Thrift::info(), Toit::info(),
            Turing::info(), Turtle::info(), Twig::info(), TypeLanguage::info(),
            TypeScript::info(), TypeSpec::info(), Typst::info(),
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
            XS::info(), XSLT::info(), Xmake::info(), Xojo::info(), Xonsh::info(),
            Xtend::info(), YAML::info(), YANG::info(), YARA::info(), YASnippet::info(),
            Yacc::info(), Yul::info(), ZAP::info(), ZIL::info(), Zeek::info(),
            ZenScript::info(), Zephir::info(), Zig::info(), Zimpl::info(),
            CURLConfig::info(), Crontab::info(), Desktop::info(), Dircolors::info(),
            EC::info(), Edn::info(), Fish::info(), Hoon::info(), ICalendar::info(),
            Jq::info(), Kvlang::info(), MIRCScript::info(), Mcfunction::info(),
            Mdsvex::info(), Mupad::info(), Nanorc::info(), NesC::info(), Ooc::info(),
            Q::info(), ReStructuredText::info(), Robotstxt::info(), Sed::info(),
            Templ::info(), VCard::info(), Wisp::info(), XBase::info()
        ];
        let mut by_name = HashMap::new();
        let mut by_extension = HashMap::new();
        for (index, lang) in languages.iter().enumerate() {
            by_name.insert(lang.name, index);
            for ext in lang.extensions {
                by_extension.entry(*ext).or_insert_with(Vec::new).push(index);
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
    pub fn get_by_id(&self, id: u64) -> Option<&Language> {
        self.languages.iter().find(|lang| lang.language_id == id)
    }
    pub fn get_by_alias(&self, alias: &str) -> Vec<&Language> {
        self.languages.iter().filter(|lang| lang.aliases.contains(&alias)).collect()
    }
    pub fn get_by_filename(&self, filename: &str) -> Vec<&Language> {
        self.languages.iter().filter(|lang| lang.filenames.contains(&filename)).collect()
    }
    pub fn get_by_interpreter(&self, interpreter: &str) -> Vec<&Language> {
        self.languages
            .iter()
            .filter(|lang| lang.interpreters.contains(&interpreter))
            .collect()
    }
    pub fn get_by_extension(&self, ext: &str) -> Vec<&Language> {
        self.by_extension
            .get(ext)
            .map(|indices| indices.iter().map(|&index| &self.languages[index]).collect())
            .unwrap_or_default()
    }
    pub fn get_by_type(&self, r#type: &str) -> Vec<&Language> {
        self.languages.iter().filter(|lang| lang.r#type == r#type).collect()
    }
    pub fn all_languages(&self) -> &[Language] {
        &self.languages
    }
}
impl Default for Languages {
    fn default() -> Self {
        Self::new()
    }
}
pub fn get_languages() -> Languages {
    Languages::new()
}
pub static LANGUAGES: Lazy<Languages> = Lazy::new(Languages::new);
