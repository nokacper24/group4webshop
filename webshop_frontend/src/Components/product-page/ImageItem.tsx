import {RowItem} from './DescriptionRow'

/**
 * Creates an image in a row
 * 
 * @param rowItem image based RowItem
 * @returns JSX element
 */
export const ImageItem = (rowItem: RowItem | undefined) => {
    let item: JSX.Element;
    if(rowItem === undefined ||rowItem.content === undefined) {
        item = <div className='empty-item'></div>
    } else {
        item = (
        <img src={rowItem.content} alt="" className='row-image'/>
        );
    }
    return item;
}