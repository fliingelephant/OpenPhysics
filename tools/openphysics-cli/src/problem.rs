//! Entry model and parser. The format is defined in docs/DESIGN.md.

use clap::ValueEnum;
use roxmltree::{Document, Node};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Status {
    Open,
    Solved,
}

#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Kind {
    Proof,
    Construction,
}

#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Field {
    #[value(name = "QIT")]
    Qit,
    #[value(name = "QCT")]
    Qct,
    #[value(name = "QTD")]
    Qtd,
    #[value(name = "QMB")]
    Qmb,
}

#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Tag {
    Status,
    Kind,
    Field,
    Name,
    Keys,
    Related,
    Claim,
    Def,
    Known,
    Refs,
    Ask,
    Out,
}

/// Required tag order. `Related` is the only optional tag.
const ORDER: [Tag; 12] = [
    Tag::Status,
    Tag::Kind,
    Tag::Field,
    Tag::Name,
    Tag::Keys,
    Tag::Related,
    Tag::Claim,
    Tag::Def,
    Tag::Known,
    Tag::Refs,
    Tag::Ask,
    Tag::Out,
];

/// Lines before the XML body: heading, blank line, opening fence.
const XML_OFFSET: usize = 3;

fn value_name<T: ValueEnum>(v: T) -> String {
    v.to_possible_value().unwrap().get_name().to_string()
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&value_name(*self))
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&value_name(*self))
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&value_name(*self))
    }
}

pub struct Problem {
    pub path: PathBuf,
    pub id: u32,
    pub status: Status,
    pub kind: Kind,
    pub field: Field,
    pub name: String,
    pub keys: String,
    pub related: Vec<u32>,
    pub claim: String,
    pub def: String,
    pub known: String,
    pub refs: Vec<String>,
    pub ask: String,
    pub out: String,
}

impl Problem {
    pub fn tag(&self, tag: Tag) -> String {
        match tag {
            Tag::Status => self.status.to_string(),
            Tag::Kind => self.kind.to_string(),
            Tag::Field => self.field.to_string(),
            Tag::Name => self.name.clone(),
            Tag::Keys => self.keys.clone(),
            Tag::Related => self.related.iter().map(u32::to_string).collect::<Vec<_>>().join(" "),
            Tag::Claim => self.claim.clone(),
            Tag::Def => self.def.clone(),
            Tag::Known => self.known.clone(),
            Tag::Refs => self.refs.join("\n"),
            Tag::Ask => self.ask.clone(),
            Tag::Out => self.out.clone(),
        }
    }
}

pub struct Issue {
    pub path: PathBuf,
    pub line: usize,
    pub message: String,
}

impl fmt::Display for Issue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}: {}", self.path.display(), self.line, self.message)
    }
}

