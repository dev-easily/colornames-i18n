/* eslint-disable global-require */

import {translate} from '@docusaurus/Translate';
import {sortBy} from '@site/src/utils/jsUtils';

export type TagType =
  | 'favorite'
  | 'private'
  | 'china'

const Sites: Site[] = [
  {
    title: 'Coder',
    description: '一键创建云开发环境',
    preview: require('./navigation/coder.png'),
    website: 'https://agile-ts.org/',
    tags: ['favorite', 'private'],
  },
  {
    title: 'Code Server',
    description: '自用在线代码编辑器',
    preview: require('./navigation/cs.png'),
    website: 'https://cs.eusoftbank.com',
    tags: ['favorite'],
  },
  {
    title: "交流电社区",
    description: '好玩的私人论坛',
    preview: require('./navigation/eubbs.png'),
    website: 'https://bbs.eusoftbank.com',
    tags: ['favorite', 'china'],
  },
];

export type Site = {
  title: string;
  description: string;
  preview: string | null; // null = use our serverless screenshot service
  website: string;
  source?: string;
  tags: TagType[];
};

export type Tag = {
  label: string;
  description: string;
  color: string;
};

export const Tags: {[type in TagType]: Tag} = {
  favorite: {
    label: translate({message: '最爱'}),
    description: translate({
      message:
        '最受喜爱的网站',
      id: 'navigation.tag.favorite.description',
    }),
    color: '#e9669e',
  },

  private: {
    label: translate({message: '私人'}),
    description: translate({
      message: '私人网站',
      id: 'navigation.tag.private.description',
    }),
    color: '#39ca30',
  },

  china: {
    label: translate({message: '你懂的'}),
    description: translate({
      message: '专治网络超时，下载失败',
      id: 'navigation.tag.china.description',
    }),
    color: '#dfd545',
  },
};

export const TagList = Object.keys(Tags) as TagType[];
function sortSites() {
  let result = Sites;
  // Sort by site name
  result = sortBy(result, (user) => user.title.toLowerCase());
  // Sort by favorite tag, favorites first
  result = sortBy(result, (user) => !user.tags.includes('favorite'));
  return result;
}

export const sortedSites = sortSites();
