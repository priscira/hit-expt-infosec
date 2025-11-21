use std::collections::{HashMap, HashSet};


#[derive(Debug)]
pub struct WuManber {
  pats: Vec<Vec<u8>>,
  m: usize,
  b: usize,
  other: usize,
  shift: HashMap<Vec<u8>, usize>,
  hash_prefix: HashMap<(Vec<u8>, Vec<u8>), Vec<Vec<u8>>>,
}


impl WuManber {
  pub fn new<P: AsRef<[u8]>>(pats: &[P], b: Option<usize>) -> Result<Self, String> {
    let pats: Vec<Vec<u8>> = pats.iter().map(|pati| pati.as_ref().to_vec()).collect();
    let m = pats.iter().map(|pati| pati.len()).min().unwrap();
    let b = match b {
      Some(val) => val,
      None => m.min(2),
    };

    if m < b {
      return Err("Pattern length must be greater than or equal to block size.".into());
    }

    let other = m - b + 1;
    let shift = Self::build_shift(&pats, m, b, other);
    let hash_prefix = Self::build_hash_prefix(&pats, m, b, &shift);

    Ok(WuManber { pats, m, b, other, shift, hash_prefix })
  }

  fn build_shift(pats: &[Vec<u8>], m: usize, b: usize, other: usize) -> HashMap<Vec<u8>, usize> {
    let mut blks: HashSet<Vec<u8>> = HashSet::new();
    for pati in &pats.to_vec() {
      for i in 0..=m - b {
        blks.insert(pati[i..i + b].to_vec());
      }
    }

    let mut shifts: HashMap<Vec<u8>, usize> = HashMap::new();
    for blki in blks {
      let mut better = other;
      for pati in &pats.to_vec() {
        if let Some(idx) = find_subslice_left(&pati[..m], &blki) {
          better = better.min(m - idx - b);
        }
      }
      shifts.insert(blki, better);
    }
    shifts
  }

  fn build_hash_prefix(
    pats: &[Vec<u8>], m: usize, b: usize, shift: &HashMap<Vec<u8>, usize>,
  ) -> HashMap<(Vec<u8>, Vec<u8>), Vec<Vec<u8>>> {
    let mut hash_prefix: HashMap<(Vec<u8>, Vec<u8>), Vec<Vec<u8>>> = HashMap::new();
    let zero_shifts: HashSet<&Vec<u8>> = shift.iter()
      .filter(|(_, shift_v)| **shift_v == 0)
      .map(|(shift_k, _)| shift_k)
      .collect();

    for pati in &pats.to_vec() {
      let suf_blk = pati[m - b..m].to_vec();
      if zero_shifts.contains(&suf_blk) {
        let pre_blk = pati[..b].to_vec();
        hash_prefix.entry((suf_blk, pre_blk)).or_default().push(pati.clone());
      }
    }
    hash_prefix
  }

  pub fn search(&self, text: &[u8]) -> HashMap<Vec<u8>, Vec<(usize, usize)>> {
    let mut results: HashMap<Vec<u8>, Vec<(usize, usize)>> =
      self.pats.iter().cloned().map(|pati| (pati, Vec::new())).collect();

    let textl = text.len();
    if textl < self.b {
      return results;
    }

    let mut site: usize = self.m - self.b;
    while site <= textl - self.b {
      let now_suffix: &[u8] = &text[site..site + self.b];
      let mut step: usize = match self.shift.get(now_suffix) {
        Some(&shift_val) => {
          if shift_val == 0 {
            // 原文中对应的模式前缀块的字符子串 起始位置
            let win_start = site - (self.m - self.b);
            let now_prefix: &[u8] = &text[win_start..win_start + self.b];
            if let Some(inner) = self.hash_prefix.get(&(now_suffix.to_vec(), now_prefix.to_vec())) {
              for now_pat in inner {
                let result_start = win_start;
                let result_finish = result_start + now_pat.len();
                if result_finish <= textl && &text[result_start..result_finish] == now_pat.as_slice() {
                  results.get_mut(now_pat).map(
                    |result_val| result_val.push((result_start, result_finish))
                  );
                }
              }
            }
            1
          } else {
            shift_val
          }
        }
        None => self.other,
      };

      if step == 0 {
        step = 1;
      }
      site += step;
    }

    results
  }
}


#[derive(Debug)]
pub struct DHSWuManber {
  base: WuManber,
  slip: HashMap<Vec<u8>, usize>,
}


impl DHSWuManber {
  pub fn new<P: AsRef<[u8]>>(pats: &[P], blk_size: Option<usize>) -> Result<Self, String> {
    let base = WuManber::new(pats, blk_size)?;
    let slip = Self::build_slip(&base);
    Ok(DHSWuManber { base, slip })
  }

