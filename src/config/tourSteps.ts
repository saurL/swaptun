export interface TourStep {
  id: string
  title: string
  description: string
  target: string // CSS selector
  placement: 'top' | 'bottom' | 'left' | 'right' | 'center'
  route?: string // Optional: route where this step should be shown
  action?: () => void | Promise<void> // Optional: action to perform before showing
  highlight?: boolean // Whether to highlight the target element
}

export const tourSteps: TourStep[] = [
  {
    id: 'welcome',
    title: 'Welcome to Swaply! 🎵',
    description: 'Let\'s take a quick tour to help you get started with sharing your music with friends.',
    target: 'body',
    placement: 'center',
    route: '/home',
    highlight: false
  },
  {
    id: 'connect-platform',
    title: 'Connect your music platform',
    description: 'Start by connecting your favorite music streaming service. You can connect Spotify, Apple Music, YouTube Music, or Deezer.',
    target: '[data-tour="platform-button"]',
    placement: 'bottom',
    route: '/home',
    highlight: true
  },
  {
    id: 'your-playlists',
    title: 'Your playlists',
    description: 'Once connected, all your playlists will appear here. You can browse and filter them by platform using these chips.',
    target: '[data-tour="home-tab"]',
    placement: 'top',
    route: '/home',
    highlight: true
  },
  {
    id: 'shared-playlists-tab',
    title: 'Shared playlists',
    description: 'Check out playlists that your friends have shared with you here. You\'ll be notified when someone sends you a playlist.',
    target: '[data-tour="shared-tab"]',
    placement: 'top',
    route: '/home',
    highlight: true
  },
  {
    id: 'friends-tab',
    title: 'Add friends',
    description: 'Add friends to start sharing your favorite playlists. Search for friends by username and connect with them.',
    target: '[data-tour="friends-tab"]',
    placement: 'top',
    route: '/home',
    highlight: true
  },
  {
    id: 'profile-tab',
    title: 'Your profile & settings',
    description: 'Manage your connected platforms, view your account settings, and customize your experience here.',
    target: '[data-tour="profile-tab"]',
    placement: 'top',
    route: '/home',
    highlight: true
  },
  {
    id: 'notifications',
    title: 'Stay notified 🔔',
    description: 'Enable notifications to be instantly alerted when friends share playlists with you. You\'ll never miss a great music recommendation!',
    target: 'body',
    placement: 'center',
    route: '/home',
    highlight: false
  },
  {
    id: 'complete',
    title: 'You\'re all set! 🎉',
    description: 'Now you know your way around! Start connecting platforms, adding friends, and sharing your music.',
    target: 'body',
    placement: 'center',
    route: '/home',
    highlight: false
  }
]

export const TOUR_VERSION = '1.2'
