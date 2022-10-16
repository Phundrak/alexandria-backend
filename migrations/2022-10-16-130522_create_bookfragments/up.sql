-- Your SQL goes here
create type ImageType as enum ('none', 'url', 'auto');
create type SoundType as enum ('none', 'url');

create table BookFragments (
       Id varchar(511) primary key,
       Content text not null,
       OneShotSoundSource varchar(255),
       BgSoundType SoundType,
       BgSoundSource varchar(255),
       ImgType ImageType,
       ImgSource varchar(255),
       Book varchar(255)
            references Books(Id)
            on update cascade
            on delete cascade
            not null,
       Chapter integer not null,
       Rank integer not null
);
