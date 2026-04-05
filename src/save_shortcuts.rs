use std::collections::HashMap;

use crate::{
    blocks::{
        gv_45::StaticChapter, gv_66::TowerChapters, gv_70000::UncannyChapters, gv_80000::Talent,
        gv_80600::LegendQuest, gv_90000::Medals, gv_90300::GauntletChapters, gv_90400::Enigma,
        gv_120000::NewChapters,
    },
    game::main_story::TOTAL_CLEAR_TIME_STAGES,
    save::Save,
};

impl Save {
    pub fn calculate_user_rank(&self) -> i32 {
        let mut ur: i32 = 0;
        for (unlocked, level) in self.unlocked_cats.iter().zip(self.cat_upgrades.iter()) {
            if *unlocked != 0 {
                ur += (level.base() as i32) + (level.plus as i32);
            }
        }

        ur
    }

    pub fn catseyes(&self) -> &Vec<i32> {
        &self.gv_53.catseyes
    }
    pub fn catseyes_mut(&mut self) -> &mut Vec<i32> {
        &mut self.gv_53.catseyes
    }

    pub fn labyrinth_medals(&self) -> &Vec<i16> {
        &self.gv_111000.labyrinth_medals
    }
    pub fn labyrinth_medals_mut(&mut self) -> &mut Vec<i16> {
        &mut self.gv_111000.labyrinth_medals
    }

    pub fn treasure_chests(&self) -> &Vec<i32> {
        &self.gv_140300.treasure_chests
    }
    pub fn treasure_chests_mut(&mut self) -> &mut Vec<i32> {
        &mut self.gv_140300.treasure_chests
    }

    pub fn catamins(&self) -> &Vec<i32> {
        &self.gv_53.catamins
    }
    pub fn catamins_mut(&mut self) -> &mut Vec<i32> {
        &mut self.gv_53.catamins
    }

    pub fn talent_orbs(&self) -> &HashMap<i16, i16> {
        &self.gv_90700.talent_orbs
    }
    pub fn talent_orbs_mut(&mut self) -> &mut HashMap<i16, i16> {
        &mut self.gv_90700.talent_orbs
    }

    pub fn catfruit(&self) -> &Vec<i32> {
        &self.gv_53.catfruit
    }
    pub fn catfruit_mut(&mut self) -> &mut Vec<i32> {
        &mut self.gv_53.catfruit
    }

    pub fn platinum_shards(&self) -> i32 {
        self.gv_100600.platinum_shards
    }
    pub fn platinum_shards_mut(&mut self) -> &mut i32 {
        &mut self.gv_100600.platinum_shards
    }

    pub fn np(&self) -> i32 {
        self.gv_80000.np
    }
    pub fn np_mut(&mut self) -> &mut i32 {
        &mut self.gv_80000.np
    }

    pub fn engineers(&self) -> i32 {
        self.gv_64.ototo.engineers
    }
    pub fn engineers_mut(&mut self) -> &mut i32 {
        &mut self.gv_64.ototo.engineers
    }
    pub fn ototo_cannon(&self) -> &HashMap<i32, Vec<i32>> {
        &self.gv_64.ototo.cannon_levels
    }
    pub fn ototo_cannon_mut(&mut self) -> &mut HashMap<i32, Vec<i32>> {
        &mut self.gv_64.ototo.cannon_levels
    }

    pub fn gamatoto_xp(&self) -> i32 {
        self.gv_53.gamatoto.xp
    }
    pub fn gamatoto_xp_mut(&mut self) -> &mut i32 {
        &mut self.gv_53.gamatoto.xp
    }
    pub fn cat_shrine_xp(&self) -> i64 {
        self.gv_90900.cat_shrine.xp_offering
    }
    pub fn cat_shrine_xp_mut(&mut self) -> &mut i64 {
        &mut self.gv_90900.cat_shrine.xp_offering
    }

    pub fn gamatoto_helpers(&self) -> &Vec<i32> {
        &self.gv_54.block_1.gamatoto_helpers
    }
    pub fn gamatoto_helpers_mut(&mut self) -> &mut Vec<i32> {
        &mut self.gv_54.block_1.gamatoto_helpers
    }

    pub fn base_materials(&self) -> &Vec<i32> {
        &self.gv_64.base_materials
    }
    pub fn base_materials_mut(&mut self) -> &mut Vec<i32> {
        &mut self.gv_64.base_materials
    }

    pub fn leadership(&self) -> i16 {
        self.gv_80200.leadership
    }
    pub fn leadership_mut(&mut self) -> &mut i16 {
        &mut self.gv_80200.leadership
    }

    pub fn platinum_tickets(&self) -> i32 {
        self.gv_55.platinum_tickets
    }

    pub fn legend_tickets(&self) -> i32 {
        self.gv_100000.legend_tickets
    }
    pub fn platinum_tickets_mut(&mut self) -> &mut i32 {
        &mut self.gv_55.platinum_tickets
    }
    pub fn inquiry_code(&self) -> &str {
        &self.gv_44.inquiry_code
    }
    pub fn inquiry_code_mut(&mut self) -> &mut String {
        &mut self.gv_44.inquiry_code
    }

    pub fn legend_tickets_mut(&mut self) -> &mut i32 {
        &mut self.gv_100000.legend_tickets
    }

    pub fn play_time(&self) -> i32 {
        self.gv_44.play_time
    }

