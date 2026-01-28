use crate::curie_parser::CurieParser;
use crate::validators::regex_validator::CurieRegexValidator;

macro_rules! define_curie_validators {
    ( $( $fn_name:ident, $const_name:ident => $pattern:literal ),* $(,)? ) => {

        $(
            pub const $const_name: &'static str = $pattern;
        )*

        impl CurieRegexValidator {
            $(
                pub fn $fn_name() -> Self {
                    let regex = regex::Regex::new($const_name)
                        .expect(concat!("Error compiling regex for ", stringify!($const_name)));
                    Self::from(regex)
                }
            )*

            pub fn from_prefix(prefix: &str) -> Option<Self> {
                match prefix.to_lowercase().as_str() {
                    $(
                        stringify!($fn_name) => Some(Self::$fn_name()),
                    )*
                    _ => None,
                }
            }
        }

        impl CurieParser<CurieRegexValidator> {
            $(
                pub fn $fn_name() -> Self {
                    CurieParser {
                        validator: CurieRegexValidator::$fn_name(),
                    }
                }
            )*

            pub fn from_prefix(prefix: &str) -> Option<Self> {
                match prefix.to_lowercase().as_str() {
                    $(
                        stringify!($fn_name) => Some(Self::$fn_name()),
                    )*
                    _ => None,
                }
            }
        }
    };
}

