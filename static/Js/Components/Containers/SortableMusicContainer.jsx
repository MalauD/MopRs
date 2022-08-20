import React from 'react';
import PropTypes from 'prop-types';
import { SortableContainer } from 'react-sortable-hoc';
import MediaLayout from '../Layout/MediaLayout';

export default class SortableMusicContainer extends React.Component {
    static propTypes = {
        children: PropTypes.arrayOf(PropTypes.element).isRequired,
        accessories: PropTypes.arrayOf(PropTypes.element),
        title: PropTypes.string.isRequired,
    };

    static defaultProps = {
        accessories: [],
    };

    render() {
        const { title, children, accessories, ...props } = this.props;
        const MySortableContainer = SortableContainer(() => (
            <MediaLayout title={title} accessories={accessories} {...props}>
                <table className="table table-hover table-borderless">
                    <tbody>{children}</tbody>
                </table>
            </MediaLayout>
        ));

        return (
            <MySortableContainer useDragHandle useWindowAsScrollContainer {...props}>
                {children}
            </MySortableContainer>
        );
    }
}
