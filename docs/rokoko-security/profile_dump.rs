// The pinned parameter sets live in the RoKoKo binary; copy its `src/instantiation.rs`
// next to this file (see README.md) so the dump reads exactly the chains the benchmark runs.
#[allow(dead_code)]
mod instantiation;
use instantiation::{instantiation, ParamSet};
use rokoko::protocol::config::{Config, Projection};
fn main() {
 eprintln!("sampler_mean_attempts={:.8}",rokoko::common::short_challenge::repetition_rate());
 for set in [ParamSet::P22,ParamSet::P24,ParamSet::P26,ParamSet::P28,ParamSet::P30] {
  let name=set.name(); let inst=instantiation(set).expect("plain chain");
  let mut current=Some(&inst.config); let mut round=0;
  while let Some(c)=current { round+=1;
   match c {
    Config::Sumcheck(s)=> { println!("{},{},{},{},{},{},{},{}",name,round,s.witness_height,s.witness_width,s.projection_ratio,s.projection_height,match &s.projection_recursion {Projection::Skip=>"skip",Projection::Fine(_)=>"fine",Projection::Coarse(_)=>"coarse"},s.basic_commitment_rank); current=s.next.as_deref(); },
    Config::Simple(s)=>{println!("{},{},{},{},{},{},simple,{}",name,round,s.witness_height,s.witness_width,s.projection_ratio,s.projection_height,s.basic_commitment_rank); current=None;},
    Config::Intermediate(_)=>panic!("unexpected intermediate config")
   }
  }
 }
}
