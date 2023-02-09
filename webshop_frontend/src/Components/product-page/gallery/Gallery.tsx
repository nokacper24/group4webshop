import { Component } from "react";
import { ParagraphSlide } from "./ParagraphSlide";
import { SlideType } from "./SlideTypes";

export type SlidesProps = {
  slides: {
    mainContent: string;
    reviewerProfile: {
      picturePath: string;
      name: string;
      title: string;
    };
    slideType: SlideType;
  }[];
};

export default class Gallery extends Component<SlidesProps> {
  main() {
    let slides: JSX.Element[];

    this.props.slides.forEach(function (prop) {
      switch (
        prop.slideType //Using switch in case of future expandibiliy, if we ever create more slide types
      ) {
        case SlideType.PARAGRAPH: {
          slides.push(<ParagraphSlide slides={prop} />); //adds a ready made slide component to the list.
        }
      }
    });
  }
}
