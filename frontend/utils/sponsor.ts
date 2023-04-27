export interface Sponsor {
    uid: string,
    name: string,
    imageUrl?: string,
    shortDescription: string,
    fields: SponsorField[],
    tags: string[],
    favoursCompleted: boolean,
    favours: SponsorFavour[]
}

export interface SponsorField {
    name: string,
    value: string,
}

export interface SponsorFavour {
    uid: string,
    sponsorUid: string,
    condition: string,
    completed: boolean,
    comment: string,
    dueUntil: Date | undefined,
}
