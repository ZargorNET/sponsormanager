export interface SearchSponsor {
    id: string,
    name: string,
    shortDescription: string,
    tags: string[],
    fields: SearchSponsorField[],
}

export interface SearchSponsorField {
    name: string,
    value: string,
}

export interface SearchSponsorFavour {
    id: string,
    sponsorUid: string,
    condition: string,
    completed: boolean,
    dueUntil: Date,
}
