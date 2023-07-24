use std::fs;
use std::os::unix::raw::off_t;
use lazy_static::lazy_static;
use regex::Regex;
use tl::NodeHandle;
use tl::queryselector::iterable::QueryIterable;
use crate::info::Info;

pub struct Loader {
    info: Info,
}

const PAGE_START: &'static str = "https://cabinet.spbu.ru/Lists/1k_EntryLists/";

const PAGES: [&'static str; 93] = [
    "list_fbc96390-0914-4d9a-931e-8442ac9942d4.html",
    "list_821726da-0cf7-4345-9048-9e590c9dce5f.html",
    "list_03cbcb2b-7ebf-42af-afa9-4fdf873824a6.html",
    "list_6da93f46-88d6-45c6-babb-cd9a18eb7a95.html",
    "list_635e3729-73a5-4c04-9983-4150cd3016ff.html",
    "list_4dc851eb-ef73-4452-96f1-4e7bfcf0cbfb.html",
    "list_ac3dcbcb-81b7-4cb0-b3b2-b0a638e3bab0.html",
    "list_269b5ea2-afca-4b74-8625-a1c988a3c442.html",
    "list_1facd280-a059-4cd0-84c8-33f487fc955d.html",
    "list_ecf23a61-4cc1-4c5e-a2a5-b300a858943f.html",
    "list_544345e5-26d3-4c82-be1d-c6ef66dd7fd1.html",
    "list_d1065aef-a6ea-42f9-921c-09ed1993bc4e.html",
    "list_d32b6ab7-ddd4-4346-ac5c-e184ab483eec.html",
    "list_b8ba9964-c931-4410-a187-36ba2cca71e6.html",
    "list_5fdedaec-7571-4918-a80d-48ce45faa594.html",
    "list_322c4089-954c-4abc-9193-8a2767a49da9.html",
    "list_2c0b0232-0c82-43ae-93f6-aa411b535a37.html",
    "list_f7f6cbb8-6ae8-4dba-9f3c-b21b25758df6.html",
    "list_3d11f957-e2de-46de-a852-174585156d19.html",
    "list_b6f9356b-ccfc-4309-95fc-7317c0c6f21b.html",
    "list_5af5d009-6da9-4daa-a5fd-84cc9a9e564b.html",
    "list_2904a6dc-e73b-46ee-9816-36dfadb7bf4b.html",
    "list_71491a14-f92c-4c36-a054-61b1529fbedc.html",
    "list_31825967-9a77-4be1-a087-9c4ad3ef269f.html",
    "list_5d31a301-7fe0-414e-a320-e3776c9aa8b7.html",
    "list_67a7aecb-635d-4e6f-aee9-8f9bf8aa2276.html",
    "list_70209201-d276-48f8-ad13-6c339fdffcfc.html",
    "list_50cd9cf8-51d5-44f8-98ef-c97f3eb64c7f.html",
    "list_0cfcac1a-49e3-49be-9d87-86cad5168031.html",
    "list_a46ee3bc-64fc-49ba-af3b-1ff84430b83c.html",
    "list_c120cafe-d3b8-46e8-a9ec-8fe02ec204f3.html",
    "list_a82d6f96-248e-45d7-96aa-360fff0eecbb.html",
    "list_2262f7fc-5974-45be-9d9e-48b12064e7a6.html",
    "list_aa6e0445-fa6c-406a-8258-6be520f3630f.html",
    "list_48c5c142-75f9-49cb-94d3-a197f335697f.html",
    "list_14d67627-f164-4551-a8be-0cd64ff739a0.html",
    "list_7c5ccde8-19c2-4958-bfed-c238997546de.html",
    "list_36257470-5d77-495c-ab4d-7c35c70f7ad9.html",
    "list_9804b86c-2a83-43da-87e5-f4966b81f31b.html",
    "list_66d4dfc6-2253-4c12-8a7f-f0a4220dac37.html",
    "list_66ab81aa-c70e-44c6-98c3-f89d409b6c23.html",
    "list_99e14c32-f830-49c0-8deb-cbabe92a3ae2.html",
    "list_f7125cc3-90b0-41b6-8397-079aa9c2e3d0.html",
    "list_be9c6382-bd39-4dee-8698-db8729301a20.html",
    "list_c452a608-4e04-41c3-8baa-7a23739bd0ad.html",
    "list_0dece47a-a0e6-499c-8919-0872d923a319.html",
    "list_545bb745-ccac-4adb-a99b-856442acada1.html",
    "list_fb5705e7-2f07-4395-818a-3c0afa17b8e4.html",
    "list_797a503d-d7a9-4302-85ab-f61c1456967c.html",
    "list_821902f8-cdd1-485d-bc32-14f1e0d87475.html",
    "list_b358ff88-353e-4397-8ade-fdcacbb20196.html",
    "list_4849e1c9-2fc9-44b3-a482-0a3ebbe3d7b8.html",
    "list_6e07c26b-c512-4b0d-9f4e-6e2485090ded.html",
    "list_3bc67247-de2f-4c3c-824f-ad22ca08917e.html",
    "list_b4010d26-a2a8-4772-beae-da49bfb905d4.html",
    "list_d077d694-5f27-47cf-b681-806510160c31.html",
    "list_2e1804e3-9c83-4a31-a75a-1c245a97dc1d.html",
    "list_19f6f6ab-79af-4425-89de-92116bbfe701.html",
    "list_cda10052-4522-43e6-bbd8-034292960483.html",
    "list_873f5635-fff8-4bc9-b0b0-f7ab5c48b626.html",
    "list_e2307bcf-ea93-4752-9ee5-4d82fd4f4cd3.html",
    "list_a0527b1b-fc22-4b30-8c3c-7f77e9cf0aec.html",
    "list_fabf7f19-2c9e-4b04-a455-f5c30b5474fb.html",
    "list_c44dd141-14cb-4c50-8d3e-e6820756788a.html",
    "list_e974dda1-dd89-4f1f-8cf6-2e3857d9f474.html",
    "list_e139f69e-648e-4a9d-b397-b4ea3d20a083.html",
    "list_089baeb7-b556-473a-8a2b-cadb7d0cdf1c.html",
    "list_5d519931-d9bb-4ab5-9d14-58e8ae45520b.html",
    "list_5f86ff95-574e-4645-8566-d2d9778de03c.html",
    "list_dc3f2329-25bd-4d93-83d9-b80454340eb8.html",
    "list_47190685-f292-454e-82e6-a970768fc7f3.html",
    "list_18eaaca6-48d2-4942-9888-3182a704d3eb.html",
    "list_cea3f4af-fd93-4713-ae41-d58cb90d210f.html",
    "list_09acfe40-e30f-4416-8ddc-43455c54af85.html",
    "list_ecee4d77-8ac9-434d-b062-9e7b13d491b0.html",
    "list_5b02d3cf-201f-4653-9025-e0a131827cf8.html",
    "list_d7242918-a700-4d65-8d9f-0d929d8d887f.html",
    "list_ebff3b22-7983-4060-9d69-ac83b380b2c9.html",
    "list_38d0d0af-7566-44eb-94d1-6cbbdf0b241b.html",
    "list_bd9bb295-b936-496d-8e72-6f8f5fcb732c.html",
    "list_6f4e7237-5e4a-43e4-8892-ea7a2ef23fed.html",
    "list_3a483215-72a5-4dce-ade5-27f95e06c2d0.html",
    "list_c8cf115a-d9b9-4b59-ad11-d7d053115081.html",
    "list_29a692c8-8a2c-4fc4-9bfe-d6e6e9accaa9.html",
    "list_d6d6e64c-4412-4fed-827e-6f9a1fa55064.html",
    "list_110554da-9af0-471a-91ff-d247efc4e1f8.html",
    "list_ec2041fb-d3a3-4361-a1be-66eee13c9ba5.html",
    "list_e760917a-3afe-48a8-a1f2-a07940fbdb3c.html",
    "list_11270386-b1f5-4b7d-b241-72906ee1414f.html",
    "list_4eb04d8e-6b52-4d87-b28b-4b49e081fc55.html",
    "list_f67a0c98-1fae-4139-83b5-e697afa39a8c.html",
    "list_fde7fd82-e89c-4e5b-a79c-5b1500a65085.html",
    "list_593c599a-8b72-429f-8e59-e85fbf012b3c.html"
];

