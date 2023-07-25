use std::cmp::{min};


use lazy_static::lazy_static;
use regex::Regex;
use tl::NodeHandle;
use indicatif::ProgressBar;
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


const TARGET_QUOTA_PAGES: [&'static str; 103] = [
    "list_913c75c9-f7e6-40e7-bd8d-3434366e39a2.html",
    "list_eb130b43-f0ce-4ae4-a209-b569152e5be2.html",
    "list_4831f2ad-2a0f-4809-81a5-7e3d5ebd7fa1.html",
    "list_55d43c45-1b2f-43d7-a258-15b3bfe9b671.html",
    "list_55726302-edc9-47f2-882e-8773b4ac406c.html",
    "list_e05eab8e-cec9-4512-8f17-d343ed220b6b.html",
    "list_679f8865-3b0d-4452-ba88-d8bd38f6f139.html",
    "list_8d404bed-0181-4a41-babe-9b1571a88be9.html",
    "list_73c9c6f1-d836-4f5b-848e-a4c8936d4526.html",
    "list_b7cb357c-db1b-4cf5-b1fe-c87e1a4cd850.html",
    "list_a867d3a5-74ed-48ab-a9e6-cc49a86ba5a2.html",
    "list_68a236fd-527c-48da-a605-568ca0bf546b.html",
    "list_27421aaf-027a-46e1-8cac-cbda76704f03.html",
    "list_95c9ce64-b382-4e8b-9f32-0f5c290bd567.html",
    "list_242e99c5-d348-43d9-ac90-8b9cfc58673b.html",
    "list_79655878-e62e-439c-928f-db9037f12b41.html",
    "list_d8110cca-09fb-45c0-840e-0e03df87c5b4.html",
    "list_b9d6ed20-a7cf-4443-a615-d564a02c16a7.html",
    "list_e722c64e-02d2-4f5a-aeac-1fd81cd1b21d.html",
    "list_af757d85-dbda-4de3-8923-5615116f9a09.html",
    "list_326216cc-864f-42e0-8d80-6fc8121b27b5.html",
    "list_707735f8-a1ff-4036-985d-4058fc405c84.html",
    "list_ca510a70-2923-448c-8a72-2e539160d699.html",
    "list_7c633994-6e48-49b0-986b-ff310a6d5018.html",
    "list_0a45e4a1-af8a-4cd2-b174-cb7468c2902d.html",
    "list_732e4854-b153-4217-8a91-4fe5da950329.html",
    "list_52fb8a4b-32f4-481b-a97e-c7c4d50877f4.html",
    "list_34e77db3-60c5-425a-a46b-ac0639cb711f.html",
    "list_e9eb0bb1-9a4e-4263-ae7f-7d57bd0b25e1.html",
    "list_bb2a7e93-ed1c-451b-9f5d-e2dcc4784f95.html",
    "list_b0d43715-6eba-4815-b937-5a1210cfdc0b.html",
    "list_caa162de-5008-48a9-8b88-0deccc5cf398.html",
    "list_a109ec11-d0dc-405e-bd60-e6e42eb2bbbe.html",
    "list_b6d380e1-ce38-47d7-ae56-c568df802634.html",
    "list_e9cfc150-0911-4c10-aa7e-38cfaefc291c.html",
    "list_31a87dc1-2a18-478f-9d5d-3c3353603cc8.html",
    "list_cbeb259d-1384-40d5-94cd-ee76f0de90e7.html",
    "list_1c8f0ddb-862a-4030-b72f-4524e8757358.html",
    "list_d6887cac-ca87-44bd-b56b-c2d880949670.html",
    "list_ef864236-5af7-4a26-97f3-a128795e87b8.html",
    "list_37f53cbc-5360-4a30-9027-4daf821545ae.html",
    "list_d4493601-188e-4f15-b33a-1416728ec946.html",
    "list_bb799880-09e1-43ef-89f0-cbd5893f177a.html",
    "list_676e9bff-7df2-4ce7-9714-bf0aacbe4363.html",
    "list_52d7bf0d-edd8-429f-9655-5f90a020cb5b.html",
    "list_89c1a7a2-e0bf-4dd5-8e19-635b49aceac6.html",
    "list_7c46b93f-bab2-40f5-9cff-55cf3aedfae2.html",
    "list_68ce215f-be7f-4c1c-969e-3c99b399067b.html",
    "list_94660546-9d91-4655-84e6-14f8d5570005.html",
    "list_be904f57-470e-4202-ba99-02c55a359658.html",
    "list_c26ad089-f1ab-4775-999f-354cacb9daf1.html",
    "list_02d62299-dd9f-41d0-8f8f-efd7b5d5c90f.html",
    "list_f12f07dc-596e-4849-8707-868b4597bb22.html",
    "list_5bf1db9e-1c2c-4553-bc99-35726f99435b.html",
    "list_025fedb8-1a5d-49c0-abb6-bd338e1ce817.html",
    "list_e335fbb6-1c94-4695-bc3c-5f565fcf455e.html",
    "list_d87b7d6f-1217-4279-b995-60859b9a66bb.html",
    "list_9a5d0e13-8851-42ea-9577-2002e8d75356.html",
    "list_02d20453-7293-4aac-9faa-b7b7ae422cbe.html",
    "list_62966845-7424-4c3d-a0cf-b5000c33527f.html",
    "list_5345a16e-d822-4af1-b736-aa7f523c5979.html",
    "list_b9582ac8-a5c7-40c8-8d70-a9959a4ce500.html",
    "list_cf40a373-831d-41bf-8c4d-f0c528f43cd6.html",
    "list_1fcfec37-98c0-490f-9156-ee79be7959ac.html",
    "list_caed11b2-9ac9-4907-b57e-f58fa1b8d843.html",
    "list_c05634d2-f8f2-4660-aadd-253fc945e9f3.html",
    "list_c26bae75-c056-4342-a1e1-485839118cac.html",
    "list_a06ab435-034d-4303-8390-76ce30e768fc.html",
    "list_cde7d337-f260-4a35-bf03-43e9b8316592.html",
    "list_9ddd1eb8-5d84-4a6d-9b06-3f0590528cdf.html",
    "list_88dbc08f-3a6b-4366-bedd-28ae0839eff0.html",
    "list_b277faef-1bd6-4d18-b2a0-58327715a281.html",
    "list_ef765a1f-d9be-4ee9-ae25-1af5c3cb66ea.html",
    "list_82afed2a-95c3-420d-9caa-ecc72a6f5c97.html",
    "list_2cb823b5-3322-441b-8934-88cbc5b00304.html",
    "list_b644f9a2-668b-47dc-90a2-aa8738d61682.html",
    "list_a7bc37b5-d2c2-4b89-9c6d-41efc3d74cfb.html",
    "list_6910ba99-132a-4902-8ee7-6ba7601c6b57.html",
    "list_1085ab85-b0d9-4f1f-b98e-3798e09b0477.html",
    "list_b1d2af52-48c3-49fa-bdd1-eb072c98671d.html",
    "list_18a6a7c0-87ea-44bd-b3d0-715b26635f6f.html",
    "list_68c530c6-3120-4948-9f92-dd5cfcb71f59.html",
    "list_5b20fdd5-728f-4698-b979-23b8b12aefa1.html",
    "list_e24c7c41-0ba1-460d-a9a1-95ae92649936.html",
    "list_f05d3446-be19-4f32-97e9-76f6422aebfb.html",
    "list_ea0be61c-aab2-4190-b292-fb5f66871ffc.html",
    "list_0b3fb752-c0c2-4256-9eea-0c5275d35c27.html",
    "list_e5c0372a-c428-455e-b3cf-b2d5b05a25b6.html",
    "list_1f7c5aee-36da-429b-a90c-ea77debe31ea.html",
    "list_006bce93-20dc-4500-8db2-2f532212fb66.html",
    "list_60a0cc00-6f5f-44c2-95da-ec461086f6c4.html",
    "list_6c846e47-0211-443c-a3b4-eee2a74f4781.html",
    "list_9b18fd1b-c4ab-4def-9283-31c5e071c477.html",
    "list_dbe7e02c-bff8-4db3-ae6d-5b66f85bbcc4.html",
    "list_5767f7de-df36-4d3e-b64d-d20e98cf55f0.html",
    "list_5f9a7ec0-d0fe-453a-8fa8-ab69fa8bf005.html",
    "list_cc4a6940-9680-4400-b904-7837b2712f77.html",
    "list_1bd2aa79-7f26-49ae-a2bb-c1ea90cfb8c0.html",
    "list_ac66ba05-fb6d-423f-8340-3a7623286435.html",
    "list_df62321a-7c2c-4f17-bdc3-447ea56b4aa7.html",
    "list_9e918552-ddf6-4038-a5ae-3afbde7fc21a.html",
    "list_76e1feaf-3c42-4071-93cc-3b0b088d88dd.html",
    "list_1c4cffc1-63e7-41a7-8a18-145addefc04c.html"
];

