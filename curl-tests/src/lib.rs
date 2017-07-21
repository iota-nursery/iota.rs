#![allow(unused_imports)]
#![feature(alloc)]

extern crate iota_trytes as trytes;
extern crate iota_curl as curl;

extern crate core;
extern crate alloc;

use alloc::Vec;
use core::fmt;

use curl::*;
use trytes::*;

pub trait TransformerFn<A> {
    fn transform(&self, trits: &[Trit]) -> Vec<A>;
}

mod inner {
    use super::*;
    use trytes::*;

    fn test_hash_eq<A, B>(trans: &[A], expected: &[A])
    where
        A: Copy + Clone + Sized,
        Vec<A>: fmt::Debug + PartialEq,
        B: Curl<A>,
    {

        let mut curl = B::default();
        curl.absorb(trans);
        let mut hash: Vec<A> = Vec::with_capacity(HASH_LENGTH);
        unsafe {
            hash.set_len(HASH_LENGTH);
        }
        curl.squeeze(hash.as_mut_slice());

        assert_eq!(hash, expected.to_vec());

        curl.reset();
        /*let mut duplex_hash : Vec<A> = Vec::with_capacity((trans.len() ))
        curl.duplex(trans, hash.as_mut_slice());
        assert_eq!(trans.len(), hash.len());
        assert_ne!(trans.to_vec(), hash);*/
    }

