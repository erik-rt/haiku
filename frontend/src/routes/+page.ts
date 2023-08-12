import type { PageLoad } from './$types';

export const load: PageLoad = async () => {
	return {
		haikus: (await fetch('http://127.0.0.1:8000').then((data) => data.json())) as Haiku[]
	};
};
