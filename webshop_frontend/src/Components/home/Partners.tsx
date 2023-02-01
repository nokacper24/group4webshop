import React, { Component } from 'react';
import {Partner} from './Partner';

type PartnersProps = {
    partners: {
        name: string;
        link: string;
        imageSize: number[];
        imageSource: string;
    }[];
}

export const Partners = ({partners}: PartnersProps) => {
    return (
        <ul className="partners">
            {partners.map((partner) => (
                <Partner
                    key={partner.name}
                    partner={partner}
                />
            ))}
        </ul>
    );
}