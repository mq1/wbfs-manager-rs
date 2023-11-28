// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-2.0-only

use crate::types::drive::Drive;

pub mod adding_games;
pub mod drives;
pub mod games;

pub enum Page {
    Drives,
    Games(Drive),
    AddingGames(usize),
}
