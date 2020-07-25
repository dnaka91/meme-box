import * as fs from 'fs';
import {Clip, Config, Dictionary, HasId, Screen, ScreenClip, State, Twitch} from "../projects/contracts/src/lib/types";
import {createInitialState} from "../projects/contracts/src/lib/createInitialState";
import {Observable, Subject} from "rxjs";
import * as path from "path";


function uuidv4() {
  return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function (c) {
    var r = Math.random() * 16 | 0, v = c == 'x' ? r : (r & 0x3 | 0x8);
    return v.toString(16);
  });
}

const initialObsUrlObj: Screen = Object.freeze({
  id: '',
  name: '',
  clips: {}
});

export class Persistence {

  private updated$ = new Subject();
  private data: State = Object.assign({}, createInitialState());

  constructor(private filePath: string) {
    const dir = path.dirname(filePath);
    if (!fs.existsSync(dir)){
      fs.mkdirSync(dir);
    }

    // Read the file
    fs.readFile(filePath, {
      encoding: 'utf8',
      flag: 'a+' // open or create if not exist
    }, (err, data) => {
      if (err) {
        return console.log(err);
      }
      console.info('SETTING DATA', data);

      let dataFromFile = {};

      if (data && data.includes('{')) {
        dataFromFile = JSON.parse(data);
      }
// execute upgrade , changing names or other stuff

      this.data = Object.assign({}, createInitialState(), dataFromFile);

      console.info({data: this.data});
    });
  }

  public dataUpdated$ () : Observable<any> {
    return this.updated$.asObservable();
  }

  public fullState() {
    return  this.data;
  }

  // save it on changes
  // return sub data off it

  /*
   *  Clips Persistence
   */

  public addClip(clip: Clip) {
    console.info({clip});

    clip.id = uuidv4();
    this.data.clips[clip.id] = clip;

    console.info(this.data.clips);

    this.saveData();
    return clip.id;
  }

  public updateClip(id: string, clip: Clip) {
    console.info({clip});

    clip.id = id;
    this.updateItemInDictionary(this.data.clips, clip);

    this.saveData();
    return clip;
  }

  public deleteClip(id: string) {
    this.deleteItemInDictionary(this.data.clips, id);

    this.saveData();
  }

  public listClips(): Clip[] {
    return Object.values(this.data.clips);
  }

  /*
   *  OBS URLs Settings
   */

  public listScreens() {
    return Object.values(this.data.screen);
  }

  public addScreen(obsUrl: Screen) {
    console.info({obsUrl});

    obsUrl.id = uuidv4();
    this.data.screen[obsUrl.id] = Object.assign({}, initialObsUrlObj, obsUrl);

    this.saveData();
    return obsUrl.id;
  }

  public updateScreen(id: string, obsUrl: Screen) {
    console.info({obsUrl});

    obsUrl.id = id;

    this.updateItemInDictionary(this.data.screen, obsUrl);

    this.saveData();
    return obsUrl;
  }

  public deleteScreen(id: string) {
    console.info({ prevObsUrl: this.data.screen, id});

    this.deleteItemInDictionary(this.data.screen, id);

    console.info({ postObsUrl: this.data.screen, id});

    this.saveData();
  }


  /*
   *  OBS URLs Settings
   */

  public addObsClip(targetUrlId: string, obsClip: ScreenClip) {

    obsClip.id = uuidv4();
    this.data.screen[targetUrlId].clips[obsClip.id] = obsClip;

    this.saveData();
    return obsClip.id;
  }

  public updateObsClip(targetUrlId: string, id: string, obsClip: ScreenClip) {
    console.info({obsUrl: obsClip});

    obsClip.id = id;

    this.updateItemInDictionary(this.data.screen[targetUrlId].clips, obsClip);

    this.saveData();
    return obsClip;
  }

  public deleteObsClip(targetUrlId: string, id: string) {
    this.deleteItemInDictionary(this.data.screen[targetUrlId].clips, id);

    this.saveData();
  }

  public listObsUrls(): Screen[] {
    return Object.values(this.data.screen);
  }


  /*
   *  Twitch Event Settings
   */

  public addTwitchEvent(twitchEvent: Twitch) {
    twitchEvent.id = uuidv4();
    this.data.twitchEvents[twitchEvent.id] = twitchEvent;

    this.saveData();
    return twitchEvent.id;
  }

  public updateTwitchEvent(id: string, twitchEvent: Twitch) {
    twitchEvent.id = id;

    this.updateItemInDictionary(this.data.twitchEvents, twitchEvent);

    this.saveData();
    return twitchEvent;
  }

  public deleteTwitchEvent(id: string) {
    this.deleteItemInDictionary(this.data.twitchEvents, id);

    this.saveData();
  }

  public listTwitchEvents(): Twitch[] {
    return Object.values(this.data.twitchEvents);
  }

  /*
   *  General Settings
   */

  // TODO maybe key/value safety / validations
  public updateConfig(config: Config) {
    this.data.config = config;

    this.saveData();
  }

  public getConfig() {
    return this.data.config;
  }

  /*
  *  Helpers
  */

  private updateItemInDictionary<T extends HasId>(dict: Dictionary<T>, item: T) {
    dict[item.id] = item;
  }

  private deleteItemInDictionary<T extends HasId>(dict: Dictionary<T>, id: string) {
    delete dict[id];
  }

  private saveData() {
    fs.writeFile(this.filePath, JSON.stringify(this.data, null, '  '), err => {
      console.error(err);
    });
    this.updated$.next();
  }
}


export const PersistenceInstance = new Persistence('./settings/settings.json');