const SPECIAL_QUOTA_PAGES: [&'static str; 92] = [
    "list_29126d46-7943-409d-9fdc-40ea3dd9b752.html",
    "list_20336014-d175-4f78-bbcd-1710f0e2a6b4.html",
    "list_52c38eed-e3c7-4aa2-8ae7-19d852ce7285.html",
    "list_b5fd69c5-bd8b-4b3d-ba16-a2c445ffe1ea.html",
    "list_cea89950-bc52-4e3e-a6ee-419d652f9bc5.html",
    "list_ca2045a1-7d9a-4cd6-9358-66d389758227.html",
    "list_3539fba1-e50e-45db-a790-8045d6a9176d.html",
    "list_e1f3533e-6122-4ecd-906c-4e46457e7f87.html",
    "list_c9fe5de5-baef-48ce-9e63-67488e807996.html",
    "list_60d073b8-102b-458b-8660-646de2bb8f81.html",
    "list_70f839ce-d998-497a-ae89-47610bebbbe4.html",
    "list_f1a99e15-c415-4468-8032-107c89aacba3.html",
    "list_e9ea723a-1d8d-47b5-aba4-07265de0d87a.html",
    "list_2a17dcc2-875b-4c5b-a75c-609d1e1a191e.html",
    "list_1b886b44-3d9f-4ecf-a0ed-ca36fe6c7330.html",
    "list_7ee8dffb-e0ec-4ec0-a590-41de6e4ccb73.html",
    "list_5105847f-c751-4687-a316-ae09f1535f65.html",
    "list_b8284d28-3d9d-4f01-a7d2-876fc0546e8a.html",
    "list_304c05b9-dada-4f99-8cc9-97e771030798.html",
    "list_af7ab49f-5f2f-4fdc-b159-fdc2ebc504df.html",
    "list_7328ca51-602d-4aeb-9617-bc2b09b90361.html",
    "list_13538a34-1a88-4006-9bb2-4c9f73d2d113.html",
    "list_1abbcd3b-e87c-44cd-9b3f-9755bba2b3a4.html",
    "list_f28dfa72-fe9c-4571-8e8d-253ab5483777.html",
    "list_7906cffa-0cb0-48c5-b3d3-be5412e15fa9.html",
    "list_463c5e57-ca8e-4ada-ae9f-b80cf3a4f7d9.html",
    "list_03d8e38d-0dbe-4cf9-88d3-60c1de7266de.html",
    "list_c4e32ac8-be14-420c-9df8-3265a4ae97fb.html",
    "list_d97d66ab-b660-4de8-b05d-6f28a2adcb30.html",
    "list_6720d39d-497d-4d4c-b758-5f95bb68eb4a.html",
    "list_25821706-5f5f-4332-819c-ef70e156dc22.html",
    "list_305a9526-727b-4312-a618-2ac3b2300b10.html",
    "list_dab77206-1f8c-48f6-af95-64b166fe79b9.html",
    "list_9508574c-f744-4829-ab1a-03176cc54bb8.html",
    "list_301ac089-bf5d-4076-b600-db432fd03260.html",
    "list_b606a94c-b23b-4e93-8d3e-5e46f3826dcc.html",
    "list_de8973f4-f9e5-4413-b0a0-0ce6625a4dc1.html",
    "list_b448e7c3-3133-42a6-8d12-d58394d229ef.html",
    "list_0e9e1224-a8ef-4491-902d-79e95c31febe.html",
    "list_25e33822-ccde-45d7-9ed4-bf366e87ea39.html",
    "list_d47ae081-23cb-4954-92ea-928e45689062.html",
    "list_72974097-e5dd-4351-90f9-e19a3b2d1dbc.html",
    "list_3ea4266d-1695-40d7-ae22-f11c7b528f0b.html",
    "list_12462a9a-3d38-4a3e-817b-b07d0b3e33d3.html",
    "list_a304cc91-268c-4ce5-80e9-1c20206a1810.html",
    "list_f590d6c5-43db-4563-8268-0b2c91a962cb.html",
    "list_837ea4ef-56c7-4185-a755-e4f4e4b782c5.html",
    "list_caa78096-98c5-4bd6-80bc-707bd89b96b3.html",
    "list_735efbc3-ad8a-48d5-9ffb-26daaf714ba0.html",
    "list_5b5f7cb0-4277-4740-a4fb-49dc0a97fce6.html",
    "list_9ba4dd9e-ea06-404c-9173-e9c688cfe491.html",
    "list_85381e6a-f7de-4749-9363-68a80daeb8b3.html",
    "list_35811c7f-4c68-49dc-8b8c-413e304b77d1.html",
    "list_014f1a4c-4457-49b9-bbcb-1062e5177aa3.html",
    "list_68cc8642-3b7c-4402-86af-280cf9f91469.html",
    "list_e0451182-8d1e-4811-bc19-dcda33e2d9a6.html",
    "list_bb6dc366-108c-4093-b32d-7766b5377478.html",
    "list_dcdf3596-3625-4292-873a-f24d0ff5e9d1.html",
    "list_4ec45c76-bd10-4e58-aa01-1b4b039dc41e.html",
    "list_1b32af79-16f7-45a5-b416-c153c7f30d46.html",
    "list_6f5cba2b-032e-4aea-84f4-4c1733f7615d.html",
    "list_564c4803-0d4a-4eb3-bba7-da347a0d3778.html",
    "list_4e65fb74-037e-4e41-99c8-27a6317303ba.html",
    "list_748a7586-abec-4766-bfef-a26068a20c17.html",
    "list_83b80a6e-3c5c-456b-9689-0509aaff367a.html",
    "list_332e344d-f3e7-4b28-a1d7-d0c774ad1983.html",
    "list_88964f99-b77c-4453-aaa1-79cb5c0ca743.html",
    "list_46fd5ea9-58b6-469a-962f-6a391e7562e4.html",
    "list_b2e91d8f-af22-4657-b4a9-b018642ba6b6.html",
    "list_95a844de-a2f9-4e1f-9b8b-b506260239a1.html",
    "list_7ccea9d3-fa31-4062-8a7f-73d48d0d6f7b.html",
    "list_08753f85-c3f3-4d29-8023-9b425b32a313.html",
    "list_03080d2d-131e-494d-bf98-4a625851522c.html",
    "list_787e40fb-362c-4ae8-8159-52727ed12f09.html",
    "list_ea123acb-55f7-476d-a124-cefc7fd015ad.html",
    "list_b3974a8d-b2ea-4bb2-a31a-9714fc796f68.html",
    "list_845ec7ef-7518-4295-9e66-fac88375a04e.html",
    "list_e13401fe-f6e0-4325-9f30-cde3270c503b.html",
    "list_dd76e596-00fd-4b22-b69e-acd6b9d566bf.html",
    "list_004ec4e4-b600-44a1-853f-a8bd37e3a455.html",
    "list_42d1cb12-bf43-4b33-bc77-215f0d06f253.html",
    "list_1e05f3f6-5c3e-4e60-be50-e04197c025bc.html",
    "list_624d3c9c-0bf3-4f03-8869-10f0d2cf1846.html",
    "list_d2cd1452-30db-45db-ac9f-8c13d233585c.html",
    "list_0a186c30-8a87-4539-b352-64aa85b0022d.html",
    "list_e11c3093-c5b5-4601-869a-2b5390edfe13.html",
    "list_edc79ace-935a-4a08-81aa-d0adc418ae11.html",
    "list_8109b57f-8bab-429e-8e92-a05fe387e8dc.html",
    "list_0ba12d17-dd9b-4100-97f9-0acd004c8557.html",
    "list_8f4f366f-e2ed-45c9-8fd5-4b883ebbe3f4.html",
    "list_2e5c18b8-5977-41ab-94d7-fe8ad7355129.html",
    "list_33b7db0e-9312-4b8b-9d6e-cd4374b6a4c0.html"
];

