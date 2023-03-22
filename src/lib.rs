use std::result;

use fuzzy_matcher::{clangd::ClangdMatcher, skim::SkimMatcherV2, FuzzyMatcher};
use jaro_winkler_distance::{jaro_winkler_distance, PrefixLength};
use levenshtein_automata::{Distance, LevenshteinAutomatonBuilder};
use ngrammatic::{Corpus, CorpusBuilder, Pad};
use rust_fuzzy_search::fuzzy_search_sorted;
use sublime_fuzzy::best_match;

#[inline]
pub fn get_data() -> Vec<&'static str> {
    vec![
        "1Password",
        "7-Zip",
        "Adobe Acrobat",
        "Adobe Creative Cloud",
        "Adobe Photoshop",
        "Adobe Premiere Pro",
        "Adobe Reader",
        "Adobe XD",
        "Airtable",
        "Amazon AWS CLI",
        "Anaconda",
        "Android Studio",
        "Apache",
        "Atom",
        "Atom VPN",
        "Audacity",
        "AutoCAD",
        "Avast Antivirus",
        "AVG Antivirus",
        "Azure Data Studio",
        "Bitbucket",
        "BlueJeans",
        "Bluestacks",
        "Brave",
        "Calibre",
        "CCleaner",
        "Chrome",
        "Cisco AnyConnect",
        "Citrix Receiver",
        "Citrix Workspace",
        "Cloudflare WARP",
        "CodeIgniter",
        "Confluence",
        "Core FTP",
        "Cyberduck",
        "Dashlane",
        "DBeaver",
        "Discord",
        "Docker",
        "Dropbox",
        "Eclipse",
        "Edge",
        "Elasticsearch",
        "Enpass",
        "Epic Games Launcher",
        "ESET Antivirus",
        "Evernote",
        "FileZilla",
        "Firefox",
        "F.lux",
        "GanttProject",
        "GIMP",
        "Git",
        "GitHub Desktop",
        "GNS3",
        "Google Backup and Sync",
        "Google Chrome",
        "Google Drive",
        "Google Meet",
        "Google Play Music Desktop Player",
        "Google Play Services",
        "Google Play Store",
        "Google Sheets",
        "Google Slides",
        "Gpg4win",
        "GraphPad Prism",
        "GreenShot",
        "Hadoop",
        "HMA VPN",
        "Hootsuite",
        "HxD Hex Editor",
        "iTerm2",
        "Java",
        "Jenkins",
        "Jira",
        "Kaspersky Antivirus",
        "KeePass",
        "Krita",
        "LastPass",
        "Latex",
        "LibreOffice",
        "Logitech Options",
        "Luminar AI",
        "MAMP",
        "Malwarebytes Anti-Malware",
        "MariaDB",
        "MATLAB",
        "Microsoft Edge",
        "Microsoft Office",
        "Microsoft Power BI",
        "Microsoft Remote Desktop",
        "Microsoft Teams",
        "Microsoft Visio",
        "Microsoft Visual Studio",
        "Microsoft Visual Studio Code",
        "MindManager",
        "MongoDB Compass",
        "Mumble",
        "MySQL Workbench",
        "NetBeans",
        "Notepad++",
        "Obsidian",
        "Octave",
        "Office Lens",
        "OneDrive",
        "OneNote",
        "Opera",
        "Oracle Database",
        "Oracle VirtualBox",
        "Outlook",
        "Paint.NET",
        "Pandoc",
        "Pencil Project",
        "PhpStorm",
        "Postman",
        "PowerISO",
        "Prism Video Converter",
        "PuTTY",
        "Python",
        "QGIS",
        "QuickTime Player",
        "R",
        "R Studio",
        "Redis Desktop Manager",
        "Robo 3T",
        "Ruby",
        "Rufus",
        "SAP GUI",
        "SAS",
        "Scrivener",
        "SeaMonkey",
        "Security Onion",
        "Shotcut",
        "Skype",
        "Slack",
        "SlickEdit",
        "Snagit",
        "SourceTree",
        "Spark",
        "Spotify",
        "Spyder",
        "Sublime Text",
        "SugarSync",
        "Syncthing",
        "Tableau",
        "TeamViewer",
        "TeraTerm",
        "Termius",
        "Texmaker",
        "Thunderbird",
        "Toggl Track",
        "TopTracker",
        "TortoiseSVN",
        "Total Commander",
        "Transmission",
        "TunnelBear",
        "TurboTax",
        "Unity",
        "Vagrant",
        "VeraCrypt",
        "VLC media player",
        "VMware Workstation",
        "VNC Viewer",
        "Visual Studio for Mac",
        "VMware Fusion",
        "VSDC Free Video Editor",
        "Wavebox",
        "WebStorm",
        "WinDirStat",
        "Windows Terminal",
        "WinMerge",
        "WinSCP",
        "Wireshark",
        "Wordpress",
        "XAMPP",
        "Xcode",
        "XMind",
        "Zoom",
        "Zotero",
        "Zsh",
        "Adobe After Effects",
        "Adobe Audition",
        "Adobe Dreamweaver",
        "Adobe Illustrator",
        "Adobe InDesign",
        "Adobe Lightroom",
        "Adobe Media Encoder",
        "Adobe Prelude",
        "Adobe Spark",
        "Adobe Stock",
        "Adobe XD Design Specs",
        "Android Debug Bridge",
        "AnyDesk",
        "Apache JMeter",
        "Archi",
        "Asana",
        "Aspera Connect",
        "Audible",
        "AudibleSync",
        "AutoIt",
        "Autodesk Maya",
        "Autodesk Sketchbook",
        "Axure RP",
        "Balsamiq Wireframes",
        "Bartender",
        "Beyond Compare",
        "Bitwarden",
        "Blender",
        "Bluebeam Revu",
        "Box Drive",
        "Browsersync",
        "BurnAware",
        "Camtasia",
        "Carbon Copy Cloner",
        "Ccleaner Professional",
        "Cheat Engine",
        "Cisco Packet Tracer",
        "Civ6",
        "Clonezilla",
        "CloudApp",
        "Cocktail",
        "Color Picker",
        "ConnectWise Control",
        "Couchbase Server",
        "CrashPlan",
        "Creative Cloud Cleaner Tool",
        "Cubase",
        "CyberGhost VPN",
        "DaVinci Resolve",
        "DB Browser for SQLite",
        "Deluge",
        "DeskScapes",
        "Dev-C++",
        "Disk Drill",
        "Divvy",
        "Docker Compose",
        "Dolphin Emulator",
        "Driver Easy",
        "Dropbox Business",
        "EasyBCD",
        "EasyBCD Community Edition",
        "Eclipse IDE for Java EE Developers",
        "Eclipse Marketplace",
        "Emby Server",
        "EndNote",
        "Epic Games Launcher Unreal Tournament",
        "Epson Connect Printer Setup",
        "ES File Explorer",
        "ExpanDrive",
        "ExpressVPN",
        "Figma",
        "Fing",
        "FireAlpaca",
        "Firebird",
        "FlowJo",
        "Fluxbox",
        "Foxit Reader",
        "FreeFileSync",
        "FrostWire",
        "Fusion 360",
        "GanttProject Classic",
        "Geany",
        "GeForce Experience",
        "GlassWire",
        "GnuCash",
        "Godot",
        "Google Backup and Sync from Google",
        "Google Cloud SDK",
        "Google Play Movies & TV",
        "Google Play Services for AR",
        "Google Tag Assistant",
        "Grammarly for Microsoft Word",
        "Greenfoot",
        "Gyazo",
        "HandBrake",
        "HeidiSQL",
        "Helix Core Command-Line Client",
        "Hugin",
        "Hyper",
        "IIS Express",
        "IrfanView",
        "iTunes",
        "Java Development Kit",
        "JetBrains Toolbox",
        "JMP",
        "Jupyter",
        "Kaleidoscope",
        "Kap",
        "Kaspersky Endpoint Security 10",
        "Kaspersky Security Cloud",
        "KeyShot",
        "Kodi",
        "Koofr",
        "Krita Gemini",
        "Krita Studio",
        "LaunchBox",
        "Launchy",
        "LICEcap",
        "Lightworks",
        "LingPipe",
        "LiquidPlanner",
        "LMMS",
        "Logitech Gaming Software",
        "LogMeIn Hamachi",
        "MAMP PRO",
        "ManageEngine Applications Manager",
        "Marktext",
        "MegaSync",
        "MeshLab",
        "Microsoft Excel Viewer",
        "Microsoft Flight Simulator X",
        "Microsoft PowerPoint Viewer",
        "Microsoft Project Professional 2019",
        "Microsoft Remote Desktop",
        "Microsoft Teams",
        "Microsoft Visual Studio Installer Projects",
        "Microsoft Visual Studio Installer Projects Extension",
        "Microsoft Visual Studio Installer Projects Preview",
        "Microsoft Visual Studio Installer Projects Visual Basic Runtime",
        "Microsoft Visual Studio Professional 2019",
        "Microsoft Visual Studio Shell (Integrated)",
        "Microsoft Visual Studio Tools for Unity",
        "Microsoft Visual Studio Web Authoring Component",
        "Microsoft Word",
        "Miniconda3",
        "MongoDB Compass",
        "Moodle Desktop",
        "MorphoJ",
        "Mozilla Firefox",
        "Mozilla Thunderbird",
        "MSI Afterburner",
        "MySQL Connector/NET",
        "MySQL Installer",
        "MySQL Notifier",
        "MySQL Server 8.0",
        "Namecheap VPN",
        "Navicat Premium",
        "Neo4j Desktop",
        "NetBeans IDE",
        "Netscape",
        "Nextcloud",
        "Nik Collection",
        "NinjaRMM",
        "Notepad++",
        "NVIDIA Control Panel",
        "NVIDIA GeForce Experience",
        "OBS Studio",
        "Office Deployment Tool",
        "Okta Verify",
        "OmniGraffle",
        "OneDrive",
        "OneNote",
        "Open Broadcaster Software",
        "OpenVPN Connect",
        "Opera",
        "Oracle VM VirtualBox",
        "Outlook",
        "Paint 3D",
        "Pandoc",
        "Parallels Desktop",
        "Password Safe",
        "PDFsam Basic",
        "Pencil2D",
        "Plex Media Server",
        "Postman",
        "PowerShell",
        "Private Internet Access",
        "Pro Tools First",
        "Proxifier",
        "PuTTY",
        "PyCharm",
        "Python 3.7",
        "Python 3.8",
        "Python 3.9",
        "Python Launcher",
        "qBittorrent",
        "QuickTime Player",
        "R for Windows",
        "R Studio",
        "Racket",
        "RapidMiner Studio",
        "React Developer Tools",
        "RealVNC",
        "Reaper",
        "Redshift",
        "Redis Desktop Manager",
        "Remote Desktop Manager",
        "Rider",
        "RStudio",
        "RStudio Desktop",
        "RStudio Server",
        "Safari",
        "SAP GUI",
        "SAP GUI for Java",
        "SAS Studio",
        "Screaming Frog SEO Spider",
        "ScreenFlow",
        "Screenshot Captor",
        "Scrivener",
        "Seafile Client",
        "SecureCRT",
        "SecureFX",
        "SecurityKISS Tunnel",
        "Selenium WebDriver",
        "ShareX",
        "Signal",
        "SimpleMind",
        "SketchUp Pro",
        "Skype",
        "Slack",
        "SmartGit",
        "SmartSVN",
        "SmartThings",
        "Snagit",
        "SoapUI",
        "SourceTree",
        "SPSS",
        "SQL Server Management Studio",
        "SQLiteSpy",
        "Stellarium",
        "Sublime Text",
        "SuperDuper!",
        "Supremo Remote Desktop",
        "Synology Assistant",
        "Tableau Desktop",
        "TeraCopy",
        "TeX Live",
        "TeXstudio",
        "The Unarchiver",
        "TightVNC",
        "Todoist",
        "Tor Browser",
        "Total Commander",
        "Transmit",
        "Trello",
        "Turbo C++",
        "Turbo VPN",
        "TurboCollage",
        "TuxGuitar",
        "Twitch Studio",
        "Ultimate Windows Tweaker 4",
        "Unity Hub",
        "Unity Personal",
        "Unity Pro",
        "uTorrent",
        "Vagrant",
        "Vegas Pro",
        "VirtualBox Guest Additions",
        "VirtualBox Oracle VM VirtualBox Extension Pack",
        "VirtualBox VM Selector",
        "VirtualDJ 2021",
        "Visual Studio Code",
        "Visual Studio Enterprise 2019",
        "Visual Studio Installer",
        "VLC media player",
        "VMware Fusion",
        "VMware Horizon Client",
        "VMware Remote Console",
        "VMware Workstation Player",
        "VNC Viewer",
        "VSCodium",
        "Vue.js devtools",
        "WampServer",
        "Webex",
        "WebStorm",
        "WhatsApp",
        "Wireshark",
        "Wolfram Mathematica",
        "WordPress.com",
        "XAMPP",
        "Xcode",
        "Xmind",
        "Zoom",
        "ZoomIt",
    ]
}

