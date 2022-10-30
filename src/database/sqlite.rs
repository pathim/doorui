use super::{Card, Log};

pub struct Sqlite {
    conn: rusqlite::Connection,
}

impl Sqlite {
    pub fn new(filename: &str) -> rusqlite::Result<Self> {
        let conn = rusqlite::Connection::open(filename)?;
        Ok(Self { conn })
    }
}

impl From<rusqlite::Error> for super::Error {
    fn from(e: rusqlite::Error) -> Self {
        super::Error(format!("sqlite error: {}", e))
    }
}

impl TryFrom<&rusqlite::Row<'_>> for Card {
    type Error = rusqlite::Error;

    fn try_from(r: &rusqlite::Row<'_>) -> Result<Self, Self::Error> {
        Ok(Card {
            uid: r.get(0)?,
            pin: r.get(1)?,
            owner: r.get(2)?,
            member_id: r.get(3)?,
            expiry: r.get(4)?,
            allow_unlock: r.get(5)?,
            allow_entry: r.get(6)?,
            allow_sneaky: r.get(7)?,
            always_sneaky: r.get(8)?,
        })
    }
}

impl TryFrom<&rusqlite::Row<'_>> for Log {
    type Error = rusqlite::Error;

    fn try_from(r: &rusqlite::Row<'_>) -> Result<Self, Self::Error> {
        Ok(Log {
            date: r.get(0)?,
            card_uid: r.get(1)?,
            req_type: r.get(2)?,
            door_name: r.get(3)?,
            id: r.get(4)?,
        })
    }
}

impl super::Db for Sqlite {
    fn get_cards(&self) -> super::Result<Vec<super::Card>> {
        let mut stmt=self.conn.prepare("SELECT uid, pin, owner,member_id, expiry, allow_unlock, allow_entry, allow_sneaky, always_sneaky FROM cards;")?;
        let res = stmt
            .query_map([], |r| r.try_into())?
            .collect::<Result<_, _>>()?;
        Ok(res)
    }

    fn insert_card(&self, card: super::Card) -> super::Result<()> {
        self.conn.execute(
			"INSERT INTO cards(uid, pin, owner,member_id, expiry, allow_unlock,allow_entry, allow_sneaky, always_sneaky)
			VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8);",
	(card.uid,card.pin,card.owner,card.member_id,card.expiry,card.allow_unlock,card.allow_entry,card.allow_sneaky,card.always_sneaky))?;
        Ok(())
    }

    fn remove_card(&self, uid: String) -> super::Result<()> {
        self.conn.execute("DELETE FROM cards WHERE uid=?;", [uid])?;
        Ok(())
    }

    fn get_logs_success(&self) -> super::Result<Vec<super::Log>> {
        let mut stmt = self
            .conn
            .prepare("SELECT date, card_uid, req_type, door_name, id FROM requests_success;")?;
        let res = stmt
            .query_map([], |r| r.try_into())?
            .collect::<Result<_, _>>()?;
        Ok(res)
    }

    fn get_logs_failure(&self) -> super::Result<Vec<super::Log>> {
        let mut stmt = self
            .conn
            .prepare("SELECT date, card_uid, req_type, door_name, id FROM requests_failure;")?;
        let res = stmt
            .query_map([], |r| r.try_into())?
            .collect::<Result<_, _>>()?;
        Ok(res)
    }
}
