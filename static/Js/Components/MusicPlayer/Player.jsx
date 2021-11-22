import React from 'react';
import {
	Col, Image, Navbar, Row, Button,
} from 'react-bootstrap';
import PropTypes from 'prop-types';
import { connect } from 'react-redux';
import Axios from 'axios';
import ButtonIcon from '../Helper/ButtonIcon';
import {
	ChangePlayingId as ChangePlayingIdRedux,
	UpdateCurrentPlaylist as UpdateCurrentPlaylistRedux,
	AddMultipleMusics as AddMultipleMusicsRedux
} from '../../Actions/Action';
import PlayerSlider from './PlayerSlider';

const mapStateToProps = (state) => {
	const { Playlist } = state.MusicPlayerReducer;
	return {
		PlayingMusic: Playlist.Musics[state.MusicPlayerReducer.Playlist.PlayingId],
		NextMusic: Playlist.Musics[state.MusicPlayerReducer.Playlist.PlayingId + 1],
		CurrentMusicId: Playlist.PlayingId,
		PlaylistLength: Playlist.Musics.length,
		MusicFilePath: Playlist.Musics[Playlist.PlayingId] ? `/Music/cdn/${Playlist.Musics[Playlist.PlayingId]._id}` : undefined,
	};
};

const mapDispatchToProps = (dispatch) => ({
	ChangePlayingId: (id) => {
		dispatch(ChangePlayingIdRedux(id));
	},
	UpdateCurrentPlaylist: (Musics, PlayingId) => {
		dispatch(UpdateCurrentPlaylistRedux(Musics, PlayingId));
	},
	AddMultipleMusics: (Musics) => {
		dispatch(AddMultipleMusicsRedux(Musics));
	},
});

class PlayerConnected extends React.Component {
	static propTypes = {
		history: PropTypes.shape({
			push: PropTypes.func.isRequired,
			goBack: PropTypes.func.isRequired,
			location: PropTypes.shape({
				pathname: PropTypes.string.isRequired,
			}),
		}).isRequired,
		ChangePlayingId: PropTypes.func.isRequired,
		UpdateCurrentPlaylist: PropTypes.func.isRequired,
		AddMultipleMusics: PropTypes.func.isRequired,
		MusicFilePath: PropTypes.string,
		PlayingMusic: PropTypes.shape({
			_id: PropTypes.string.isRequired,
			Title: PropTypes.string.isRequired,
			Album: PropTypes.string.isRequired,
			Artist: PropTypes.string.isRequired,
			FilePath: PropTypes.string.isRequired,
			AlbumId: PropTypes.shape({
				Image: PropTypes.string,
				ImagePathDeezer: PropTypes.string,
			}).isRequired,
		}),
		NextMusic: PropTypes.shape({
			Title: PropTypes.string.isRequired,
		}),
		CurrentMusicId: PropTypes.number,
	}

	static defaultProps = {
		PlayingMusic: undefined,
		NextMusic: undefined,
		CurrentMusicId: undefined,
		MusicFilePath: undefined,
	}

	constructor(props) {
		super(props);
		this.state = {
			IsPlaying: true,
		};
	}


	HandleTimeUpdate = () => {
		this.forceUpdate();
	};

	HandlePlay = () => {
		this.setState((prevState) => ({ IsPlaying: !prevState.IsPlaying }), () => {
			const { IsPlaying } = this.state;
			if (this.player) {
				IsPlaying ? this.player.play() : this.player.pause();
			}
		});
	};

	HandleNext = () => {
		const { NextMusic, ChangePlayingId, CurrentMusicId } = this.props;
		if (NextMusic) ChangePlayingId(CurrentMusicId + 1);
	}

	HandleBack = () => {
		const { ChangePlayingId, CurrentMusicId } = this.props;
		if (CurrentMusicId !== 0) ChangePlayingId(CurrentMusicId - 1);
	}

	HandleSliderChange = (PosX) => {
		this.player.currentTime = PosX;
		this.forceUpdate();
	};

	componentWillUnmount = () => {
		clearInterval(this.refreshPlayer);
	};

	componentDidMount = () => {
		const { UpdateCurrentPlaylist } = this.props;
		const { IsPlaying } = this.state;
		if (this.player) {
			IsPlaying ? this.player.play() : this.player.pause();
		}
		Axios.get('/User/CurrentPlaylist')
			.then(({ data }) => {
				UpdateCurrentPlaylist(data.CurrentPlaylist, data.CurrentPlaylistPlaying);
			});
	}