#[derive(Debug, Clone)]
pub struct Item {
    pub score: usize,
    pub item: String,
}

impl Item {
    pub fn new(score: usize, item: String) -> Item {
        Self { score, item }
    }
}

pub fn jarowinkler(input: &str, data: &Vec<&str>, min_score: usize) -> Vec<Item> {
    let mut results: Vec<Item> = Vec::with_capacity(data.len());
    for item in data {
        let score = jaro_winkler_distance(input, item, &PrefixLength::Four);
        if score > min_score as f64 {
            results.push(Item::new((score * 100.0) as usize, item.to_string()));
        }
    }
    results.sort_by(|a, b| b.score.cmp(&a.score));
    results
}

pub fn fuzzy_matcher_skimv2(input: &str, data: &Vec<&str>, min_score: usize) -> Vec<Item> {
    let mut results: Vec<Item> = Vec::with_capacity(data.len());
    let matcher = SkimMatcherV2::default();
    for item in data {
        let score = matcher.fuzzy_match(item, input).unwrap_or(0) as usize;
        if score > min_score {
            results.push(Item::new(score, item.to_string()));
        }
    }
    results.sort_by(|a, b| b.score.cmp(&a.score));
    results
}

pub fn fuzzy_matcher_clangd(input: &str, data: &Vec<&str>, min_score: usize) -> Vec<Item> {
    let mut results: Vec<Item> = Vec::with_capacity(data.len());
    let matcher = ClangdMatcher::default();
    for item in data {
        let score = matcher.fuzzy_match(item, input).unwrap_or(0) as usize;
        if score > min_score {
            results.push(Item::new(score, item.to_string()));
        }
    }
    results.sort_by(|a, b| b.score.cmp(&a.score));
    results
}