const SEPARATE_QUOTA_PAGES: [&'static str; 84] = [
    "list_6a3e3438-da17-4af3-904a-ff0bfbf299b6.html",
    "list_7a647dc0-a44e-4c65-8c66-6d9390fe6e22.html",
    "list_eb8a5e47-2909-4a30-a972-aba751d55fa0.html",
    "list_58d133f9-19fa-4072-b272-50254068b9fe.html",
    "list_b0289705-9763-435b-9ed3-5e1712787666.html",
    "list_1f12e8c9-4549-4f9f-bdc8-4022a832f019.html",
    "list_fd187d4e-83d5-4ebc-b84e-671a10ef96d1.html",
    "list_025addb2-3fe9-495f-b73f-1f9eedb66487.html",
    "list_83dec6fd-60ed-456a-a0eb-9042dec3760a.html",
    "list_c1a8eaa0-c86d-43ef-8890-6594dde4aae7.html",
    "list_f7067586-7fb7-49ee-93a4-d461c6c301ce.html",
    "list_03f09d2b-0e92-463a-83d6-100bb23aafc1.html",
    "list_256886f2-34d4-435c-b0ff-8f0ffc1fe5ed.html",
    "list_72bb69c8-5408-487d-b1ec-bfe9eab3dc73.html",
    "list_0b464644-88d9-4717-aee4-fd0161511d90.html",
    "list_ea955482-3f1d-4cda-9127-823931a1d50c.html",
    "list_7fb6c5e0-8e9b-4fc6-9781-eb4b19ce7973.html",
    "list_11f4ed99-ccf5-4976-82bd-1628b11b1d9d.html",
    "list_1f48f571-956e-4170-aa1a-5af3b0c733d6.html",
    "list_8baef634-66bd-4754-ab0f-7fbc7ef9c09d.html",
    "list_b099e30b-8ebd-449c-8962-e77d1f87ccec.html",
    "list_688b88d0-9cc6-4969-837a-b53847ce101c.html",
    "list_94702a28-92cb-461c-b582-39bcb443a547.html",
    "list_9e1a50c1-3b5a-47eb-932f-0f4923e44498.html",
    "list_3edff4c3-5cfa-4ac5-8c4b-f41e9ff8e046.html",
    "list_ba48675d-a51d-46d0-9906-3f89c1ae506f.html",
    "list_42dec5dd-144c-461d-8d46-b096e202e50c.html",
    "list_c82cc0f7-74ca-4bec-b908-ac6431ee318b.html",
    "list_ad5508d8-76ab-4946-9cc6-ed48100584e0.html",
    "list_00a55920-b434-4989-99c7-7ee4ae58e4c7.html",
    "list_e7144355-f4ac-45cb-825f-9f0dc3c69e22.html",
    "list_4f118f64-2470-4610-8584-f662d55e41d7.html",
    "list_f1b8767a-7107-455e-9629-cf86f0457510.html",
    "list_eee93587-2cb7-4814-96eb-83efb0e7cb16.html",
    "list_db506e41-e6e5-4c74-bdd7-f39a7f37072e.html",
    "list_bbbfd0bd-f12b-4276-90b4-4380e35e443c.html",
    "list_af05316c-45df-4468-ba89-c1d5e8810bee.html",
    "list_4a555ce9-5741-440d-b877-7411c8e48724.html",
    "list_9aa1b484-a32e-4f59-b2e0-382556672ad5.html",
    "list_26e93891-838f-4fd8-81d9-13c0e42e8d43.html",
    "list_aac4648d-5f5e-4f74-a7b6-9c30f941c719.html",
    "list_429c2e94-0bad-4883-910e-bca077767907.html",
    "list_adb752e8-6482-4d97-886d-29daf1cafa57.html",
    "list_0ce26e0e-77d8-4c5f-989e-4da12659ced6.html",
    "list_e66a136d-9f42-46b2-b696-403139006169.html",
    "list_776186c8-3f1e-4f81-bbc7-eda09079f26c.html",
    "list_61f23317-e03a-491e-8160-738405057846.html",
    "list_697c71ea-14e7-4ad9-90d7-99dd8cb7aec3.html",
    "list_7607a0fc-15d7-4b74-b281-0afb9dfea7f1.html",
    "list_76b7bc68-c50c-4d95-b37c-69ff112817c5.html",
    "list_2fe76219-abcb-4096-8439-961cf05c5e4a.html",
    "list_e8030363-8f92-4781-8f83-34ef196c4f88.html",
    "list_69089918-f0c7-446a-a4bf-288d718f2f3c.html",
    "list_9d621e90-9bc7-4840-af7d-a93cfa5e9190.html",
    "list_c651080b-846e-4551-9aa0-8659fa2fcf43.html",
    "list_3f1604bb-4d35-4303-99b4-f800aaee09aa.html",
    "list_a295840f-b426-4b3d-9e89-3c49aeca9d20.html",
    "list_39760649-3eba-4f13-9a9c-180eeab096c3.html",
    "list_b6443562-5d2f-4c41-a771-85d30f6ea88b.html",
    "list_62658984-8e6d-4f92-b318-5255c26e76da.html",
    "list_a929899b-85ba-4902-97b3-ebff51cdd59a.html",
    "list_34422408-7d25-462b-ae07-e9a327fd0aa0.html",
    "list_15299336-4ed0-4301-9f65-0fde9041cf3d.html",
    "list_9d44ab84-755e-456e-9091-29eb3e22a64b.html",
    "list_96f2590b-b7fd-4079-a71a-606bc5429b29.html",
    "list_9d8ad588-1e28-47e8-8b11-7cf9cb9c359c.html",
    "list_5a2902ff-fb18-46f5-ba2a-a1fbb5abf0a9.html",
    "list_8b75da4d-4ecd-4dd9-b4b7-7f406d9ee5d2.html",
    "list_a7a92ef9-f8ad-48e4-a56c-9b9a2bf25ab0.html",
    "list_f6048691-ef2b-4405-8a41-41bcfa16448b.html",
    "list_75758375-f61f-4a66-a7e9-dde132a29128.html",
    "list_6cbd0b13-89b2-4ddc-a429-c4fe405753fe.html",
    "list_f0e23aca-a6b9-4102-af1c-f2fc20842944.html",
    "list_f211ddf8-a7d8-47df-9833-ffd09efaa837.html",
    "list_3bb222fa-9dab-4bec-8ea5-778d3a470f30.html",
    "list_aadf4aca-1501-47bc-9b87-ee205a135aa7.html",
    "list_731db22d-eec9-47a7-a29e-9097876ed9a1.html",
    "list_5f565dde-6cdf-478e-8f4f-8c60a123661c.html",
    "list_b6a03965-64d6-4c02-9713-8e27b471f497.html",
    "list_ee151ec5-1c5d-4d73-9f20-0751aa0e5088.html",
    "list_6401c4c8-daa6-45d5-a9df-fc57f1e3177f.html",
    "list_b06e60cc-01a2-4000-b4aa-436a65d8d935.html",
    "list_5511e861-5865-43b0-af99-320f04c8fff0.html",
    "list_e360b1be-aac8-40c9-b125-d415f0585512.html"
];

