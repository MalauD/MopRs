import React from 'react';
import PropTypes from 'prop-types';
import { Row, Col } from 'react-bootstrap';
import { SortableContainer } from 'react-sortable-hoc';

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
        const MySortableContainer = SortableContainer(({ c }) => (
            <div className="m-4">
                <small className="text-muted">
                    <Row className="p-1">
                        <Col xs={9} className="mr-auto">
                            <h3 className="align-self-center my-auto">{title}</h3>
                        </Col>
                        <Col xs={3}>
                            <Row>
                                <Col md="auto" className="mr-auto" />
                                {accessories.map((accessory) => (
                                    <Col xs={2} className="mx-1">
                                        {accessory}
                                    </Col>
                                ))}
                            </Row>
                        </Col>
                    </Row>
                </small>
                <table className="table table-hover table-borderless">
                    <tbody>{c}</tbody>
                </table>
            </div>
        ));

        return (
            <MySortableContainer useDragHandle {...props}>
                {children}
            </MySortableContainer>
        );
    }
}
