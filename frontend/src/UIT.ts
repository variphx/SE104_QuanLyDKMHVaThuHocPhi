
import type { CustomThemeConfig } from '@skeletonlabs/tw-plugin';

export const UIT: CustomThemeConfig = {
	name: 'UIT',
	properties: {
		// =~= Theme Properties =~=
		"--theme-font-family-base": `Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'`,
		"--theme-font-family-heading": `Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'`,
		"--theme-font-color-base": "0 0 0",
		"--theme-font-color-dark": "255 255 255",
		"--theme-rounded-base": "9999px",
		"--theme-rounded-container": "8px",
		"--theme-border-base": "1px",
		// =~= Theme On-X Colors =~=
		"--on-primary": "255 255 255",
		"--on-secondary": "0 0 0",
		"--on-tertiary": "0 0 0",
		"--on-success": "0 0 0",
		"--on-warning": "255 255 255",
		"--on-error": "0 0 0",
		"--on-surface": "0 0 0",
		// =~= Theme Colors  =~=
		// primary | #0a2fbb 
		"--color-primary-50": "218 224 245", // #dae0f5
		"--color-primary-100": "206 213 241", // #ced5f1
		"--color-primary-200": "194 203 238", // #c2cbee
		"--color-primary-300": "157 172 228", // #9dace4
		"--color-primary-400": "84 109 207", // #546dcf
		"--color-primary-500": "10 47 187", // #0a2fbb
		"--color-primary-600": "9 42 168", // #092aa8
		"--color-primary-700": "8 35 140", // #08238c
		"--color-primary-800": "6 28 112", // #061c70
		"--color-primary-900": "5 23 92", // #05175c
		// secondary | #69d5d8 
		"--color-secondary-50": "233 249 249", // #e9f9f9
		"--color-secondary-100": "225 247 247", // #e1f7f7
		"--color-secondary-200": "218 245 245", // #daf5f5
		"--color-secondary-300": "195 238 239", // #c3eeef
		"--color-secondary-400": "150 226 228", // #96e2e4
		"--color-secondary-500": "105 213 216", // #69d5d8
		"--color-secondary-600": "95 192 194", // #5fc0c2
		"--color-secondary-700": "79 160 162", // #4fa0a2
		"--color-secondary-800": "63 128 130", // #3f8082
		"--color-secondary-900": "51 104 106", // #33686a
		// tertiary | #eee872 
		"--color-tertiary-50": "252 252 234", // #fcfcea
		"--color-tertiary-100": "252 250 227", // #fcfae3
		"--color-tertiary-200": "251 249 220", // #fbf9dc
		"--color-tertiary-300": "248 246 199", // #f8f6c7
		"--color-tertiary-400": "243 239 156", // #f3ef9c
		"--color-tertiary-500": "238 232 114", // #eee872
		"--color-tertiary-600": "214 209 103", // #d6d167
		"--color-tertiary-700": "179 174 86", // #b3ae56
		"--color-tertiary-800": "143 139 68", // #8f8b44
		"--color-tertiary-900": "117 114 56", // #757238
		// success | #81e368 
		"--color-success-50": "236 251 232", // #ecfbe8
		"--color-success-100": "230 249 225", // #e6f9e1
		"--color-success-200": "224 248 217", // #e0f8d9
		"--color-success-300": "205 244 195", // #cdf4c3
		"--color-success-400": "167 235 149", // #a7eb95
		"--color-success-500": "129 227 104", // #81e368
		"--color-success-600": "116 204 94", // #74cc5e
		"--color-success-700": "97 170 78", // #61aa4e
		"--color-success-800": "77 136 62", // #4d883e
		"--color-success-900": "63 111 51", // #3f6f33
		// warning | #c40e6f 
		"--color-warning-50": "246 219 233", // #f6dbe9
		"--color-warning-100": "243 207 226", // #f3cfe2
		"--color-warning-200": "240 195 219", // #f0c3db
		"--color-warning-300": "231 159 197", // #e79fc5
		"--color-warning-400": "214 86 154", // #d6569a
		"--color-warning-500": "196 14 111", // #c40e6f
		"--color-warning-600": "176 13 100", // #b00d64
		"--color-warning-700": "147 11 83", // #930b53
		"--color-warning-800": "118 8 67", // #760843
		"--color-warning-900": "96 7 54", // #600736
		// error | #ff3d3d 
		"--color-error-50": "255 226 226", // #ffe2e2
		"--color-error-100": "255 216 216", // #ffd8d8
		"--color-error-200": "255 207 207", // #ffcfcf
		"--color-error-300": "255 177 177", // #ffb1b1
		"--color-error-400": "255 119 119", // #ff7777
		"--color-error-500": "255 61 61", // #ff3d3d
		"--color-error-600": "230 55 55", // #e63737
		"--color-error-700": "191 46 46", // #bf2e2e
		"--color-error-800": "153 37 37", // #992525
		"--color-error-900": "125 30 30", // #7d1e1e
		// surface | #8f8f8f 
		"--color-surface-50": "238 238 238", // #eeeeee
		"--color-surface-100": "233 233 233", // #e9e9e9
		"--color-surface-200": "227 227 227", // #e3e3e3
		"--color-surface-300": "210 210 210", // #d2d2d2
		"--color-surface-400": "177 177 177", // #b1b1b1
		"--color-surface-500": "143 143 143", // #8f8f8f
		"--color-surface-600": "129 129 129", // #818181
		"--color-surface-700": "107 107 107", // #6b6b6b
		"--color-surface-800": "86 86 86", // #565656
		"--color-surface-900": "70 70 70", // #464646

	}
}