#[inline]
pub fn setup_corpus(data: &Vec<&str>) -> Corpus {
    let mut corpus = CorpusBuilder::new().arity(2).pad_full(Pad::Auto).finish();
    for item in data {
        corpus.add_text(item);
    }
    corpus
}

pub fn ngrammatic(input: &str, min_score: usize, corpus: &mut Corpus) -> Vec<Item> {
    let results = corpus
        .search(input, (min_score as f32) / 100.0)
        .iter()
        .map(|res| Item::new((res.similarity * 100.0) as usize, res.text.to_owned()))
        .collect();
    results
}

pub fn rust_fuzzy_search(input: &str, data: &Vec<&str>, min_score: usize) -> Vec<Item> {
    let results = fuzzy_search_sorted(input, data)
        .iter()
        .filter(|res| res.1 > (min_score as f32 / 100.0))
        .map(|res| Item::new((res.1 * 100.0) as usize, res.0.to_string()))
        .collect();
    results
}

pub fn sublime_fuzzy(input: &str, data: &Vec<&str>, min_score: usize) -> Vec<Item> {
    let mut results: Vec<Item> = Vec::with_capacity(data.len());
    for item in data {
        let res = best_match(input, item);
        if let Some(res) = res {
            let score = res.score() as usize;
            if score > min_score {
                results.push(Item::new(score, item.to_string()));
            }
        }
    }
    results.sort_by(|a, b| b.score.cmp(&a.score));
    results
}