define_curie_validators! {mi, MI_PATTERN => "^MI:\\d{4}$",
// Human Reference Atlas Common Coordinate Framework Ontology
ccf, CCF_PATTERN => "^CCF:\\S+$",
agro, AGRO_PATTERN => "^AGRO:\\d{8}$",
// Homeostasis imbalance process ontology
hoip, HOIP_PATTERN => "^HOIP:\\d{7}$",
sbo, SBO_PATTERN => "^SBO:\\d{7}$",
fbbt, FBBT_PATTERN => "^FBbt:\\d{8}$",
upa, UPA_PATTERN => "^UPA:(UCR|UCY|UER|ULS|UPA|UPC|UPX)\\d{5}$",
// Chemical Entities of Biological Interest
chebi, CHEBI_PATTERN => "^CHEBI:\\d+$",
mro, MRO_PATTERN => "^MRO:\\d{7}$",
lbo, LBO_PATTERN => "^LBO:\\d{7}$",
uo, UO_PATTERN => "^UO:\\d{7}$",
rnao, RNAO_PATTERN => "^RNAO:\\d{7}$",
wbbt, WBBT_PATTERN => "^WBbt:\\d{7}$",
// Gender, Sex, and Sexual Orientation Ontology
gsso, GSSO_PATTERN => "^GSSO:\\d{6}$",
eupath, EUPATH_PATTERN => "^EUPATH:\\d{7}$",
disdriv, DISDRIV_PATTERN => "^DISDRIV:\\d+$",
tto, TTO_PATTERN => "^TTO:\\d+$",
fix, FIX_PATTERN => "^FIX:\\d{7}$",
cl, CL_PATTERN => "^CL:\\d{7}$",
sasap, SASAP_PATTERN => "^SASAP:\\d+$",
ito, ITO_PATTERN => "^ITO:\\d+$",
// Drug Target Ontology
dto, DTO_PATTERN => "^DTO:\\d+$",
oostt, OOSTT_PATTERN => "^OOSTT:\\d{8}$",
// Human Dermatological Disease Ontology
dermo, DERMO_PATTERN => "^DERMO:\\d{7}$",
// Food-Biomarker Ontology
fobi, FOBI_PATTERN => "^FOBI:\\d{4,6}$",
// Microbial Ecophysiological Trait and Phenotype Ontology
metpo, METPO_PATTERN => "^METPO:\\d+$",
// Teleost Anatomy Ontology
tao, TAO_PATTERN => "^TAO:\\d{7}$",
// Minimum Information about any (x) Sequence
mixs, MIXS_PATTERN => "^MIXS:\\d{7}$",
fideo, FIDEO_PATTERN => "^FIDEO:\\d+$",
trans, TRANS_PATTERN => "^TRANS:\\d{7}$",
ms, MS_PATTERN => "^MS:\\d{7}$",
clyh, CLYH_PATTERN => "^CLYH:\\d+$",
ero, ERO_PATTERN => "^ERO:\\d{7}$",
peco, PECO_PATTERN => "^PECO:\\d{7}$",
ohmi, OHMI_PATTERN => "^OHMI:\\d{7}$",
pco, PCO_PATTERN => "^PCO:\\d{7}$",
// Relation Ontology
ro, RO_PATTERN => "^RO:(HOM)?\\d{7}$",
mfoem, MFOEM_PATTERN => "^MFOEM:\\d{6}$",
ehda, EHDA_PATTERN => "^EHDA:\\d+$",
mf, MF_PATTERN => "^MF:\\d{7}$",
lepao, LEPAO_PATTERN => "^LEPAO:\\d{7}$",
cdao, CDAO_PATTERN => "^CDAO:\\d{7}$",
chmo, CHMO_PATTERN => "^CHMO:\\d{7}$",
// HUGO Gene Nomenclature Committee
hgnc, HGNC_PATTERN => "^HGNC:\\d{1,5}$",
// PhenX Toolkit
phenx, PHENX_PATTERN => "^PHENX:\\d+$",
mpath, MPATH_PATTERN => "^MPATH:\\d+$",
mco, MCO_PATTERN => "^MCO:\\d+$",
aeo, AEO_PATTERN => "^AEO:\\d{7}$",
ppo, PPO_PATTERN => "^PPO:\\d{7}$",
mcro, MCRO_PATTERN => "^MCRO:\\d{7}$",
obcs, OBCS_PATTERN => "^OBCS:\\d{7}$",
// Livestock Product Trait Ontology
lpt, LPT_PATTERN => "^LPT:\\d+$",
aro, ARO_PATTERN => "^ARO:\\d{7}$",
salmon, SALMON_PATTERN => "^SALMON:\\d+$",
txpo, TXPO_PATTERN => "^TXPO:\\d{7}$",
// Online Mendelian Inheritance in Man
omim, OMIM_PATTERN => "^OMIM:\\d+$",
zfs, ZFS_PATTERN => "^ZFS:\\d{7}$",
// National Drug Data File
nddf, NDDF_PATTERN => "^NDDF:\\d{6}$",
xao, XAO_PATTERN => "^XAO:\\d{7}$",
spd, SPD_PATTERN => "^SPD:\\d{7}$",
zp, ZP_PATTERN => "^ZP:\\d+$",
bcio, BCIO_PATTERN => "^BCIO:\\d{6}$",
psdo, PSDO_PATTERN => "^PSDO:\\d{7}$",
go, GO_PATTERN => "^GO:\\d{7}$",
opb, OPB_PATTERN => "^OPB:\\d+$",
// Mondo Disease Ontology
mondo, MONDO_PATTERN => "^MONDO:\\d{7}$",
mamo, MAMO_PATTERN => "^MAMO:\\d{7}$",
ncro, NCRO_PATTERN => "^NCRO:\\d{7}$",
htn, HTN_PATTERN => "^HTN:\\d{8}$",
// Agronomy Vocabulary
agrovoc, AGROVOC_PATTERN => "^AGROVOC:[a-z0-9]+$",
ordo, ORDO_PATTERN => "^ORDO:C?\\d+$",
roleo, ROLEO_PATTERN => "^ROLEO:\\d{7}$",
bco, BCO_PATTERN => "^BCO:\\d{7}$",
apo, APO_PATTERN => "^APO:\\d{7}$",
ceph, CEPH_PATTERN => "^CEPH:\\d{7}$",
cio, CIO_PATTERN => "^CIO:\\d{7}$",
ecso, ECSO_PATTERN => "^ECSO:\\d+$",
dcm, DCM_PATTERN => "^DCM:\\d+$",
mp, MP_PATTERN => "^MP:\\d{7}$",
fbcv, FBCV_PATTERN => "^FBcv:\\d{7}$",
exo, EXO_PATTERN => "^ExO:\\d{7}$",
rs, RS_PATTERN => "^RS:\\d{7}$",
obib, OBIB_PATTERN => "^OBIB:\\d{7}$",
ohd, OHD_PATTERN => "^OHD:\\d{7}$",
omrse, OMRSE_PATTERN => "^OMRSE:\\d{8}$",
omo, OMO_PATTERN => "^OMO:\\d{7}$",
cteno, CTENO_PATTERN => "^CTENO:\\d{7}$",
mfmo, MFMO_PATTERN => "^MFMO:\\d{7}$",
ontoneo, ONTONEO_PATTERN => "^ONTONEO:\\d{8}$",
labo, LABO_PATTERN => "^LABO:\\d{7}$",
clo, CLO_PATTERN => "^CLO:\\d{7}$",
phipo, PHIPO_PATTERN => "^PHIPO:\\d{7}$",
vso, VSO_PATTERN => "^VSO:\\d{7}$",
wbls, WBLS_PATTERN => "^WBls:\\d{7}$",
aero, AERO_PATTERN => "^AERO:\\d{7}$",
hsapdv, HSAPDV_PATTERN => "^HsapDv:\\d{7}$",
taxrank, TAXRANK_PATTERN => "^TAXRANK:\\d{7}$",
// Arctic Data Center Academic Disciplines Ontology
adcad, ADCAD_PATTERN => "^ADCAD:\\d{5}$",
// International Classification of Diseases, 10th Revision, Clinical Modification
icd10cm, ICD10CM_PATTERN => "^ICD10CM:([A-Z][0-9][0-9AB]((-[A-Z][0-9][0-9AB])|(\\.[0-9A-KXZ]([0-9A-EXYZ]([0-9A-HX][0-59A-HJKMNP-S]?)?)?)?))$",
vto, VTO_PATTERN => "^VTO:\\d{7}$",
// Foundational Model of Anatomy
fma, FMA_PATTERN => "^FMA:\\d+$",
tgma, TGMA_PATTERN => "^TGMA:\\d{7}$",
// NFDI MatWerk Ontology
mwo, MWO_PATTERN => "^MWO:\\d{7}$",
hom, HOM_PATTERN => "^HOM:\\d{7}$",
// A nomenclatural ontology for biological names
nomen, NOMEN_PATTERN => "^NOMEN:\\d{7}$",
// Molecular Process Ontology
mop, MOP_PATTERN => "^MOP:\\d{7}$",
dideo, DIDEO_PATTERN => "^DIDEO:\\d{8}$",
xpo, XPO_PATTERN => "^XPO:\\d+$",
micro, MICRO_PATTERN => "^MICRO:\\d{7}$",
to, TO_PATTERN => "^TO:\\d{7}$",
rex, REX_PATTERN => "^REX:\\d{7}$",
// Electrocardiogram Ontology
ecg, ECG_PATTERN => "^ECG:\\d+$",
ornaseq, ORNASEQ_PATTERN => "^ORNASEQ:\\d{7}$",
occo, OCCO_PATTERN => "^OCCO:\\d+$",
cob, COB_PATTERN => "^COB:\\d{7}$",
opl, OPL_PATTERN => "^OPL:\\d{7}$",
cvdo, CVDO_PATTERN => "^CVDO:\\d{7}$",
// Reagent Ontology
reo, REO_PATTERN => "^REO:\\d{7}$",
// NIF Standard Ontology
nifstd, NIFSTD_PATTERN => "^NIFSTD:BAMSC\\d+$",
ovae, OVAE_PATTERN => "^OVAE:\\d{7}$",
omiabis, OMIABIS_PATTERN => "^OMIABIS:\\d{7}$",
// Vertebrate Homologous Organ Group Ontology
vhog, VHOG_PATTERN => "^VHOG:\\d{7}$",
// UMLS Semantic Types Ontology
sty, STY_PATTERN => "^STY:T\\d{3}$",
miro, MIRO_PATTERN => "^MIRO:\\d{8}$",
ato, ATO_PATTERN => "^ATO:\\d{7}$",
// Coleoptera Anatomy Ontology
colao, COLAO_PATTERN => "^COLAO:\\d{7}$",
so, SO_PATTERN => "^SO:\\d{7}$",
wbphenotype, WBPHENOTYPE_PATTERN => "^WBPhenotype:\\d{7}$",
// Uber Anatomy Ontology
uberon, UBERON_PATTERN => "^UBERON:\\d+$",
geo, GEO_PATTERN => "^GEO:\\d{9}$",
cmpo, CMPO_PATTERN => "^CMPO:\\d{7}$",
// Veterans Administration National Drug File
vandf, VANDF_PATTERN => "^VANDF:\\d+$",
// Ontology for Avida digital evolution platform
ontoavida, ONTOAVIDA_PATTERN => "^ONTOAVIDA:\\d{8}$",
// International Classification of Functioning, Disability and Health
icf, ICF_PATTERN => "^ICF:\\d+$",
teddy, TEDDY_PATTERN => "^TEDDY:\\d+$",
pw, PW_PATTERN => "^PW:\\d{7}$",
// Planarian Anatomy and Schmidtea mediterranean Developmental Stage Ontology
plana, PLANA_PATTERN => "^PLANA:\\d{7}$",
emap, EMAP_PATTERN => "^EMAP:\\d+$",
// Glycan Naming and Subsumption Ontology
gno, GNO_PATTERN => "^GNO:(\\d{8}|(\\w+\\d+\\w+))$",
vt, VT_PATTERN => "^VT:\\d{7}$",
xco, XCO_PATTERN => "^XCO:\\d{7}$",
hso, HSO_PATTERN => "^HSO:\\d{7}$",
pdro, PDRO_PATTERN => "^PDRO:\\d{7}$",
sepio, SEPIO_PATTERN => "^SEPIO:\\d{7}$",
bfo, BFO_PATTERN => "^BFO:\\d{7}$",
epso, EPSO_PATTERN => "^EPSO:\\d{7}$",
// Common Terminology Criteria for Adverse Events
ctcae, CTCAE_PATTERN => "^CTCAE:E\\d+$",
stato, STATO_PATTERN => "^STATO:\\d{7}$",
vido, VIDO_PATTERN => "^VIDO:\\d{7}$",
ons, ONS_PATTERN => "^ONS:\\d{7}$",
envo, ENVO_PATTERN => "^ENVO:\\d{7,8}$",
proco, PROCO_PATTERN => "^PROCO:\\d{7}$",
// Chemical Information Ontology
cheminf, CHEMINF_PATTERN => "^CHEMINF:\\d{6}$",
// Name Reaction Ontology
rxno, RXNO_PATTERN => "^RXNO:\\d{7}$",
ecto, ECTO_PATTERN => "^ECTO:\\d{7}$",
po, PO_PATTERN => "^PO:\\d+$",
ohpi, OHPI_PATTERN => "^OHPI:\\d+$",
// SBGN Bricks data and ontology
bko, BKO_PATTERN => "^BKO:\\d+$",
// Provenance, Authoring, and Versioning Vocabulary
pav, PAV_PATTERN => "^PAV:[a-z][a-zA-Z]+$",
nmr, NMR_PATTERN => "^NMR:\\d+$",
ecocore, ECOCORE_PATTERN => "^ECOCORE:\\d+$",
hancestro, HANCESTRO_PATTERN => "^HANCESTRO:\\d{4}$",
mfo, MFO_PATTERN => "^MFO:\\d{7}$",
gecko, GECKO_PATTERN => "^GECKO:\\d{7}$",
cryoem, CRYOEM_PATTERN => "^CRYOEM:\\d{7}$",
ogms, OGMS_PATTERN => "^OGMS:\\d{7}$",
// Medical Subject Headings
mesh, MESH_PATTERN => "^MESH:(C|D|Q)\\d+$",
tads, TADS_PATTERN => "^TADS:\\d{7}$",
// Variation Ontology
vario, VARIO_PATTERN => "^VariO:\\d+$",
zea, ZEA_PATTERN => "^ZEA:\\d{7}$",
sibo, SIBO_PATTERN => "^SIBO:\\d{7}$",
dron, DRON_PATTERN => "^DRON:\\d{8}$",
ddanat, DDANAT_PATTERN => "^DDANAT:\\d{7}$",
hao, HAO_PATTERN => "^HAO:\\d{7}$",
// Ontology for the Anatomy of the Insect SkeletoMuscular system
aism, AISM_PATTERN => "^AISM:\\d{7}$",
emapa, EMAPA_PATTERN => "^EMAPA:\\d+$",
ngbo, NGBO_PATTERN => "^NGBO:\\d{7}$",
maxo, MAXO_PATTERN => "^MAXO:\\d{7}$",
// International Classification of Diseases, 10th Revision
icd10, ICD10_PATTERN => "^ICD10:(([XVI]+)|([A-Z][0-9]+((-[A-Z][0-9]+)|(\\.[0-9]))?))$",
sdgio, SDGIO_PATTERN => "^SDGIO:\\d{8}$",
// PLOS Thesaurus
plosthes, PLOSTHES_PATTERN => "^PLOSTHES:\\d+$",
cido, CIDO_PATTERN => "^CIDO:\\d{7}$",
gaz, GAZ_PATTERN => "^GAZ:\\d{8}$",
ogi, OGI_PATTERN => "^OGI:\\d{7}$",
ehdaa2, EHDAA2_PATTERN => "^EHDAA2:\\d{7}$",
// Chemical Analysis Ontology
cao, CAO_PATTERN => "^CAO:\\d+$",
// Dengue Fever Ontology
idoden, IDODEN_PATTERN => "^IDODEN:\\d{7}$",
iceo, ICEO_PATTERN => "^ICEO:\\d{7}(_\\d)?$",
eol, EOL_PATTERN => "^EOL:\\d{7}$",
cdno, CDNO_PATTERN => "^CDNO:\\d{7}$",
planp, PLANP_PATTERN => "^PLANP:\\d+$",
oba, OBA_PATTERN => "^OBA:(VT)?\\d{7}$",
ehdaa, EHDAA_PATTERN => "^EHDAA:\\d+$",
pato, PATO_PATTERN => "^PATO:\\d{7}$",
ogg, OGG_PATTERN => "^OGG:\\d+$",
doid, DOID_PATTERN => "^DOID:\\d+$",
// Protein Ontology
pr, PR_PATTERN => "^PR:(?:\\d{9}|[OPQ][0-9][A-Z0-9]{3}[0-9](?:-\\d+)?|[A-NR-Z][0-9](?:[A-Z][A-Z0-9]{2}[0-9]){1,2}(?:-\\d+)?)$",
// BRENDA Tissue Ontology
bto, BTO_PATTERN => "^BTO:\\d{7}$",
vbo, VBO_PATTERN => "^VBO:\\d{7}$",
mmusdv, MMUSDV_PATTERN => "^MmusDv:\\d{7}$",
// terms4FAIRskills
t4fs, T4FS_PATTERN => "^T4FS:\\d{7}$",
// Biomedical Informatics Research Network Lexicon
birnlex, BIRNLEX_PATTERN => "^BIRNLEX:\\d+$",
fovt, FOVT_PATTERN => "^FOVT:\\d{7}$",
pdumdv, PDUMDV_PATTERN => "^PdumDv:\\d{7}$",
// Clinical Trials Ontology
cto, CTO_PATTERN => "^CTO:\\d{7}$",
kisao, KISAO_PATTERN => "^KISAO:\\d+$",
// Minimum Anformation About a Phylogenetic Analysis Ontology
miapa, MIAPA_PATTERN => "^MIAPA:\\d{7}$",
oarcs, OARCS_PATTERN => "^OARCS:\\d{7}$",
// Subcellular Anatomy Ontology
sao, SAO_PATTERN => "^SAO:\\d+$",
// Vertebrate Skeletal Anatomy Ontology
vsao, VSAO_PATTERN => "^VSAO:\\d{7}$",
zeco, ZECO_PATTERN => "^ZECO:\\d{7}$",
pcl, PCL_PATTERN => "^PCL:\\d{7}$",
// MOSAiC Ontology
mosaic, MOSAIC_PATTERN => "^MOSAIC:\\d{8}$",
mpio, MPIO_PATTERN => "^MPIO:\\d{7}$",
scdo, SCDO_PATTERN => "^SCDO:\\d{7}$",
duo, DUO_PATTERN => "^DUO:\\d{7}$",
vo, VO_PATTERN => "^VO:\\d{7}$",
rbo, RBO_PATTERN => "^RBO:\\d{6,8}$",
eco, ECO_PATTERN => "^ECO:\\d{7}$",
ico, ICO_PATTERN => "^ICO:\\d{7}$",
// NCI Thesaurus
ncit, NCIT_PATTERN => "^NCIT:[CRPA]\\d+$",
cmo, CMO_PATTERN => "^CMO:\\d{7}$",
// NanoParticle Ontology
npo, NPO_PATTERN => "^NPO:\\d+$",
flu, FLU_PATTERN => "^FLU:\\d{7}$",
ddpheno, DDPHENO_PATTERN => "^DDPHENO:\\d{7}$",
poro, PORO_PATTERN => "^PORO:\\d{7}$",
// International Classification of Diseases, 9th Revision, Clinical Modification
icd9cm, ICD9CM_PATTERN => "^ICD9CM:([\\dA-Z]\\d{2}(\\.\\d{1,3}|))|(\\d{2}(\\.\\d{1,2}|))$",
ma, MA_PATTERN => "^MA:\\d+$",
xlmod, XLMOD_PATTERN => "^XLMOD:\\d{5}$",
olatdv, OLATDV_PATTERN => "^OlatDv:\\d{7}$",
// Costal and Marine Ecological Classification Standard
cmecs, CMECS_PATTERN => "^CMECS:\\d+$",
caro, CARO_PATTERN => "^CARO:\\d{7}$",
clao, CLAO_PATTERN => "^CLAO:\\d{7}$",
bspo, BSPO_PATTERN => "^BSPO:\\d{7}$",
epio, EPIO_PATTERN => "^EPIO:\\d{7}$",
fao, FAO_PATTERN => "^FAO:\\d{7}$",
idomal, IDOMAL_PATTERN => "^IDOMAL:(5?)\\d{7}$",
symp, SYMP_PATTERN => "^SYMP:\\d{7}$",
cco, CCO_PATTERN => "^CCO:\\w+$",
// NCBI Taxonomy
ncbitaxon, NCBITAXON_PATTERN => "^NCBITaxon:(\\d+)|([a-zA-Z_]+)$",
fbdv, FBDV_PATTERN => "^FBdv:\\d{8}$",
amphx, AMPHX_PATTERN => "^AMPHX:\\d+$",
// WikiPathways GPML Vocabulary
gpml, GPML_PATTERN => "^GPML:[A-Za-z]+$",
opmi, OPMI_PATTERN => "^OPMI:\\d{7}$",
ado, ADO_PATTERN => "^ADO:\\d{7}$",
genepio, GENEPIO_PATTERN => "^GENEPIO:\\d{7}$",
sep, SEP_PATTERN => "^SEP:\\d{5,6}$",
senso, SENSO_PATTERN => "^SENSO:\\d+$",
ogsf, OGSF_PATTERN => "^OGSF:\\d{7}$",
mfomd, MFOMD_PATTERN => "^MFOMD:\\d{7}$",
obi, OBI_PATTERN => "^OBI:\\d{7}$",
cro, CRO_PATTERN => "^CRO:\\d{7}$",
nbo, NBO_PATTERN => "^NBO:\\d{7}$",
omp, OMP_PATTERN => "^OMP:\\d{7}$",
ino, INO_PATTERN => "^INO:\\d{7}$",
// RSNA Informatics RadLex
radlex, RADLEX_PATTERN => "^RADLEX:RID\\d+$",
zfa, ZFA_PATTERN => "^ZFA:\\d{7}$",
swo, SWO_PATTERN => "^SWO:\\d{7,8}$",
one, ONE_PATTERN => "^ONE:\\d{7}$",
bcgo, BCGO_PATTERN => "^BCGO:\\d{7}$",
nando, NANDO_PATTERN => "^NANDO:\\d+$",
// RxNorm
rxnorm, RXNORM_PATTERN => "^RXNORM:[0-9]{1,7}$",
ido, IDO_PATTERN => "^IDO:\\d{7}$",
flopo, FLOPO_PATTERN => "^FLOPO:\\d{7}$",
// Fission Yeast Phenotype Ontology
fypo, FYPO_PATTERN => "^FYPO:\\d{7}$",
mmo, MMO_PATTERN => "^MMO:\\d{7}$",
pso, PSO_PATTERN => "^PSO:\\d{7}$",
ecao, ECAO_PATTERN => "^ECAO:\\d{7}$",
geno, GENO_PATTERN => "^GENO:\\d{7}$",
apollo_sv, APOLLO_SV_PATTERN => "^APOLLO_SV:\\d{8}$",
fbbi, FBBI_PATTERN => "^FBbi:\\d+$",
// Biological and Environmental Research Variable Ontology
bervo, BERVO_PATTERN => "^BERVO:\\d{7}$",
gallont, GALLONT_PATTERN => "^GALLONT:\\d{7}$",
enm, ENM_PATTERN => "^ENM:\\d+$",
oae, OAE_PATTERN => "^OAE:\\d{7}$",
atol, ATOL_PATTERN => "^ATOL:\\d{7}$",
// Logical Observation Identifiers Names and Codes
loinc, LOINC_PATTERN => "^LOINC:(\\d|\\w)+-\\d$",
// Human Phenotype Ontology
hp, HP_PATTERN => "^HP:\\d{7}$",
// The Food Ontology
foodon, FOODON_PATTERN => "^FOODON:[0-9]{8}$",
sio, SIO_PATTERN => "^SIO:\\d{6}$",
// Minimal Anatomical Terminology
mat, MAT_PATTERN => "^MAT:\\d{7}$",
// Current Procedural Terminology
cpt, CPT_PATTERN => "^CPT:\\d+$",
}