  fn build_slip(base: &WuManber) -> HashMap<Vec<u8>, usize> {
    let mut slip: HashMap<Vec<u8>, usize> = HashMap::new();
    let pats_m: Vec<&[u8]> = base.pats.iter().map(|pati| &pati[..base.m]).collect();

    for (shift_k, &shift_v) in &base.shift {
      if shift_v == 0 {
        let mut slip_val = base.other;
        let mut better_idx: isize = -1;
        for pati_m in &pats_m {
          if let Some(idx) = find_subslice_right(pati_m, &shift_k) {
            if idx != (base.m - base.b) && (idx as isize) > better_idx {
              better_idx = idx as isize;
            }
          }
        }
        if better_idx != -1 {
          slip_val = base.m - (better_idx as usize);
        }
        slip.insert(shift_k.clone(), slip_val);
      }
    }
    slip
  }

  pub fn search(&self, text: &[u8]) -> HashMap<Vec<u8>, Vec<(usize, usize)>> {
    let mut results: HashMap<Vec<u8>, Vec<(usize, usize)>> =
      self.base.pats.iter().cloned().map(|pati| (pati, Vec::new())).collect();

    let textl = text.len();
    if textl < self.base.b {
      return results;
    }

    let mut site: usize = self.base.m - self.base.b;
    while site <= textl - self.base.b {
      let now_suffix: &[u8] = &text[site..site + self.base.b];
      let mut step = match self.base.shift.get(now_suffix) {
        Some(&shift_val) => {
          if shift_val == 0 {
            let slip_step: usize = *self.slip.get(now_suffix).unwrap_or(&1);
            // 原文中对应的模式前缀块的字符子串 起始位置
            let win_start = site - (self.base.m - self.base.b);
            let now_prefix: &[u8] = &text[win_start..win_start + self.base.b];
            if let Some(inner) = self.base.hash_prefix.get(&(now_suffix.to_vec(), now_prefix.to_vec())) {
              for now_pat in inner {
                let result_start = win_start;
                let result_finish = result_start + now_pat.len();
                if result_finish <= textl && &text[result_start..result_finish] == now_pat.as_slice() {
                  results.get_mut(now_pat).map(
                    |result_val| result_val.push((result_start, result_finish)));
                }
              }
            }
            slip_step
          } else {
            shift_val
          }
        }
        None => self.base.other,
      };

      if step == 0 {
        step = 1;
      }
      site += step;
    }
    results
  }
}


/// 在字节序列中查找目标片段的第一个出现位置
///
/// ## 参数
/// - `hay`：原始字节序列
/// - `needle`：目标子字节片段
///
/// ## 返回
/// 子片段在字节序列中的第一个出现位置
fn find_subslice_left(hay: &[u8], needle: &[u8]) -> Option<usize> {
  if needle.is_empty() {
    return Some(0);
  }
  if needle.len() > hay.len() {
    return None;
  }
  hay.windows(needle.len()).position(|hay_window| hay_window == needle)
}


/// 在字节序列中从右查找目标片段的第一个出现位置
///
/// ## 参数
/// - `hay`：原始字节序列
/// - `needle`：目标子字节片段
///
/// ## 返回
/// 子片段在字节序列中的右侧第一个出现位置
fn find_subslice_right(hay: &[u8], needle: &[u8]) -> Option<usize> {
  if needle.is_empty() {
    return Some(hay.len());
  }
  if needle.len() > hay.len() {
    return None;
  }
  for i in (0..=hay.len() - needle.len()).rev() {
    if &hay[i..i + needle.len()] == needle {
      return Some(i);
    }
  }
  None
}


/// Helper: 将结果打印为更友好的 UTF-8（若无法解码则打印 bytes）
fn print_matches(res: &HashMap<Vec<u8>, Vec<(usize, usize)>>, text: &[u8]) {
  for (pat, vecpos) in res {
    match String::from_utf8(pat.clone()) {
      Ok(s) => {
        println!("pattern {:?} => occurrences: {}", s, vecpos.len());
      }
      Err(_) => {
        println!("pattern {:?} (bytes) => occurrences: {}", pat, vecpos.len());
      }
    }
    for (s, e) in vecpos {
      match std::str::from_utf8(&text[*s..*e]) {
        Ok(snippet) => println!("  at {}..{} => {:?}", s, e, snippet),
        Err(_) => println!("  at {}..{} => bytes{:?}", s, e, &text[*s..*e]),
      }
    }
  }
}


pub fn test_wu_manber() {
  let patterns = ["still", "trill", "study", "basic", "stability"];
  let patterns_bytes: Vec<&[u8]> = patterns.iter().map(|s| s.as_bytes()).collect();
  let text = b"this chapter will introduce the basic concepts about stability and study";

  let wm = WuManber::new(&patterns_bytes, None).unwrap();
  let res_wm = wm.search(text);
  println!("WM matches:");
  print_matches(&res_wm, text);

  println!("SHIFT: {:?}", wm.shift);
  println!("HASH_PREFIX: {:?}", wm.hash_prefix);
  println!("==============================");

  let dhs = DHSWuManber::new(&patterns_bytes, None).unwrap();
  let res_dhs = dhs.search(text);
  println!("DHSWM matches:");
  print_matches(&res_dhs, text);

  println!("SHIFT: {:?}", dhs.base.shift);
  println!("HASH_PREFIX: {:?}", dhs.base.hash_prefix);
  println!("SLIP: {:?}", dhs.slip);
  println!("==============================");
}