const SITE_ID_PATTERN: &'static str = "<b>Образовательная программа:<\\/b>(.*)<br><\\/br>";
const SITE_CAPACITY_PATTERN: &'static str = "<b>КЦП по конкурсу:<\\/b>(.*)<br><\\/br>";

impl Loader {
    pub fn new() -> Self {
        Self {
            info: Info::new(),
        }
    }

    pub fn run(&mut self) {
        for i in 0..PAGES.len() {
            let page = PAGES[i];
            self.load_page((PAGE_START.to_owned() + page).as_str());

            let percent = ((i + 1) as f64 / PAGES.len() as f64 * 100.).round() as i32;

            println!("Loading the data... {}%", percent);
        }
    }

    pub fn load_page(&mut self, addr: &str) {
        let content = reqwest::blocking::get(
            addr
        )
            .unwrap()
            .text()
            .unwrap();

        let dom = tl::parse(content.as_str(), tl::ParserOptions::default()).unwrap();
        let parser = dom.parser();

        let about = dom.query_selector("p").unwrap().next().unwrap().get(parser).unwrap();
        let about = about.inner_html(parser);

        lazy_static! {
            static ref SITE_ID_RE: Regex = Regex::new(SITE_ID_PATTERN).unwrap();
            static ref SITE_CAPACITY_RE: Regex = Regex::new(SITE_CAPACITY_PATTERN).unwrap();
        }

        let site_id = SITE_ID_RE.captures(about.as_ref()).unwrap()[1].trim().to_string();

        let site_capacity: usize = SITE_CAPACITY_RE.captures(about.as_ref()).unwrap()[1].trim().parse().unwrap();

        self.info.set_site_capacity(&site_id, site_capacity);

        let entries = dom.query_selector("tr.1").unwrap();

        for entry in entries {
            let content = entry.get(parser).unwrap().inner_html(parser);

            let dom = tl::parse(content.as_ref(), tl::ParserOptions::default()).unwrap();
            let parser = dom.parser();

            let children: Vec<NodeHandle> = dom.query_selector("td").unwrap().collect();

            let len = children.len();

            let place = children[0].get(parser).unwrap().inner_text(parser);
            let id = children[1].get(parser).unwrap().inner_text(parser);
            let comp_ty = children[2].get(parser).unwrap().inner_text(parser);
            let priority = children[3].get(parser).unwrap().inner_text(parser);
            let sum_total = children[4].get(parser).unwrap().inner_text(parser);
            let sum_scores = children[5].get(parser).unwrap().inner_text(parser);

            let mut scores = vec![];

            let mut i = 6;

            while i < (len - 3) {
                scores.push(children[i].get(parser).unwrap().inner_text(parser).as_ref().to_string());

                i += 1;
            }

            let sum_ind = children[i].get(parser).unwrap().inner_text(parser);
            let ind_descr = children[i + 1].get(parser).unwrap().inner_text(parser);
            let optional = children[i + 2].get(parser).unwrap().inner_text(parser);

            let id = id.trim().to_string();
            let priority: usize = priority.trim().parse().unwrap();
            let place: usize = place.trim().parse().unwrap();
            let sum_total: usize = if sum_total.trim().is_empty() {
                999
            } else {
                sum_total.trim().split(",").next().unwrap().parse().unwrap()
            };

            let abit = self.info.get_abit_mut(&id);

            abit.add_priority(&site_id, place, priority);
            abit.site_scores.insert(site_id.clone(), sum_total);
        }
    }

    pub fn release(self) -> Info {
        self.info
    }
}