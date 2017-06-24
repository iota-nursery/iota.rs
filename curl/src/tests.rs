#![allow(unused_imports)]

use curl::*;
use trytes::*;
use std::iter::FromIterator;

pub mod testsuite {
    use super::*;

    #[cfg(test)]
    mod inner {
        use super::*;

        fn test_hash_eq<T>(trans: Trinary, expected: Trinary)
        where
            T: Copy + Clone + Sized,
            Curl<T>: Sponge + Default,
            Trinary: IntoTrits<T> + FromIterator<T>,
        {

            let mut curl = Curl::<T>::default();
            let trits = trans.trits();
            curl.absorb(trits.as_slice());
            let hash: Trinary = curl.squeeze(HASH_LENGTH).into_iter().collect();

            assert_eq!(hash, expected);
        }

        pub fn hash_works2<T>()
        where
            T: Copy + Clone + Sized,
            Curl<T>: Sponge + Default,
            Trinary: IntoTrits<T> + FromIterator<T>,
        {

            let trans: Trinary = "RSWWSFXPQJUBJROQBRQZWZXZJWMUBVIVMHPPTYSNW9YQIQQF9RCSJJCVZG9Z\
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
                .collect();

            let hash: Trinary = "KXRVLFETGUTUWBCNCC9DWO99JQTEI9YXVOZHWELSYP9SG9KN9WCKXOVTEFHFH\
                                 9EFZJKFYCZKQPPBXYSGJ"
                .chars()
                .collect();

            test_hash_eq::<T>(trans, hash);

        }

        pub fn hash_works1<T>()
        where
            T: Copy + Clone + Sized,
            Curl<T>: Sponge + Default,
            Trinary: IntoTrits<T> + FromIterator<T>,
        {
            let trans: Trinary = "9999999999999999999999999999999999999999999999999999999999999\
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
                .collect();

            let ex_hash: Trinary = "TAQCQAEBHLLYKAZWMNSXUPWQICMFSKWPEGQBNM9AQMGLFZGME9REOZTQIJQRKYH\
                             DANIYSMFYPVABX9999"
                .chars()
                .collect();

            test_hash_eq::<T>(trans, ex_hash);
        }
    }

    #[cfg(test)]
    pub fn run<T>()
    where
        T: Copy + Clone + Sized,
        Curl<T>: Sponge + Default,
        Trinary: IntoTrits<T> + FromIterator<T>,
    {
        // run tests
        inner::hash_works1::<T>();
        inner::hash_works2::<T>();
    }

}