    pub fn hash_works1<A, B>(transformer: &TransformerFn<A>)
    where
        A: Copy + PartialEq + fmt::Debug,
        B: Curl<A>,
    {

        let trans: Vec<Trit> = "RSWWSFXPQJUBJROQBRQZWZXZJWMUBVIVMHPPTYSNW9YQIQQF9RCSJJCVZG9Z\
                                   WITXNCSBBDHEEKDRBHVTWCZ9SZOOZHVBPCQNPKTWFNZAWGCZ9QDIMKRVINMI\
                                   RZBPKRKQAIPGOHBTHTGYXTBJLSURDSPEOJ9UKJECUKCCPVIQQHDUYKVKISCE\
                                   IEGVOQWRBAYXWGSJUTEVG9RPQLPTKYCRAJ9YNCUMDVDYDQCKRJOAPXCSUDAJ\
                                   GETALJINHEVNAARIPONBWXUOQUFGNOCUSSLYWKOZMZUKLNITZIFXFWQAYVJC\
                                   VMDTRSHORGNSTKX9Z9DLWNHZSMNOYTU9AUCGYBVIITEPEKIXBCOFCMQPBGXY\
                                   JKSHPXNUKFTXIJVYRFILAVXEWTUICZCYYPCEHNTK9SLGVL9RLAMYTAEPONCB\
                                   HDXSEQZOXO9XCFUCPPMKEBR9IEJGQOPPILHFXHMIULJYXZJASQEGCQDVYFOM\
                                   9ETXAGVMSCHHQLFPATWOSMZIDL9AHMSDCE9UENACG9OVFAEIPPQYBCLXDMXX\
                                   A9UBJFQQBCYKETPNKHNOUKCSSYLWZDLKUARXNVKKKHNRBVSTVKQCZL9RY9BD\
                                   TDTPUTFUBGRMSTOTXLWUHDMSGYRDSZLIPGQXIDMNCNBOAOI9WFUCXSRLJFIV\
                                   TIPIAZUK9EDUJJ9B9YCJEZQQELLHVCWDNRH9FUXDGZRGOVXGOKORTCQQA9JX\
                                   NROLETYCNLRMBGXBL9DQKMOAZCBJGWLNJLGRSTYBKLGFVRUF9QOPZVQFGMDJ\
                                   A9TBVGFJDBAHEVOLW9GNU9NICLCQJBOAJBAHHBZJGOFUCQMBGYQLCWNKSZPP\
                                   BQMSJTJLM9GXOZHTNDLGIRCSIJAZTENQVQDHFSOQM9WVNWQQJNOPZMEISSCL\
                                   OADMRNWALBBSLSWNCTOSNHNLWZBVCFIOGFPCPRKQSRGKFXGTWUSCPZSKQNLQ\
                                   JGKDLOXSBJMEHQPDZGSENUKWAHRNONDTBLHNAKGLOMCFYRCGMDOVANPFHMQR\
                                   FCZIQHCGVORJJNYMTORDKPJPLA9LWAKAWXLIFEVLKHRKCDG9QPQCPGVKIVBE\
                                   NQJTJGZKFTNZHIMQISVBNLHAYSSVJKTIELGTETKPVRQXNAPWOBGQGFRMMK9U\
                                   QDWJHSQMYQQTCBMVQKUVGJEAGTEQDN9TCRRAZHDPSPIYVNKPGJSJZASZQBM9\
                                   WXEDWGAOQPPZFLAMZLEZGXPYSOJRWL9ZH9NOJTUKXNTCRRDO9GKULXBAVDRI\
                                   ZBOKJYVJUSHIX9F9O9ACYCAHUKBIEPVZWVJAJGSDQNZNWLIWVSKFJUMOYDMV\
                                   UFLUXT9CEQEVRFBJVPCTJQCORM9JHLYFSMUVMFDXZFNCUFZZIKREIUIHUSHR\
                                   PPOUKGFKWX9COXBAZMQBBFRFIBGEAVKBWKNTBMLPHLOUYOXPIQIZQWGOVUWQ\
                                   ABTJT9ZZPNBABQFYRCQLXDHDEX9PULVTCQLWPTJLRSVZQEEYVBVY9KCNEZXQ\
                                   LEGADSTJBYOXEVGVTUFKNCNWMEDKDUMTKCMRPGKDCCBDHDVVSMPOPUBZOMZT\
                                   XJSQNVVGXNPPBVSBL9WWXWQNMHRMQFEQYKWNCSW9URI9FYPT9UZMAFMMGUKF\
                                   YTWPCQKVJ9DIHRJFMXRZUGI9TMTFUQHGXNBITDSORZORQIAMKY9VRYKLEHNR\
                                   NFSEFBHF9KXIQAEZEJNQOENJVMWLMHI9GNZPXYUIFAJIVCLAGKUZIKTJKGNQ\
                                   VTXJORWIQDHUPBBPPYOUPFAABBVMMYATXERQHPECDVYGWDGXFJKOMOBXKRZD\
                                   9MCQ9LGDGGGMYGUAFGMQTUHZOAPLKPNPCIKUNEMQIZOCM9COAOMZSJ9GVWZB\
                                   ZYXMCNALENZ9PRYMHENPWGKX9ULUIGJUJRKFJPBTTHCRZQKEAHT9DC9GSWQE\
                                   GDTZFHACZMLFYDVOWZADBNMEM9XXEOMHCNJMDSUAJRQTBUWKJF9RZHK9ACGU\
                                   NI9URFIHLXBXCEODONPXBSCWP9WNAEYNALKQHGULUQGAFL9LB9NBLLCACLQF\
                                   GQMXRHGBTMI9YKAJKVELRWWKJAPKMSYMJTDYMZ9PJEEYIRXRMMFLRSFSHIXU\
                                   L9NEJABLRUGHJFL9RASMSKOI9VCFRZ9GWTMODUUESIJBHWWHZYCLDENBFSJQ\
                                   PIOYC9MBGOOXSWEMLVU9L9WJXKZKVDBDMFSVHHISSSNILUMWULMVMESQUIHD\
                                   GBDXROXGH9MTNFSLWJZRAPOKKRGXAAQBFPYPAAXLSTMNSNDTTJQSDQORNJS9\
                                   BBGQ9KQJZYPAQ9JYQZJ9B9KQDAXUACZWRUNGMBOQLQZUHFNCKVQGORRZGAHE\
                                   S9PWJUKZWUJSBMNZFILBNBQQKLXITCTQDDBV9UDAOQOUPWMXTXWFWVMCXIXL\
                                   RMRWMAYYQJPCEAAOFEOGZQMEDAGYGCTKUJBS9AGEXJAFHWWDZRYEN9DN9HVC\
                                   MLFURISLYSWKXHJKXMHUWZXUQARMYPGKRKQMHVR9JEYXJRPNZINYNCGZHHUN\
                                   HBAIJHLYZIZGGIDFWVNXZQADLEDJFTIUTQWCQSX9QNGUZXGXJYUUTFSZPQKX\
                                   BA9DFRQRLTLUJENKESDGTZRGRSLTNYTITXRXRGVLWBTEWPJXZYLGHLQBAVYV\
                                   OSABIVTQYQM9FIQKCBRRUEMVVTMERLWOK"
            .chars()
            .flat_map(char_to_trits)
            .cloned()
            .collect();

        let hash: Vec<Trit> = "KXRVLFETGUTUWBCNCC9DWO99JQTEI9YXVOZHWELSYP9SG9KN9WCKXOVTEFHFH\
                                 9EFZJKFYCZKQPPBXYSGJ"
            .chars()
            .flat_map(char_to_trits)
            .cloned()
            .collect();

        let trans_t = transformer.transform(trans.as_slice());
        let hash_t = transformer.transform(hash.as_slice());

        test_hash_eq::<A, B>(trans_t.as_slice(), hash_t.as_slice());
    }