const SITE_ID_PATTERN: &'static str = "<b>Образовательная программа:<\\/b>(.*)<br><\\/br>";
const SITE_CAPACITY_PATTERN: &'static str = "<b>КЦП по конкурсу:<\\/b>(.*)<br><\\/br>";

const QUOTA_USED_PATTERN: &'static str = "<b>Количество поданных заявлений:<\\/b>(.*)<br><\\/br>";

enum QuotaKind {
    Target,
    Special,
    Separate, //ZOV
}

impl QuotaKind {
    fn regex(&self) -> &Regex {
        lazy_static! {
            static ref TARGET_RE: Regex = Regex::new("<b>Целевая квота:<\\/b>(.*)<br><\\/br>").unwrap();
            static ref SPECIAL_RE: Regex = Regex::new("<b>Особая квота:<\\/b>(.*)<br><\\/br>").unwrap();
            static ref SEPARATE_RE: Regex = Regex::new("<b>КЦП по конкурсу:<\\/b>(.*)<br><\\/br>").unwrap();
        };

        match self {
            Self::Target => &TARGET_RE,
            Self::Special => &SPECIAL_RE,
            Self::Separate => &SEPARATE_RE,
        }
    }
}

impl Loader {
    pub fn new() -> Self {
        Self {
            info: Info::new(),
        }
    }