    pub fn password_refresh_token(&self) -> &str {
        &self.gv_100000.password_refresh_token
    }
    pub fn password_refresh_token_mut(&mut self) -> &mut String {
        &mut self.gv_100000.password_refresh_token
    }

    pub fn cat_talents(&self) -> &HashMap<i32, Vec<Talent>> {
        &self.gv_80000.cat_talents
    }
    pub fn cat_talents_mut(&mut self) -> &mut HashMap<i32, Vec<Talent>> {
        &mut self.gv_80000.cat_talents
    }

    pub fn catguide_collected(&self) -> &Vec<bool> {
        &self.gv_52.catguide_collected
    }
    pub fn catguide_collected_mut(&mut self) -> &mut Vec<bool> {
        &mut self.gv_52.catguide_collected
    }

    pub fn event_seed(&self) -> u32 {
        self.gv_47.event_seed
    }
    pub fn event_seed_mut(&mut self) -> &mut u32 {
        &mut self.gv_47.event_seed
    }

    pub fn meow_medals(&self) -> &Medals {
        &self.gv_90000.medals
    }
    pub fn meow_medals_mut(&mut self) -> &mut Medals {
        &mut self.gv_90000.medals
    }

    pub fn challenge_scores(&self) -> &Vec<i32> {
        &self.gv_67.challenge_scores
    }
    pub fn challenge_scores_mut(&mut self) -> &mut Vec<i32> {
        &mut self.gv_67.challenge_scores
    }

    pub fn itf_timed_scores(&self) -> &[StaticChapter<TOTAL_CLEAR_TIME_STAGES>; 3] {
        &self.gv_45.itf_timed_scores
    }
    pub fn itf_timed_scores_mut(&mut self) -> &mut [StaticChapter<TOTAL_CLEAR_TIME_STAGES>; 3] {
        &mut self.gv_45.itf_timed_scores
    }

    pub fn dojo(&self) -> &HashMap<i32, HashMap<i32, i32>> {
        &self.gv_58.dojo_chapters
    }
    pub fn dojo_mut(&mut self) -> &mut HashMap<i32, HashMap<i32, i32>> {
        &mut self.gv_58.dojo_chapters
    }
    pub fn outbreaks(&self) -> &HashMap<i32, HashMap<i32, bool>> {
        &self.gv_59.outbreaks
    }
    pub fn outbreaks_mut(&mut self) -> &mut HashMap<i32, HashMap<i32, bool>> {
        &mut self.gv_59.outbreaks
    }

    pub fn aku_realm(&self) -> &Vec<Vec<Vec<i16>>> {
        &self.gv_100900.aku.stages
    }
    pub fn aku_realm_mut(&mut self) -> &mut Vec<Vec<Vec<i16>>> {
        &mut self.gv_100900.aku.stages
    }

    pub fn enigma(&self) -> &Enigma {
        &self.gv_90400.enigma
    }
    pub fn enigma_mut(&mut self) -> &mut Enigma {
        &mut self.gv_90400.enigma
    }

    pub fn gauntlets(&self) -> &GauntletChapters {
        &self.gv_90300.gauntlet_chapters
    }
    pub fn gauntlets_mut(&mut self) -> &mut GauntletChapters {
        &mut self.gv_90300.gauntlet_chapters
    }

    pub fn collab_gauntlets(&self) -> &GauntletChapters {
        &self.gv_90500.collab_gauntlets
    }
    pub fn collab_gauntlets_mut(&mut self) -> &mut GauntletChapters {
        &mut self.gv_90500.collab_gauntlets
    }
    pub fn behemoth_culling(&self) -> &GauntletChapters {
        &self.gv_110500.behemoth_culling
    }
    pub fn behemoth_culling_mut(&mut self) -> &mut GauntletChapters {
        &mut self.gv_110500.behemoth_culling
    }

    pub fn uncanny_legends(&self) -> &UncannyChapters {
        &self.gv_70000.uncanny_chapters
    }
    pub fn uncanny_legends_mut(&mut self) -> &mut UncannyChapters {
        &mut self.gv_70000.uncanny_chapters
    }

    pub fn catamin_stages(&self) -> &UncannyChapters {
        &self.gv_70100.catamin_stages
    }
    pub fn catamin_stages_mut(&mut self) -> &mut UncannyChapters {
        &mut self.gv_70100.catamin_stages
    }

    pub fn legend_quest(&self) -> &LegendQuest {
        &self.gv_80600.legend_quest_chapters
    }
    pub fn legend_quest_mut(&mut self) -> &mut LegendQuest {
        &mut self.gv_80600.legend_quest_chapters
    }
    pub fn tower(&self) -> &TowerChapters {
        &self.gv_66.tower
    }
    pub fn tower_mut(&mut self) -> &mut TowerChapters {
        &mut self.gv_66.tower
    }
    pub fn zero_legends(&self) -> &NewChapters {
        &self.gv_120000.zero_legends
    }
    pub fn zero_legends_mut(&mut self) -> &mut NewChapters {
        &mut self.gv_120000.zero_legends
    }
    pub fn catclaw_championships(&self) -> &NewChapters {
        &self.gv_140000.dojo_chapters
    }
    pub fn catclaw_championships_mut(&mut self) -> &mut NewChapters {
        &mut self.gv_140000.dojo_chapters
    }
}
