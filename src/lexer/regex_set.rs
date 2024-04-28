use fancy_regex::Regex;


pub struct RegexSet {
    patterns: Vec<Regex>,
}

impl RegexSet {
    pub fn new(patterns: Vec<Regex>) -> Self {
        Self { patterns }
    }

    pub fn matches(&self, haystack: &str) -> SetMatches {
        let mut matches = SetMatches { len: 0, which: vec![] };
        for pat in self.patterns.iter() {
            if pat.is_match(haystack).unwrap() {
                matches.which.push(true);
                matches.len += 1;
            } else {
                matches.which.push(false);
            }
        }
        matches
    }
}


pub struct SetMatches {
    len: usize,
    which: Vec<bool>
}

impl SetMatches {
    pub fn iter(&self) -> PatternSetIter<'_> {
        PatternSetIter { iter: self.which.iter().enumerate() }
    }
}


#[derive(Clone, Debug)]
pub struct PatternSetIter<'a> {
    iter: core::iter::Enumerate<core::slice::Iter<'a, bool>>,
}

impl<'a> Iterator for PatternSetIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        for (index, &matches) in self.iter.by_ref() {
            if matches {
                return Some(index);
            }
        }
        None
    }
}

impl<'a> DoubleEndedIterator for PatternSetIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        while let Some((index, &matches)) = self.iter.next_back() {
            if matches {
                return Some(index);
            }
        }
        None
    }
}