    pub fn run(&mut self, with_quotas: bool) {
        println!("Loading the pages...");
        let mut bar = ProgressBar::new(PAGES.len() as u64);
        for i in 0..PAGES.len() {
            let page = PAGES[i];
            self.load_page((PAGE_START.to_owned() + page).as_str());

            // let percent = ((i + 1) as f64 / PAGES.len() as f64 * 100.).round() as i32;
            // println!("Loading the pages... {}%", percent);

            bar.inc(1);
        }
        bar.finish();

        macro_rules! load_quotas {
            ($arr:ident; $kind:expr; $log_name:expr) => {
                println!("Loading the {} pages...", $log_name);
                let mut bar = ProgressBar::new($arr.len() as u64);
                for i in 0..$arr.len() {
                    let page = $arr[i];
                    self.load_quota_page((PAGE_START.to_owned() + page).as_str(), $kind);

                    // let percent = ((i + 1) as f64 / $arr.len() as f64 * 100.).round() as i32;
                    // println!("Loading the {} pages... {}%", $log_name, percent);

                    bar.inc(1);
                }
                bar.finish();
            }
        }

        if with_quotas {
            load_quotas!(TARGET_QUOTA_PAGES; QuotaKind::Target; "target quota");
            load_quotas!(SPECIAL_QUOTA_PAGES; QuotaKind::Special; "special quota");
            load_quotas!(SEPARATE_QUOTA_PAGES; QuotaKind::Separate; "separate quota");
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
            let _comp_ty = children[2].get(parser).unwrap().inner_text(parser);
            let priority = children[3].get(parser).unwrap().inner_text(parser);
            let sum_total = children[4].get(parser).unwrap().inner_text(parser);
            let _sum_scores = children[5].get(parser).unwrap().inner_text(parser);

            let mut scores = vec![];

            let mut i = 6;

            while i < (len - 3) {
                scores.push(children[i].get(parser).unwrap().inner_text(parser).as_ref().to_string());

                i += 1;
            }

            let _sum_ind = children[i].get(parser).unwrap().inner_text(parser);
            let _ind_descr = children[i + 1].get(parser).unwrap().inner_text(parser);
            let _optional = children[i + 2].get(parser).unwrap().inner_text(parser);

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

    fn load_quota_page(&mut self, addr: &str, kind: QuotaKind) {
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
            static ref QUOTA_USED_RE: Regex = Regex::new(QUOTA_USED_PATTERN).unwrap();
        }

        let site_id = SITE_ID_RE.captures(about.as_ref()).unwrap()[1].trim().to_string();

        if !self.info.site_capacities.contains_key(&site_id) {
            return;
        }

        let quota_asked: usize = QUOTA_USED_RE.captures(about.as_ref()).unwrap()[1].trim().parse().unwrap();
        let quota_max: usize = kind.regex().captures(about.as_ref()).unwrap()[1].trim().parse().unwrap();

        let remove = min(quota_asked, quota_max);

        let current_capacity = self.info.get_site_capacity(&site_id);

        self.info.set_site_capacity(
            &site_id,
            if remove > current_capacity {
                // println!("Removed all places from {site_id}");
                0
            } else {
                // println!("Removed {remove} places from {site_id} ({current_capacity} -> {})", current_capacity - remove);
                current_capacity - remove
            }
        );
    }

    pub fn release(self) -> Info {
        self.info
    }
}