import { Component } from "react";
import { SlideType } from "./SlideTypes";

export type SlidesProps = {
  mainContent: string;
  reviewerProfile: {
    picturePath: string;
    name: string;
    title: string;
  };
  slideType: SlideType;
};

export default class Gallery extends Component<SlidesProps[]> {
  

  main() {
    
    let slide: JSX.Element[];

    this.props.forEach(function (prop) {
      switch (prop.slideType) {
        case SlideType.PARAGRAPH: {
        }
      }
    });
  }
}

/* export default function (slideProps: SlideProps[]) {
  let slide: JSX.Element[];

  slideProps.forEach(function (prop) {
    switch (prop.slideType) {
      case SlideType.PARAGRAPH: {
      }
    }
  });

  return <div></div>;
} */
