import React from 'react';
import PropTypes from 'prop-types';
import axios from 'axios';
import ButtonIcon from '../Helper/ButtonIcon';
import MusicGroup from './Groups/MusicGroup';

export default class RelatedMusics extends React.Component {
    static propTypes = {
        Musics: PropTypes.arrayOf(
            PropTypes.shape({
                _id: PropTypes.number.isRequired,
            })
        ).isRequired,
        OnAdd: PropTypes.func,
    };

    static defaultProps = {
        OnAdd: () => {},
    };

    constructor(props) {
        super(props);
        this.state = {
            RelatedMusicsData: [],
            RelatedMusicAdded: false,
            isLoading: false,
        };
    }

    componentDidMount() {
        this.getNewRelatedMusics();
    }

    componentDidUpdate(prevProps) {
        const { Musics } = this.props;
        const { RelatedMusicAdded } = this.state;
        if (prevProps.Musics !== Musics) {
            if (!RelatedMusicAdded) {
                this.getNewRelatedMusics();
            } else {
                this.setState({
                    RelatedMusicAdded: false,
                });
            }
        }
    }

    getNewRelatedMusics = () => {
        const { Musics } = this.props;
        const MusicIds = Musics.map((m) => m._id);
        this.setState({
            isLoading: true,
        });
        axios.post('/Music/Related', { MusicIds }).then((res) => {
            this.setState({
                RelatedMusicsData: res.data.RelatedMusics,
                isLoading: false,
            });
        });
    };

    onReloadRelated = () => {
        this.getNewRelatedMusics();
    };

    onAdd = (Music) => {
        const { RelatedMusicsData } = this.state;
        const { OnAdd } = this.props;

        OnAdd(Music);
        this.setState({
            RelatedMusicAdded: true,
            RelatedMusicsData: RelatedMusicsData.filter((m) => m._id !== Music._id),
        });
    };

    render() {
        const { RelatedMusicsData } = this.state;
        const { isLoading } = this.state;

        const Accessories = [
            <ButtonIcon
                dataEva="flip-outline"
                onClick={this.onReloadRelated}
                evaOptions={{
                    fill: '#d6d6d6ff',
                    width: '30px',
                    height: '30px',
                }}
            />,
        ];

        return (
            <MusicGroup
                Musics={RelatedMusicsData}
                title="Related"
                isLoading={isLoading}
                Accessories={Accessories}
                OnMusicAdded={this.onAdd}
            />
        );
    }
}
