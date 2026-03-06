use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct EvaluationKey {
    pub components: Vec<Dcrt>,
    pub n: usize,
}

#[derive(Clone, Debug)]
pub struct BootstrappingKey {
    pub ek: EvaluationKey,
    pub k_delta: Vec<Dcrt>,
    
    /// Componentes K_{c_i,i} organizados por nível de decomposição
    /// components[i] contém os criptogramas para o i-ésimo nível da decomposição
    pub components: Vec<Vec<Dcrt>>,
    pub k_8: Vec<Dcrt>,
    pub l: usize,
    pub b: u64,
}