// TODO: tantivy

pub fn levenshtein_automata() {
    let lev_automaton_builder = LevenshteinAutomatonBuilder::new(2, true);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::min;

    #[test]
    fn test_jarowinkler() {
        let data = get_data();
        let res = jarowinkler("VSCode", &data, 0);
        let res = res[0..min(10, res.len())].to_vec();
        dbg!(res);
    }

    #[test]
    fn test_fuzzy_matcher_skimv2() {
        let data = get_data();
        let res = fuzzy_matcher_skimv2("VSCode", &data, 0);
        let res = res[0..min(10, res.len())].to_vec();
        dbg!(res);
    }

    #[test]
    fn test_fuzzy_matcher_clangd() {
        let data = get_data();
        let res = fuzzy_matcher_clangd("VSCode", &data, 0);
        let res = res[0..min(10, res.len())].to_vec();
        dbg!(res);
    }

    #[test]
    fn test_ngrammatic() {
        let data = get_data();
        let mut corpus = setup_corpus(&data);
        let res = ngrammatic("VSCode", 0, &mut corpus);
        let res = res[0..min(10, res.len())].to_vec();
        dbg!(res);
    }

    #[test]
    fn test_rust_fuzzy_search() {
        let data = get_data();
        let res = rust_fuzzy_search("VSCode", &data, 0);
        let res = res[0..min(10, res.len())].to_vec();
        dbg!(res);
    }

    #[test]
    fn test_sublime_fuzzy() {
        let data = get_data();
        let res = sublime_fuzzy("VSCode", &data, 0);
        let res = res[0..min(10, res.len())].to_vec();
        dbg!(res);
    }
}