	OnPlayerEnd = () => {
		const { NextMusic, ChangePlayingId, CurrentMusicId, AddMultipleMusics } = this.props;
		if(!NextMusic) {
			Axios.get('/Music/Selection/v1')
				.then((res) => {
					AddMultipleMusics(res.data);
					if(res.data.length !== 0) {
						ChangePlayingId(CurrentMusicId + 1)
					}else {
						this.onPause();
					}
				})
		} else {
			ChangePlayingId(CurrentMusicId + 1)
		}
		
	};

	OnPlay = () => {
		this.setState({
			IsPlaying: true,
		});
	}

	OnPause = () => {
		this.setState({
			IsPlaying: false,
		});
	}

	GetSliderMaxValue = () => {
		if (this.player) {
			return this.player.duration ? this.player.duration : 0;
		}
		return 0;
	}


	HandleOpenPlaylist = () => {
		const { history } = this.props;
		if (history.location.pathname === '/CurrentPlaylist') {
			history.goBack();
		} else {
			history.push('/CurrentPlaylist');
		}
	};

	render() {
		const { IsPlaying } = this.state;
		const { PlayingMusic, NextMusic, MusicFilePath } = this.props;

		const PlayingIcon = !IsPlaying ? "play-circle-outline" : "pause-circle-outline";

		document.title = `${PlayingMusic ? PlayingMusic.Title : ''} MOP Js Edition`;

		if (PlayingMusic) {
			return (
				<>
					<Navbar fixed="bottom" className="px-2 mh-50 pt-0">
						<div id="Player" className="d-flex flex-column w-100 overflow-auto">
							<Row className="w-100 mx-0 py-0">
								<PlayerSlider
									Time={this.player ? this.player.currentTime : 0}
									EndTime={this.GetSliderMaxValue()}
									OnSliderChange={this.HandleSliderChange}
								/>
								{PlayingMusic.AlbumId.ImagePathDeezer
									? <Image className="PlayerImage my-auto" rounded height="75em" src={PlayingMusic.AlbumId.ImagePathDeezer} />
									: (
										<Image
											className="PlayerImage my-auto"
											rounded
											height="75em"
											src={PlayingMusic.AlbumId.Image
												? `data:image/jpeg;base64,${PlayingMusic.AlbumId.Image.toString('base64')}` : '/Ressources/noMusic.jpg'}
										/>
									)}

								<Col className="my-1 mt-0 col-md-auto  text-truncate" onClick={this.HandleOpenPlaylist}>
									<h6>{PlayingMusic.Title}</h6>
									<p>
										{PlayingMusic.Artist}
									</p>
								</Col>
								<ButtonIcon buttonClass="my-auto mx-2 ml-auto p-0" iconFontSize="1.75rem" onClick={this.HandleBack} style={{transform: "scale(-1)"}}  evaOptions={{fill: "#d6d6d6ff", width: '30px', height: '30px'}}  dataEva={"skip-forward-outline"} />

								<ButtonIcon buttonClass="my-auto mx-2 p-0" iconFontSize="1.75rem" onClick={this.HandlePlay} evaOptions={{fill: "#d6d6d6ff", width: '30px', height: '30px'}}  dataEva={PlayingIcon} />

								<ButtonIcon buttonClass="my-auto ml-2 p-0 mr-0" iconFontSize="1.75rem" onClick={this.HandleNext} evaOptions={{fill: "#d6d6d6ff", width: '30px', height: '30px'}}  dataEva={"skip-forward-outline"} />


								<Button variant="" className="my-auto ml-1 mt-1 d-none d-lg-block" onClick={this.HandleOpenPlaylist}>
									{NextMusic ? `Next: ${NextMusic.Title}` : 'Queue'}
								</Button>
							</Row>

							<audio
								ref={(ref) => { this.player = ref; }}
								src={MusicFilePath}
								onTimeUpdate={this.HandleTimeUpdate}
								onEnded={this.OnPlayerEnd}
								onPlay={this.OnPlay}
								onPause={this.OnPause}
								autoPlay
							>
								No html5 player
							</audio>
						</div>
					</Navbar>
				</>
			);
		}
		return <div />;
	}
}

const Player = connect(mapStateToProps, mapDispatchToProps)(PlayerConnected);

export default Player;
