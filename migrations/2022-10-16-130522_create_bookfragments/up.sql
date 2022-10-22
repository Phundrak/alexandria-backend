-- Your SQL goes here
CREATE TYPE ImageType AS ENUM ('none', 'url', 'auto', 'same');
CREATE TYPE SoundType as ENUM ('none', 'url', 'same');

CREATE TABLE BookFragments (
       Id VARCHAR(511) PRIMARY KEY,
       Content TEXT NOT NULL,
       OneShotSoundSource VARCHAR(255),
       BgSoundType SoundType NOT NULL DEFAULT 'none',
       BgSoundSource VARCHAR(255),
       ImgType ImageType NOT NULL DEFAULT 'none',
       ImgSource VARCHAR(255),
       Book VARCHAR(255)
            REFERENCES Books(Id)
            ON UPDATE CASCADE
            ON DELETE CASCADE
            NOT NULL,
       Chapter INTEGER NOT NULL,
       Rank INTEGER NOT NULL
);