    pub fn hash_works2<A, B>(transformer: &TransformerFn<A>)
    where
        A: Copy + PartialEq + fmt::Debug,
        B: Curl<A>,
    {
        let trans: Vec<Trit> = "9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              9999999999999999999999999999999999999999999999999999999999999\
                              999999999999999999999999999999T999999999999999999999999999999\
                              99999999999999999999999OLOB99999999999999999999999"
            .chars()
            .flat_map(char_to_trits)
            .cloned()
            .collect();

        let hash: Vec<Trit> = "TAQCQAEBHLLYKAZWMNSXUPWQICMFSKWPEGQBNM9AQMGLFZGME9REOZTQIJQRKYH\
                             DANIYSMFYPVABX9999"
            .chars()
            .flat_map(char_to_trits)
            .cloned()
            .collect();

        let trans_t = transformer.transform(trans.as_slice());
        let hash_t = transformer.transform(hash.as_slice());

        test_hash_eq::<A, B>(trans_t.as_slice(), hash_t.as_slice());
    }

    pub fn test_pow<A, C>()
    where
        A: ProofOfWork<Trit>,
        C: Curl<Trit>,
    {
        let trans = "RSWWSFXPQJUBJROQBRQZWZXZJWMUBVIVMHPPTYSNW9YQIQQF9RCSJJCVZG9Z\
                                   WITXNCSBBDHEEKDRBHVTWCZ9SZOOZHVBPCQNPKTWFNZAWGCZ9QDIMKRVINMI\
                                   RZBPKRKQAIPGOHBTHTGYXTBJLSURDSPEOJ9UKJECUKCCPVIQQHDUYKVKISCE\
                                   IEGVOQWRBAYXWGSJUTEVG9RPQLPTKYCRAJ9YNCUMDVDYDQCKRJOAPXCSUDAJ\
                                   GETALJINHEVNAARIPONBWXUOQUFGNOCUSSLYWKOZMZUKLNITZIFXFWQAYVJC\
                                   VMDTRSHORGNSTKX9Z9DLWNHZSMNOYTU9AUCGYBVIITEPEKIXBCOFCMQPBGXY\
                                   JKSHPXNUKFTXIJVYRFILAVXEWTUICZCYYPCEHNTK9SLGVL9RLAMYTAEPONCB\
                                   HDXSEQZOXO9XCFUCPPMKEBR9IEJGQOPPILHFXHMIULJYXZJASQEGCQDVYFOM\
                                   9ETXAGVMSCHHQLFPATWOSMZIDL9AHMSDCE9UENACG9OVFAEIPPQYBCLXDMXX\
                                   A9UBJFQQBCYKETPNKHNOUKCSSYLWZDLKUARXNVKKKHNRBVSTVKQCZL9RY9BD\
                                   TDTPUTFUBGRMSTOTXLWUHDMSGYRDSZLIPGQXIDMNCNBOAOI9WFUCXSRLJFIV\
                                   TIPIAZUK9EDUJJ9B9YCJEZQQELLHVCWDNRH9FUXDGZRGOVXGOKORTCQQA9JX\
                                   NROLETYCNLRMBGXBL9DQKMOAZCBJGWLNJLGRSTYBKLGFVRUF9QOPZVQFGMDJ\
                                   A9TBVGFJDBAHEVOLW9GNU9NICLCQJBOAJBAHHBZJGOFUCQMBGYQLCWNKSZPP\
                                   BQMSJTJLM9GXOZHTNDLGIRCSIJAZTENQVQDHFSOQM9WVNWQQJNOPZMEISSCL\
                                   OADMRNWALBBSLSWNCTOSNHNLWZBVCFIOGFPCPRKQSRGKFXGTWUSCPZSKQNLQ\
                                   JGKDLOXSBJMEHQPDZGSENUKWAHRNONDTBLHNAKGLOMCFYRCGMDOVANPFHMQR\
                                   FCZIQHCGVORJJNYMTORDKPJPLA9LWAKAWXLIFEVLKHRKCDG9QPQCPGVKIVBE\
                                   NQJTJGZKFTNZHIMQISVBNLHAYSSVJKTIELGTETKPVRQXNAPWOBGQGFRMMK9U\
                                   QDWJHSQMYQQTCBMVQKUVGJEAGTEQDN9TCRRAZHDPSPIYVNKPGJSJZASZQBM9\
                                   WXEDWGAOQPPZFLAMZLEZGXPYSOJRWL9ZH9NOJTUKXNTCRRDO9GKULXBAVDRI\
                                   ZBOKJYVJUSHIX9F9O9ACYCAHUKBIEPVZWVJAJGSDQNZNWLIWVSKFJUMOYDMV\
                                   UFLUXT9CEQEVRFBJVPCTJQCORM9JHLYFSMUVMFDXZFNCUFZZIKREIUIHUSHR\
                                   PPOUKGFKWX9COXBAZMQBBFRFIBGEAVKBWKNTBMLPHLOUYOXPIQIZQWGOVUWQ\
                                   ABTJT9ZZPNBABQFYRCQLXDHDEX9PULVTCQLWPTJLRSVZQEEYVBVY9KCNEZXQ\
                                   LEGADSTJBYOXEVGVTUFKNCNWMEDKDUMTKCMRPGKDCCBDHDVVSMPOPUBZOMZT\
                                   XJSQNVVGXNPPBVSBL9WWXWQNMHRMQFEQYKWNCSW9URI9FYPT9UZMAFMMGUKF\
                                   YTWPCQKVJ9DIHRJFMXRZUGI9TMTFUQHGXNBITDSORZORQIAMKY9VRYKLEHNR\
                                   NFSEFBHF9KXIQAEZEJNQOENJVMWLMHI9GNZPXYUIFAJIVCLAGKUZIKTJKGNQ\
                                   VTXJORWIQDHUPBBPPYOUPFAABBVMMYATXERQHPECDVYGWDGXFJKOMOBXKRZD\
                                   9MCQ9LGDGGGMYGUAFGMQTUHZOAPLKPNPCIKUNEMQIZOCM9COAOMZSJ9GVWZB\
                                   ZYXMCNALENZ9PRYMHENPWGKX9ULUIGJUJRKFJPBTTHCRZQKEAHT9DC9GSWQE\
                                   GDTZFHACZMLFYDVOWZADBNMEM9XXEOMHCNJMDSUAJRQTBUWKJF9RZHK9ACGU\
                                   NI9URFIHLXBXCEODONPXBSCWP9WNAEYNALKQHGULUQGAFL9LB9NBLLCACLQF\
                                   GQMXRHGBTMI9YKAJKVELRWWKJAPKMSYMJTDYMZ9PJEEYIRXRMMFLRSFSHIXU\
                                   L9NEJABLRUGHJFL9RASMSKOI9VCFRZ9GWTMODUUESIJBHWWHZYCLDENBFSJQ\
                                   PIOYC9MBGOOXSWEMLVU9L9WJXKZKVDBDMFSVHHISSSNILUMWULMVMESQUIHD\
                                   GBDXROXGH9MTNFSLWJZRAPOKKRGXAAQBFPYPAAXLSTMNSNDTTJQSDQORNJS9\
                                   BBGQ9KQJZYPAQ9JYQZJ9B9KQDAXUACZWRUNGMBOQLQZUHFNCKVQGORRZGAHE\
                                   S9PWJUKZWUJSBMNZFILBNBQQKLXITCTQDDBV9UDAOQOUPWMXTXWFWVMCXIXL\
                                   RMRWMAYYQJPCEAAOFEOGZQMEDAGYGCTKUJBS9AGEXJAFHWWDZRYEN9DN9HVC\
                                   MLFURISLYSWKXHJKXMHUWZXUQARMYPGKRKQMHVR9JEYXJRPNZINYNCGZHHUN\
                                   HBAIJHLYZIZGGIDFWVNXZQADLEDJFTIUTQWCQSX9QNGUZXGXJYUUTFSZPQKX\
                                   BA9DFRQRLTLUJENKESDGTZRGRSLTNYTITXRXRGVLWBTEWPJXZYLGHLQBAVYV\
                                   OSABIVTQYQM9FIQKCBRRUEMVVTMERLWOK";

        let min_weight = 11u8;
        let trits: Vec<Trit> = trans.chars().flat_map(char_to_trits).cloned().collect();
        let mut nonce: Vec<Trit> = vec![0; HASH_LENGTH];
        assert!(
            A::search(&trits, min_weight, nonce.as_mut_slice()),
            "Some PoW Failure."
        );

        let final_t: Vec<Trit> = trits[..(trits.len() - HASH_LENGTH)]
            .into_iter()
            .cloned()
            .chain(nonce)
            .collect();

        let mut curl = C::default();
        curl.absorb(&final_t);

        let mut hash = vec![0; HASH_LENGTH];
        curl.squeeze(hash.as_mut_slice());

        let weight: usize = hash[(HASH_LENGTH - min_weight as usize)..]
            .into_iter()
            .rev()
            .take_while(|&&t| t == 0)
            .count();
        assert_eq!(weight, min_weight as usize);
    }
    pub fn test_ham<A, C>()
    where
        A: HammingNonce<Trit>,
        C: Curl<Trit>,
    {
        let trytes = "RSWWSFXPQJUBJROQBRQZWZXZJWMUBVIVMHPPTYSNW9YQIQQF9RCSJJCVZG9Z\
                                   GBDXROXGH9MTNFSLWJZRAPOKKRGXAAQBFPYPAAXLSTMNSNDTTJQSDQORNJS9\
                                   BBGQ9KQJZYPAQ9JYQZJ9B9KQDAXUACZWRUNGMBOQLQZUHFNCKVQGORRZGAHE\
                                   S9PWJUKZWUJSBMNZFILBNBQQKLXITCTQDDBV9UDAOQOUPWMXTXWFWVMCXIXL\
                                   RMRWMAYYQJPCEAAOFEOGZQMEDAGYGCTKUJBS9AGEXJAFHWWDZRYEN9DN9HVC\
                                   MLFURISLYSWKXHJKXMHUWZXUQARMYPGKRKQMHVR9JEYXJRPNZINYNCGZHHUN\
                                   HBAIJHLYZIZGGIDFWVNXZQADLEDJFTIUTQWCQSX9QNGUZXGXJYUUTFSZPQKX\
                                   BA9DFRQRLTLUJENKESDGTZRGRSLTNYTITXRXRGVLWBTEWPJXZYLGHLQBAVYV\
                                   OSABIVTQYQM9FIQKCBRRUEMVVTMERLWOK";

        let trits: Vec<Trit> = trytes.chars().flat_map(char_to_trits).cloned().collect();
        let mut nonce: Vec<Trit> = vec![0; core::cmp::min(trits.len(), HASH_LENGTH)];
        for security in 1u8..4u8 {
            assert!(
                A::search(&trits, security, nonce.as_mut_slice()),
                "Some Search Failure."
            );

            let len_trits = {
                let l = (trits.len() / TRITS_PER_TRYTE) as isize;
                let mut out = vec![0; num::min_trits(l) as usize];
                num::int2trits(l, out.as_mut_slice());
                out
            };

            let mut curl = C::default();
            curl.absorb(len_trits.as_slice());
            curl.absorb(trits.as_slice());
            curl.absorb(nonce.as_slice());
            let mut hash = vec![0 as Trit; security as usize * HASH_LENGTH / 3];
            curl.squeeze(hash.as_mut_slice());

            let hash_security = {
                let mut sum = 0;
                for i in hash[..(HASH_LENGTH / 3)].iter() {
                    sum += *i;
                }
                if sum == 0 {
                    1
                } else {
                    sum = 0;
                    for i in hash[..(2 * HASH_LENGTH / 3)].iter() {
                        sum += *i;
                    }
                    if sum == 0 {
                        2
                    } else {
                        sum = 0;
                        for i in hash {
                            sum += i;
                        }
                        if sum == 0 { 3 } else { 0 }
                    }
                }
            };
            assert_eq!(hash_security, security);
        }
    }
}

pub fn run<A, B>(transformer: &TransformerFn<A>)
where
    A: Copy + PartialEq + fmt::Debug,
    B: Curl<A>,
{
    // run tests
    inner::hash_works1::<A, B>(transformer);
    inner::hash_works2::<A, B>(transformer);
}

pub fn run_search<A, C>()
where
    A: ProofOfWork<Trit>,
    C: Curl<Trit>,
{
    inner::test_pow::<A, C>();
}

pub fn run_ham_search<A, C>()
where
    A: HammingNonce<Trit>,
    C: Curl<Trit>,
{
    inner::test_ham::<A, C>();
}
