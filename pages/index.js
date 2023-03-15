import React from 'react';
import { Formik, Form, Field } from 'formik';
import { TextField, Button, Grid } from '@mui/material';

export default function Home() {
	const { invoke } = typeof window !== 'undefined' ? window.__TAURI__ ?? {} : {};

	const initialValues = {
		artist: 'Artist',
		title: 'Awesome Song',
		album: 'Greatest Hits'
	};

	const onSubmit = (values) => {
		const artist = values.artist;
		const title = values.title;
		const album = values.album;
		invoke('handle_send_now_playing', {
			artist,
			title,
			album
		});
	};

	return (
		<Formik initialValues={initialValues} onSubmit={(values) => onSubmit(values)}>
			{({ handleSubmit }) => (
				<Form onSubmit={handleSubmit}>
					<Grid container spacing={2}>
						<Grid item xs={12}>
							<Field name="artist">
								{({ field }) => <TextField label="Artist" variant="outlined" fullWidth {...field} />}
							</Field>
						</Grid>
						<Grid item xs={12}>
							<Field name="title">
								{({ field }) => <TextField label="Title" variant="outlined" fullWidth {...field} />}
							</Field>
						</Grid>
						<Grid item xs={12}>
							<Field name="album">
								{({ field }) => <TextField label="Album" variant="outlined" fullWidth {...field} />}
							</Field>
						</Grid>
						<Grid item xs={12}>
							<Button variant="contained" color="primary" type="submit">
								Send Now Playing
							</Button>
						</Grid>
					</Grid>
				</Form>
			)}
		</Formik>
	);
}