/// Parse one PROBLEM.md. Stops at the first structural error.
pub fn parse(path: &Path) -> Result<Problem, Issue> {
    let issue = |line: usize, message: String| Issue { path: path.to_path_buf(), line, message };
    let text = fs::read_to_string(path).map_err(|e| issue(0, e.to_string()))?;

    let (heading, rest) = text.split_once('\n').ok_or_else(|| issue(1, "empty file".into()))?;
    let (id, heading_name) = heading
        .strip_prefix("# ")
        .and_then(|h| h.split_once(' '))
        .ok_or_else(|| issue(1, "heading must be `# N Name`".into()))?;
    let id: u32 = id.parse().map_err(|_| issue(1, format!("heading id `{id}` is not an integer")))?;
    let body = rest
        .strip_prefix("\n```xml\n")
        .ok_or_else(|| issue(2, "expected a blank line and then ```xml".into()))?;
    let (xml, tail) = body.split_once("\n```").ok_or_else(|| issue(XML_OFFSET, "missing closing ```".into()))?;
    if !tail.trim().is_empty() {
        let line = XML_OFFSET + xml.lines().count() + 2;
        return Err(issue(line, "content after the closing ```".into()));
    }

    let doc = Document::parse(xml).map_err(|e| {
        let message = e.to_string();
        let message = message.rsplit_once(" at ").map_or(message.as_str(), |(m, _)| m).to_string();
        issue(XML_OFFSET + e.pos().row as usize, message)
    })?;
    let line_of = |node: Node| XML_OFFSET + doc.text_pos_at(node.range().start).row as usize;
    let root = doc.root_element();
    if root.tag_name().name() != "problem" {
        return Err(issue(line_of(root), "root element must be <problem>".into()));
    }
    if root.attribute("id") != Some(id.to_string().as_str()) {
        return Err(issue(line_of(root), format!("<problem id> must be {id}, matching the heading")));
    }

    let mut found: Vec<(Tag, Node)> = Vec::new();
    for node in root.children().filter(Node::is_element) {
        let name = node.tag_name().name();
        let tag = Tag::from_str(name, false).map_err(|_| issue(line_of(node), format!("unknown tag <{name}>")))?;
        found.push((tag, node));
    }
    let has_related = found.iter().any(|(t, _)| *t == Tag::Related);
    let expected: Vec<Tag> = ORDER.iter().copied().filter(|t| *t != Tag::Related || has_related).collect();
    let actual: Vec<Tag> = found.iter().map(|(t, _)| *t).collect();
    if actual != expected {
        let names: Vec<String> = expected.iter().map(|t| value_name(*t)).collect();
        return Err(issue(line_of(root), format!("tags must be exactly, in order: {}", names.join(" "))));
    }

    let text_of = |node: Node| -> Result<String, Issue> {
        if node.children().any(|c| c.is_element()) {
            return Err(issue(line_of(node), format!("<{}> must contain text only", node.tag_name().name())));
        }
        Ok(node.text().unwrap_or("").trim().to_string())
    };
    let mut nodes = found.into_iter();
    let mut next = || nodes.next().unwrap().1;
    let node = next();
    let status = Status::from_str(&text_of(node)?, false).map_err(|e| issue(line_of(node), e))?;
    let node = next();
    let kind = Kind::from_str(&text_of(node)?, false).map_err(|e| issue(line_of(node), e))?;
    let node = next();
    let field = Field::from_str(&text_of(node)?, false).map_err(|e| issue(line_of(node), e))?;
    let node = next();
    let name = text_of(node)?;
    if name != heading_name.trim() {
        return Err(issue(line_of(node), "<name> must equal the heading name".into()));
    }
    let keys = text_of(next())?;
    let mut related = Vec::new();
    if has_related {
        let node = next();
        for word in text_of(node)?.split_whitespace() {
            let r: u32 = word.parse().map_err(|_| issue(line_of(node), format!("related id `{word}` is not an integer")))?;
            if r == id {
                return Err(issue(line_of(node), "entry cannot relate to itself".into()));
            }
            related.push(r);
        }
    }
    let claim = text_of(next())?;
    let def = text_of(next())?;
    let known = text_of(next())?;
    let node = next();
    let mut refs = Vec::new();
    for child in node.children().filter(Node::is_element) {
        if child.tag_name().name() != "ref" {
            return Err(issue(line_of(child), "<refs> may contain only <ref>".into()));
        }
        refs.push(text_of(child)?);
    }
    if refs.is_empty() {
        return Err(issue(line_of(node), "<refs> needs at least one <ref>".into()));
    }
    let ask = text_of(next())?;
    let out = text_of(next())?;

    Ok(Problem { path: path.to_path_buf(), id, status, kind, field, name, keys, related, claim, def, known, refs, ask, out })
}

/// Load every entry under `problems/`, sorted by id, with all issues found.
pub fn load(problems: &Path) -> (Vec<Problem>, Vec<Issue>) {
    let mut entries = Vec::new();
    let mut issues = Vec::new();
    let mut ids = Vec::new();
    let dirs = fs::read_dir(problems).unwrap_or_else(|e| {
        eprintln!("{}: {e}", problems.display());
        std::process::exit(2)
    });
    for dir in dirs.map(Result::unwrap).map(|d| d.path()).filter(|p| p.is_dir()) {
        let path = dir.join("PROBLEM.md");
        let dir_id: Option<u32> = dir.file_name().unwrap().to_str().and_then(|s| s.parse().ok());
        ids.extend(dir_id);
        match (dir_id, parse(&path)) {
            (Some(dir_id), Ok(p)) if p.id == dir_id => entries.push(p),
            (Some(dir_id), Ok(p)) => issues.push(Issue { path, line: 1, message: format!("id {} does not match directory {dir_id}", p.id) }),
            (None, _) => issues.push(Issue { path, line: 0, message: "directory name must be the integer id".into() }),
            (_, Err(e)) => issues.push(e),
        }
    }
    entries.sort_by_key(|p| p.id);
    ids.sort_unstable();
    if let Some((i, id)) = ids.iter().enumerate().find(|(i, id)| **id != *i as u32 + 1) {
        let path = problems.join(id.to_string()).join("PROBLEM.md");
        issues.push(Issue { path, line: 1, message: format!("ids must be contiguous: expected {}, found {id}", i + 1) });
    }
    for p in &entries {
        for r in &p.related {
            if entries.binary_search_by_key(r, |q| q.id).is_err() {
                issues.push(Issue { path: p.path.clone(), line: 1, message: format!("related id {r} does not exist") });
            }
        }
    }
    (entries, issues)
}
